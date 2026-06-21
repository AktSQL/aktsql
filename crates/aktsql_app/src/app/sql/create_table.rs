use super::*;

pub(in crate::app) fn create_table_statement(
    form: &mut ConnectionForm,
    draft: &CreateTableDraft,
) -> Result<String, String> {
    let table = draft.name().trim();
    if table.is_empty() {
        return Err(String::from("Table name is required."));
    }
    let columns = create_table_columns(form.driver, draft)?;
    let constraints = create_table_constraints(form.driver, draft);

    let sql = match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before creating a table."));
            }
            let engine = if draft.engine().trim().is_empty() {
                "InnoDB"
            } else {
                draft.engine().trim()
            };
            let mut statements = vec![format!(
                "CREATE TABLE {}.{} (\n    {}\n) ENGINE={} DEFAULT CHARSET={} COLLATE={}{};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                join_table_parts(columns, constraints),
                engine,
                mysql_charset_or_default(draft.charset()),
                mysql_collation_or_default(draft.collation()),
                table_comment_clause(form.driver, draft.comment())
            )];
            statements.extend(create_index_statements(form.driver, database, table, draft));
            statements.join("\n")
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            let mut statements = vec![format!(
                "CREATE TABLE {} (\n    {}\n);",
                quote_dotted_identifier(table),
                join_table_parts(columns, constraints)
            )];
            statements.extend(create_index_statements(form.driver, "", table, draft));
            statements.join("\n")
        }
        DatabaseDriver::MongoDb => {
            form.database = database_name_for_create_collection(form)?;
            mongodb_create_collection_commands(table, draft)
        }
        DatabaseDriver::Sqlite => {
            let mut statements = vec![format!(
                "CREATE TABLE {} (\n    {}{}\n);",
                quote_plain_identifier(table),
                join_table_parts(columns, constraints),
                table_comment_clause(form.driver, draft.comment())
            )];
            statements.extend(create_index_statements(form.driver, "", table, draft));
            statements.join("\n")
        }
        DatabaseDriver::SqlServer => {
            return Err(String::from(
                "SQL Server create table needs a native driver before direct execution is available.",
            ));
        }
        DatabaseDriver::Oracle => {
            return Err(String::from(
                "Oracle create table needs a native driver before direct execution is available.",
            ));
        }
    };

    Ok(sql)
}

pub(super) fn create_table_columns(
    driver: DatabaseDriver,
    draft: &CreateTableDraft,
) -> Result<Vec<String>, String> {
    if driver == DatabaseDriver::MongoDb {
        return Ok(Vec::new());
    }

    let columns = draft
        .columns()
        .iter()
        .filter(|column| !column.name().trim().is_empty() || !column.data_type().trim().is_empty())
        .map(|column| create_table_column(driver, column))
        .collect::<Result<Vec<_>, _>>()?;

    if columns.is_empty() {
        Err(String::from("At least one column is required."))
    } else {
        Ok(columns)
    }
}

pub(super) fn create_table_column(
    driver: DatabaseDriver,
    column: &CreateTableColumnDraft,
) -> Result<String, String> {
    let name = column.name().trim();
    let data_type = column.data_type().trim();
    if name.is_empty() || data_type.is_empty() {
        return Err(String::from("Column name and data type are required."));
    }

    let quoted_name = match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            quote_mysql_identifier(name)
        }
        DatabaseDriver::SqlServer => quote_sql_server_identifier(name),
        _ => quote_plain_identifier(name),
    };
    let nullable = if column.nullable().trim().eq_ignore_ascii_case("YES") {
        ""
    } else {
        " NOT NULL"
    };
    let default_value = column.default_value().trim();
    let default_clause = if default_value.is_empty() {
        String::new()
    } else {
        format!(" DEFAULT {default_value}")
    };
    let extra = column.extra().trim();
    let extra_clause = if extra.is_empty() {
        String::new()
    } else {
        format!(" {extra}")
    };

    Ok(format!(
        "{quoted_name} {data_type}{nullable}{default_clause}{extra_clause}"
    ))
}

pub(super) fn create_table_constraints(
    driver: DatabaseDriver,
    draft: &CreateTableDraft,
) -> Vec<String> {
    draft
        .constraints()
        .iter()
        .filter_map(|constraint| create_table_constraint(driver, constraint))
        .collect()
}

pub(super) fn create_table_constraint(
    driver: DatabaseDriver,
    constraint: &CreateTableConstraintDraft,
) -> Option<String> {
    let kind = constraint.kind().trim().to_uppercase();
    let expression = constraint.expression().trim();
    if kind.is_empty() || expression.is_empty() {
        return None;
    }
    let name = constraint.name().trim();
    let name_clause = if name.is_empty() {
        String::new()
    } else {
        let quoted = match driver {
            DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
                quote_mysql_identifier(name)
            }
            DatabaseDriver::SqlServer => quote_sql_server_identifier(name),
            _ => quote_plain_identifier(name),
        };
        format!("CONSTRAINT {quoted} ")
    };

    let clause = match kind.as_str() {
        "PRIMARY KEY" | "UNIQUE" => format!("{kind} ({expression})"),
        "CHECK" => format!("CHECK ({expression})"),
        "FOREIGN KEY" => format!("FOREIGN KEY {expression}"),
        _ => format!("{kind} {expression}"),
    };

    Some(format!("{name_clause}{clause}"))
}

pub(super) fn join_table_parts(columns: Vec<String>, constraints: Vec<String>) -> String {
    columns
        .into_iter()
        .chain(constraints)
        .collect::<Vec<_>>()
        .join(",\n    ")
}

pub(super) fn create_index_statements(
    driver: DatabaseDriver,
    database: &str,
    table: &str,
    draft: &CreateTableDraft,
) -> Vec<String> {
    draft
        .indexes()
        .iter()
        .filter_map(|index| create_index_statement(driver, database, table, index, draft.columns()))
        .collect()
}

pub(super) fn create_index_statement(
    driver: DatabaseDriver,
    database: &str,
    table: &str,
    index: &CreateTableIndexDraft,
    table_columns: &[CreateTableColumnDraft],
) -> Option<String> {
    let name = index.name().trim();
    let columns = index.columns().trim();
    if name.is_empty() || columns.is_empty() {
        return None;
    }
    let index_type = crate::schema::normalized_index_type(driver, index.index_type());

    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let (prefix, using) = crate::schema::mysql_index_type_sql(&index_type);
            let columns = mysql_index_columns_sql(
                columns,
                table_columns
                    .iter()
                    .map(|column| (column.name(), column.data_type()))
                    .collect(),
                &index_type,
            );
            Some(format!(
                "CREATE {prefix}INDEX {}{using} ON {}.{} ({});",
                quote_mysql_identifier(name),
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                columns
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            let (unique, using) = crate::schema::postgres_index_type_sql(&index_type);
            Some(format!(
                "CREATE {unique}INDEX {} ON {} USING {using} ({});",
                quote_plain_identifier(name),
                quote_dotted_identifier(table),
                columns
            ))
        }
        DatabaseDriver::Sqlite => Some(format!(
            "CREATE {}INDEX {} ON {} ({});",
            crate::schema::sqlite_index_type_sql(&index_type),
            quote_plain_identifier(name),
            quote_plain_identifier(table),
            columns
        )),
        DatabaseDriver::SqlServer => Some(format!(
            "CREATE {} INDEX {} ON {} ({});",
            crate::schema::sql_server_index_type_sql(&index_type),
            quote_sql_server_identifier(name),
            quote_sql_server_table(table),
            columns
        )),
        DatabaseDriver::Oracle => Some(format!(
            "CREATE {}INDEX {} ON {} ({});",
            crate::schema::oracle_index_type_sql(&index_type),
            quote_plain_identifier(name),
            quote_plain_identifier(table),
            columns
        )),
        DatabaseDriver::MongoDb => None,
    }
}

pub(super) fn mongodb_create_collection_commands(table: &str, draft: &CreateTableDraft) -> String {
    let mut commands = vec![format!("{{\"create\":\"{}\"}}", json_escape(table))];
    let indexes = draft
        .indexes()
        .iter()
        .filter(|index| !index.name().trim().is_empty() && !index.columns().trim().is_empty())
        .map(|index| {
            let index_type =
                crate::schema::normalized_index_type(DatabaseDriver::MongoDb, index.index_type());
            let keys = crate::schema::mongodb_index_keys(index.columns(), &index_type, json_escape);
            format!(
                "{{\"key\":{{{keys}}},\"name\":\"{}\"}}",
                json_escape(index.name())
            )
        })
        .collect::<Vec<_>>();

    if !indexes.is_empty() {
        commands.push(format!(
            "{{\"createIndexes\":\"{}\",\"indexes\":[{}]}}",
            json_escape(table),
            indexes.join(",")
        ));
    }

    if commands.len() == 1 {
        commands.remove(0)
    } else {
        format!("[{}]", commands.join(","))
    }
}

pub(super) fn table_comment_clause(driver: DatabaseDriver, comment: &str) -> String {
    let comment = comment.trim();
    if comment.is_empty() {
        return String::new();
    }

    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            format!(" COMMENT={}", sql_string_literal(comment))
        }
        _ => String::new(),
    }
}

pub(super) fn database_name_for_create_collection(form: &ConnectionForm) -> Result<String, String> {
    let database = form.database.trim();
    if database.is_empty() {
        Err(String::from(
            "Select a MongoDB database before creating a collection.",
        ))
    } else {
        Ok(database.to_owned())
    }
}
