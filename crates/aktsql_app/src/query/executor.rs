use super::*;
use std::time::Instant;

pub(super) fn execute_query(form: &ConnectionForm, sql: &str) -> Result<QueryResult, Vec<String>> {
    let sql = sql.trim();
    if sql.is_empty() {
        return Err(vec![String::from("SQL is required.")]);
    }

    form.validate()?;

    let result = match form.driver {
        DatabaseDriver::Sqlite => execute_sqlite(form, sql),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            execute_mysql(form, sql)
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => execute_postgres(form, sql),
        DatabaseDriver::MongoDb => execute_mongodb(form, sql),
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => Err(format!(
            "{} query execution needs a native driver dependency before it can run live queries.",
            form.driver
        )),
    };

    result.map_err(|error| vec![error])
}

fn execute_sqlite(form: &ConnectionForm, sql: &str) -> Result<QueryResult, String> {
    let start = Instant::now();
    let connection = rusqlite::Connection::open(form.location.trim())
        .map_err(|error| format!("SQLite connection failed: {error}"))?;
    let statements = split_sql_statements(sql);
    if statements.len() > 1 {
        connection
            .execute_batch(sql)
            .map_err(|error| format!("SQLite batch execution failed: {error}"))?;

        return Ok(statement_result("SQLite", 0, start.elapsed().as_millis()));
    }

    let mut statement = connection
        .prepare(sql)
        .map_err(|error| format!("SQLite prepare failed: {error}"))?;

    let column_count = statement.column_count();
    if column_count == 0 {
        let rows_affected = statement
            .execute([])
            .map_err(|error| format!("SQLite execution failed: {error}"))?;
        let elapsed_ms = start.elapsed().as_millis();

        return Ok(QueryResult {
            columns: Vec::new(),
            rows: Vec::new(),
            rows_affected: Some(rows_affected),
            elapsed_ms,
            message: format!("SQLite statement executed. {rows_affected} row(s) affected."),
            truncated: false,
        });
    }

    let columns = statement
        .column_names()
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let mut row_cursor = statement
        .query([])
        .map_err(|error| format!("SQLite query failed: {error}"))?;
    let mut rows = Vec::new();
    let mut truncated = false;

    while let Some(row) = row_cursor
        .next()
        .map_err(|error| format!("SQLite row read failed: {error}"))?
    {
        if rows.len() == MAX_RESULT_ROWS {
            truncated = true;
            break;
        }

        let values = (0..column_count)
            .map(|index| value_to_string(row.get_ref(index)))
            .collect::<Result<Vec<_>, _>>()?;
        rows.push(values);
    }

    let elapsed_ms = start.elapsed().as_millis();
    let message = if truncated {
        format!(
            "SQLite query returned {} row(s) in {elapsed_ms} ms; display capped at {MAX_RESULT_ROWS}.",
            rows.len()
        )
    } else {
        format!(
            "SQLite query returned {} row(s) in {elapsed_ms} ms.",
            rows.len()
        )
    };

    Ok(QueryResult {
        columns,
        rows,
        rows_affected: None,
        elapsed_ms,
        message,
        truncated,
    })
}

fn execute_mysql(form: &ConnectionForm, sql: &str) -> Result<QueryResult, String> {
    let start = Instant::now();
    let opts = mysql::Opts::from_url(&mysql_url(form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let mut connection = pool
        .get_conn()
        .map_err(|error| format!("MySQL connection failed: {error}"))?;
    let statements = split_sql_statements(sql);
    if statements.len() > 1 {
        let mut affected_rows = 0usize;
        for statement in statements {
            connection
                .query_drop(statement.as_str())
                .map_err(|error| format!("MySQL statement failed: {error}"))?;
            affected_rows = affected_rows.saturating_add(connection.affected_rows() as usize);
        }

        return Ok(statement_result(
            "MySQL",
            affected_rows,
            start.elapsed().as_millis(),
        ));
    }

    let mut query_result = connection
        .query_iter(sql)
        .map_err(|error| format!("MySQL query failed: {error}"))?;
    let Some(mut result_set) = query_result.iter() else {
        return Ok(statement_result("MySQL", 0, start.elapsed().as_millis()));
    };
    let columns = result_set
        .columns()
        .as_ref()
        .iter()
        .map(|column| column.name_str().into_owned())
        .collect::<Vec<_>>();

    if columns.is_empty() {
        let affected_rows = result_set.affected_rows() as usize;
        return Ok(statement_result(
            "MySQL",
            affected_rows,
            start.elapsed().as_millis(),
        ));
    }

    let mut rows = Vec::new();
    let mut truncated = false;

    for row in &mut result_set {
        if rows.len() == MAX_RESULT_ROWS {
            truncated = true;
            break;
        }

        let row = row.map_err(|error| format!("MySQL row read failed: {error}"))?;
        let values = (0..row.len())
            .map(|index| {
                row.as_ref(index)
                    .map(mysql_value_to_string)
                    .unwrap_or_else(|| String::from("NULL"))
            })
            .collect::<Vec<_>>();
        rows.push(values);
    }

    Ok(tabular_result(
        "MySQL",
        columns,
        rows,
        truncated,
        start.elapsed().as_millis(),
    ))
}

fn execute_postgres(form: &ConnectionForm, sql: &str) -> Result<QueryResult, String> {
    let start = Instant::now();
    let config = postgres_config(form)?;
    let mut client = config
        .connect(postgres::NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let messages = client
        .simple_query(sql)
        .map_err(|error| format!("PostgreSQL query failed: {error}"))?;
    let mut columns = Vec::new();
    let mut rows = Vec::new();
    let mut affected_rows = None;
    let mut truncated = false;

    for message in messages {
        match message {
            postgres::SimpleQueryMessage::RowDescription(description) if columns.is_empty() => {
                columns = description
                    .iter()
                    .map(|column| column.name().to_owned())
                    .collect();
            }
            postgres::SimpleQueryMessage::Row(row) => {
                if rows.len() == MAX_RESULT_ROWS {
                    truncated = true;
                    continue;
                }

                if columns.is_empty() {
                    columns = row
                        .columns()
                        .iter()
                        .map(|column| column.name().to_owned())
                        .collect();
                }

                rows.push(
                    (0..row.len())
                        .map(|index| row.get(index).unwrap_or("NULL").to_owned())
                        .collect(),
                );
            }
            postgres::SimpleQueryMessage::CommandComplete(count) => {
                affected_rows = Some(count as usize);
            }
            _ => {}
        }
    }

    let elapsed_ms = start.elapsed().as_millis();
    if columns.is_empty() {
        return Ok(statement_result(
            "PostgreSQL",
            affected_rows.unwrap_or(0),
            elapsed_ms,
        ));
    }

    Ok(tabular_result(
        "PostgreSQL",
        columns,
        rows,
        truncated,
        elapsed_ms,
    ))
}

fn execute_mongodb(form: &ConnectionForm, command: &str) -> Result<QueryResult, String> {
    let start = Instant::now();
    let commands = parse_mongodb_commands(command)?;
    let options = mongodb::options::ClientOptions::parse(mongodb_url(form))
        .map_err(|error| format!("MongoDB connection options failed: {error}"))?;
    let client = MongoClient::with_options(options)
        .map_err(|error| format!("MongoDB client creation failed: {error}"))?;
    let database = client.database(database_name(form).as_str());
    let mut response = Document::new();
    let command_count = commands.len();

    for command in commands {
        response = database
            .run_command(command, None)
            .map_err(|error| format!("MongoDB command failed: {error}"))?;
    }

    let elapsed_ms = start.elapsed().as_millis();
    if command_count > 1 {
        return Ok(statement_result("MongoDB", command_count, elapsed_ms));
    }

    Ok(mongodb_document_result(response, elapsed_ms))
}
