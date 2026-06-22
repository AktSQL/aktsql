use crate::*;

pub fn ordered_query_sql(
    driver: DatabaseDriver,
    sql: &str,
    order_by: &[ResultSortKey],
) -> Result<String, String> {
    if order_by.is_empty() {
        return Ok(sql.to_owned());
    }

    if driver == DatabaseDriver::MongoDb {
        return Err(String::from(
            "MongoDB ad-hoc result sorting is only available from collection row browsing.",
        ));
    }

    let base_sql = trimmed_select_sql(sql)?;
    let order_clause = order_by_clause(driver, order_by);
    let alias = if driver == DatabaseDriver::Oracle {
        "aktsql_result"
    } else {
        "AS aktsql_result"
    };

    Ok(format!(
        "SELECT * FROM ({base_sql}) {alias} ORDER BY {order_clause};"
    ))
}

pub(super) fn trimmed_select_sql(sql: &str) -> Result<String, String> {
    let mut sql = sql.trim().to_owned();
    while sql.ends_with(';') {
        sql.pop();
        sql = sql.trim_end().to_owned();
    }

    if sql.is_empty() {
        return Err(String::from("SQL is required."));
    }

    if !sql.trim_start().to_ascii_lowercase().starts_with("select") {
        return Err(String::from(
            "Result sorting requires a SELECT query result set.",
        ));
    }

    Ok(sql)
}

pub(super) fn order_by_clause(driver: DatabaseDriver, order_by: &[ResultSortKey]) -> String {
    order_by
        .iter()
        .map(|key| {
            format!(
                "{} {}",
                quote_result_identifier(driver, &key.column_name),
                key.direction.sql()
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn quote_result_identifier(driver: DatabaseDriver, value: &str) -> String {
    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            quote_mysql_identifier(value)
        }
        DatabaseDriver::SqlServer => quote_sql_server_identifier(value),
        _ => quote_plain_identifier(value),
    }
}

pub fn select_rows_statement(
    form: &mut ConnectionForm,
    table: &str,
    page: usize,
    page_size: usize,
    order_by: &[ResultSortKey],
) -> Result<String, String> {
    let offset = page.saturating_mul(page_size);
    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from(
                    "Select a database before browsing table rows.",
                ));
            }
            let order_clause = table_order_clause(form.driver, order_by);
            Ok(format!(
                "SELECT * FROM {}.{}{} LIMIT {} OFFSET {};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                order_clause,
                page_size,
                offset
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            let order_clause = table_order_clause(form.driver, order_by);
            Ok(format!(
                "SELECT * FROM {}{} LIMIT {} OFFSET {};",
                quote_dotted_identifier(table),
                order_clause,
                page_size,
                offset
            ))
        }
        DatabaseDriver::Sqlite => {
            let order_clause = table_order_clause(form.driver, order_by);
            Ok(format!(
                "SELECT * FROM {}{} LIMIT {} OFFSET {};",
                quote_plain_identifier(table),
                order_clause,
                page_size,
                offset
            ))
        }
        DatabaseDriver::MongoDb => Ok(format!(
            "{{\"find\":\"{}\",\"filter\":{{}}{},\"limit\":{},\"skip\":{}}}",
            json_escape(table),
            mongodb_sort_clause(order_by),
            page_size,
            offset
        )),
        DatabaseDriver::SqlServer => Ok(format!(
            "SELECT * FROM {}{} OFFSET {} ROWS FETCH NEXT {} ROWS ONLY;",
            quote_sql_server_table(table),
            sql_server_order_clause(order_by),
            offset,
            page_size
        )),
        DatabaseDriver::Oracle => Ok(format!(
            "SELECT * FROM {}{} OFFSET {} ROWS FETCH NEXT {} ROWS ONLY",
            quote_plain_identifier(table),
            table_order_clause(form.driver, order_by),
            offset,
            page_size
        )),
    }
}

pub(super) fn table_order_clause(driver: DatabaseDriver, order_by: &[ResultSortKey]) -> String {
    if order_by.is_empty() {
        String::new()
    } else {
        format!(" ORDER BY {}", order_by_clause(driver, order_by))
    }
}

pub(super) fn sql_server_order_clause(order_by: &[ResultSortKey]) -> String {
    if order_by.is_empty() {
        String::from(" ORDER BY (SELECT NULL)")
    } else {
        format!(
            " ORDER BY {}",
            order_by_clause(DatabaseDriver::SqlServer, order_by)
        )
    }
}

pub(super) fn mongodb_sort_clause(order_by: &[ResultSortKey]) -> String {
    if order_by.is_empty() {
        return String::new();
    }

    let sort = order_by
        .iter()
        .map(|key| {
            let direction = match key.direction {
                SortDirection::Asc => 1,
                SortDirection::Desc => -1,
            };
            format!("\"{}\":{}", json_escape(&key.column_name), direction)
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(",\"sort\":{{{sort}}}")
}
