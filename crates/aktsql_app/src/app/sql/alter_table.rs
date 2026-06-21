use super::*;

pub(in crate::app) fn alter_table_statement(
    form: &mut ConnectionForm,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let table = draft.table().trim();
    if table.is_empty() {
        return Err(String::from("Table name is required."));
    }

    match draft.operation() {
        AlterTableOperation::RenameColumn => rename_column_statement(form, table, draft),
        AlterTableOperation::AddColumn => add_column_statement(form, table, draft),
        AlterTableOperation::AddIndex => add_index_statement(form, table, draft),
        AlterTableOperation::AddConstraint => add_constraint_statement(form, table, draft),
        AlterTableOperation::MoveColumn => move_column_statement(form, table, draft),
    }
}

pub(super) fn rename_column_statement(
    form: &mut ConnectionForm,
    table: &str,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let column = draft.column_name().trim();
    let new_column = draft.new_column_name().trim();
    if column.is_empty() || new_column.is_empty() {
        return Err(String::from("Old and new column names are required."));
    }
    if !column.eq_ignore_ascii_case(new_column)
        && column_name_exists(draft.original_column_names(), new_column)
    {
        return Err(format!("Column already exists: {new_column}."));
    }

    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before altering a table."));
            }
            Ok(format!(
                "ALTER TABLE {}.{} RENAME COLUMN {} TO {};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                quote_mysql_identifier(column),
                quote_mysql_identifier(new_column)
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => Ok(format!(
            "ALTER TABLE {} RENAME COLUMN {} TO {};",
            quote_dotted_identifier(table),
            quote_plain_identifier(column),
            quote_plain_identifier(new_column)
        )),
        DatabaseDriver::Sqlite => Ok(format!(
            "ALTER TABLE {} RENAME COLUMN {} TO {};",
            quote_plain_identifier(table),
            quote_plain_identifier(column),
            quote_plain_identifier(new_column)
        )),
        DatabaseDriver::SqlServer => Ok(format!(
            "EXEC sp_rename {}, {}, 'COLUMN';",
            sql_string_literal(format!("{table}.{column}").as_str()),
            sql_string_literal(new_column)
        )),
        DatabaseDriver::Oracle => Ok(format!(
            "ALTER TABLE {} RENAME COLUMN {} TO {};",
            quote_plain_identifier(table),
            quote_plain_identifier(column),
            quote_plain_identifier(new_column)
        )),
        DatabaseDriver::MongoDb => Err(String::from(
            "MongoDB collection field rename needs an updateMany pipeline and a filter; it is not a table structure operation.",
        )),
    }
}

pub(super) fn add_column_statement(
    form: &mut ConnectionForm,
    table: &str,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let column = draft.column_name().trim();
    if column_name_exists(draft.original_column_names(), column) {
        return Err(format!("Column already exists: {column}."));
    }
    validate_unique_column_names(draft.reordered_columns())?;

    let definition = alter_column_definition(form.driver, draft)?;
    let positioned_rebuild = !draft.reordered_columns().is_empty()
        && draft.reordered_columns().len() > draft.original_column_names().len()
        && draft.column_position() != "LAST";

    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before altering a table."));
            }
            let position = mysql_column_position_clause(draft);
            Ok(format!(
                "ALTER TABLE {}.{} ADD COLUMN {}{};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                definition,
                position
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb if positioned_rebuild => {
            Ok(rebuild_table_with_added_column_statement(
                form.driver,
                table,
                draft.reordered_columns(),
                draft.original_column_names(),
                quote_dotted_identifier,
                quote_plain_identifier,
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => Ok(format!(
            "ALTER TABLE {} ADD COLUMN {};",
            quote_dotted_identifier(table),
            definition
        )),
        DatabaseDriver::Sqlite if positioned_rebuild => {
            Ok(sqlite_rebuild_table_with_added_column_statement(
                table,
                draft.reordered_columns(),
                draft.original_column_names(),
            ))
        }
        DatabaseDriver::Sqlite => Ok(format!(
            "ALTER TABLE {} ADD COLUMN {};",
            quote_plain_identifier(table),
            definition
        )),
        DatabaseDriver::SqlServer if positioned_rebuild => {
            Ok(rebuild_table_with_added_column_statement(
                form.driver,
                table,
                draft.reordered_columns(),
                draft.original_column_names(),
                quote_sql_server_table,
                quote_sql_server_identifier,
            ))
        }
        DatabaseDriver::SqlServer => Ok(format!(
            "ALTER TABLE {} ADD {};",
            quote_sql_server_table(table),
            definition
        )),
        DatabaseDriver::Oracle if positioned_rebuild => {
            Ok(rebuild_table_with_added_column_statement(
                form.driver,
                table,
                draft.reordered_columns(),
                draft.original_column_names(),
                quote_plain_identifier,
                quote_plain_identifier,
            ))
        }
        DatabaseDriver::Oracle => Ok(format!(
            "ALTER TABLE {} ADD ({});",
            quote_plain_identifier(table),
            definition
        )),
        DatabaseDriver::MongoDb => Err(String::from(
            "MongoDB has no relational add-column operation. Add fields with updateMany or schema validation.",
        )),
    }
}

pub(super) fn add_index_statement(
    form: &mut ConnectionForm,
    table: &str,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let index = draft.index_name().trim();
    let columns = draft.index_columns().trim();
    if index.is_empty() || columns.is_empty() {
        return Err(String::from("Index name and index columns are required."));
    }
    let index_type = crate::schema::normalized_index_type(form.driver, draft.index_type());

    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before creating an index."));
            }
            let (prefix, using) = crate::schema::mysql_index_type_sql(&index_type);
            let columns = mysql_index_columns_sql(
                columns,
                draft
                    .reordered_columns()
                    .iter()
                    .map(|column| (column.name.as_str(), column.data_type.as_str()))
                    .collect(),
                &index_type,
            );
            Ok(format!(
                "CREATE {prefix}INDEX {}{using} ON {}.{} ({});",
                quote_mysql_identifier(index),
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                columns
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            let (unique, using) = crate::schema::postgres_index_type_sql(&index_type);
            Ok(format!(
                "CREATE {unique}INDEX {} ON {} USING {using} ({});",
                quote_plain_identifier(index),
                quote_dotted_identifier(table),
                columns
            ))
        }
        DatabaseDriver::Sqlite => Ok(format!(
            "CREATE {}INDEX {} ON {} ({});",
            crate::schema::sqlite_index_type_sql(&index_type),
            quote_plain_identifier(index),
            quote_plain_identifier(table),
            columns
        )),
        DatabaseDriver::SqlServer => Ok(format!(
            "CREATE {} INDEX {} ON {} ({});",
            crate::schema::sql_server_index_type_sql(&index_type),
            quote_sql_server_identifier(index),
            quote_sql_server_table(table),
            columns
        )),
        DatabaseDriver::Oracle => Ok(format!(
            "CREATE {}INDEX {} ON {} ({});",
            crate::schema::oracle_index_type_sql(&index_type),
            quote_plain_identifier(index),
            quote_plain_identifier(table),
            columns
        )),
        DatabaseDriver::MongoDb => Ok(format!(
            "{{\"createIndexes\":\"{}\",\"indexes\":[{{\"key\":{{{}}},\"name\":\"{}\"}}]}}",
            json_escape(table),
            crate::schema::mongodb_index_keys(columns, &index_type, json_escape),
            json_escape(index)
        )),
    }
}

pub(super) fn add_constraint_statement(
    form: &mut ConnectionForm,
    table: &str,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let name = draft.constraint_name().trim();
    let expression = draft.constraint_expression().trim();
    if name.is_empty() || expression.is_empty() {
        return Err(String::from(
            "Constraint name and constraint expression are required.",
        ));
    }

    if matches!(form.driver, DatabaseDriver::Sqlite) {
        return Err(String::from(
            "SQLite cannot add table constraints in-place. Rebuild the table instead.",
        ));
    }

    if matches!(form.driver, DatabaseDriver::MongoDb) {
        return Err(String::from(
            "MongoDB constraints must be modeled with collection validators, not relational table changes.",
        ));
    }

    let kind = crate::schema::normalized_constraint_type(form.driver, draft.constraint_kind());
    let constraint = constraint_clause(&kind, expression)?;

    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from(
                    "Select a database before adding a constraint.",
                ));
            }
            Ok(format!(
                "ALTER TABLE {}.{} ADD CONSTRAINT {} {};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                quote_mysql_identifier(name),
                constraint
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => Ok(format!(
            "ALTER TABLE {} ADD CONSTRAINT {} {};",
            quote_dotted_identifier(table),
            quote_plain_identifier(name),
            constraint
        )),
        DatabaseDriver::SqlServer => Ok(format!(
            "ALTER TABLE {} ADD CONSTRAINT {} {};",
            quote_sql_server_table(table),
            quote_sql_server_identifier(name),
            constraint
        )),
        DatabaseDriver::Oracle => Ok(format!(
            "ALTER TABLE {} ADD CONSTRAINT {} {};",
            quote_plain_identifier(table),
            quote_plain_identifier(name),
            constraint
        )),
        DatabaseDriver::Sqlite | DatabaseDriver::MongoDb => unreachable!(),
    }
}

fn constraint_clause(kind: &str, expression: &str) -> Result<String, String> {
    let expression = expression.trim();
    if expression.is_empty() {
        return Err(String::from("Constraint expression is required."));
    }

    let upper_expression = expression.to_ascii_uppercase();
    Ok(match kind.to_ascii_uppercase().as_str() {
        "PRIMARY KEY" => {
            if upper_expression.starts_with("PRIMARY KEY") {
                expression.to_owned()
            } else {
                format!("PRIMARY KEY ({expression})")
            }
        }
        "UNIQUE" => {
            if upper_expression.starts_with("UNIQUE") {
                expression.to_owned()
            } else {
                format!("UNIQUE ({expression})")
            }
        }
        "CHECK" => {
            if upper_expression.starts_with("CHECK") {
                expression.to_owned()
            } else {
                format!("CHECK ({expression})")
            }
        }
        "FOREIGN KEY" => {
            if upper_expression.starts_with("FOREIGN KEY") {
                expression.to_owned()
            } else {
                format!("FOREIGN KEY ({expression})")
            }
        }
        _ => return Err(format!("Unsupported constraint type: {kind}.")),
    })
}

pub(in crate::app) fn move_column_statement(
    form: &mut ConnectionForm,
    table: &str,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let reordered_columns = draft.reordered_columns();
    let original_column_names = draft.original_column_names();
    if reordered_columns.is_empty() {
        return Err(String::from(
            "Move a column with the arrow controls before applying.",
        ));
    }
    validate_unique_column_names(reordered_columns)?;

    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before moving a column."));
            }
            Ok(mysql_rebuild_table_statement(
                database,
                table,
                reordered_columns,
                original_column_names,
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => Ok(rebuild_table_statement(
            form.driver,
            table,
            reordered_columns,
            original_column_names,
            quote_dotted_identifier,
            quote_plain_identifier,
        )),
        DatabaseDriver::Sqlite => Ok(sqlite_rebuild_table_statement(
            table,
            reordered_columns,
            original_column_names,
        )),
        DatabaseDriver::SqlServer => Ok(rebuild_table_statement(
            form.driver,
            table,
            reordered_columns,
            original_column_names,
            quote_sql_server_table,
            quote_sql_server_identifier,
        )),
        DatabaseDriver::Oracle => Ok(rebuild_table_statement(
            form.driver,
            table,
            reordered_columns,
            original_column_names,
            quote_plain_identifier,
            quote_plain_identifier,
        )),
        DatabaseDriver::MongoDb => Ok(mongodb_reorder_fields_command(table, reordered_columns)),
    }
}

fn validate_unique_column_names(columns: &[TableColumnDetail]) -> Result<(), String> {
    let mut names = BTreeSet::new();
    for column in columns {
        let name = column.name.trim();
        if name.is_empty() {
            return Err(String::from("Column name is required."));
        }
        if !names.insert(name.to_ascii_lowercase()) {
            return Err(format!("Duplicate column name: {name}."));
        }
    }

    Ok(())
}

fn column_name_exists(existing: &[String], column: &str) -> bool {
    let column = column.trim();
    !column.is_empty()
        && existing
            .iter()
            .any(|name| name.trim().eq_ignore_ascii_case(column))
}

pub(super) fn rebuild_table_statement(
    driver: DatabaseDriver,
    table: &str,
    columns: &[TableColumnDetail],
    source_names: &[String],
    quote_table: fn(&str) -> String,
    quote_column: fn(&str) -> String,
) -> String {
    let temp_table = temporary_table_name(table);
    let column_definitions = columns
        .iter()
        .map(|column| reordered_column_definition(driver, column, quote_column))
        .collect::<Vec<_>>()
        .join(", ");
    let column_list = columns
        .iter()
        .map(|column| quote_column(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let source_column_list = source_column_list(source_names, columns, quote_column);
    let rename_statement = if driver == DatabaseDriver::SqlServer {
        format!(
            "EXEC sp_rename {}, {};",
            sql_string_literal(&temp_table),
            sql_string_literal(last_table_part(table))
        )
    } else {
        format!(
            "ALTER TABLE {} RENAME TO {};",
            quote_table(&temp_table),
            quote_plain_identifier(last_table_part(table))
        )
    };

    format!(
        "CREATE TABLE {} ({}); INSERT INTO {} ({}) SELECT {} FROM {}; DROP TABLE {}; {}",
        quote_table(&temp_table),
        column_definitions,
        quote_table(&temp_table),
        column_list,
        source_column_list,
        quote_table(table),
        quote_table(table),
        rename_statement
    )
}

pub(super) fn rebuild_table_with_added_column_statement(
    driver: DatabaseDriver,
    table: &str,
    columns: &[TableColumnDetail],
    original_column_names: &[String],
    quote_table: fn(&str) -> String,
    quote_column: fn(&str) -> String,
) -> String {
    let temp_table = temporary_table_name(table);
    let column_definitions = columns
        .iter()
        .map(|column| reordered_column_definition(driver, column, quote_column))
        .collect::<Vec<_>>()
        .join(", ");
    let column_list = columns
        .iter()
        .map(|column| quote_column(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let source_column_list =
        add_column_source_column_list(original_column_names, columns, quote_column);
    let rename_statement = if driver == DatabaseDriver::SqlServer {
        format!(
            "EXEC sp_rename {}, {};",
            sql_string_literal(&temp_table),
            sql_string_literal(last_table_part(table))
        )
    } else {
        format!(
            "ALTER TABLE {} RENAME TO {};",
            quote_table(&temp_table),
            quote_plain_identifier(last_table_part(table))
        )
    };

    format!(
        "CREATE TABLE {} ({}); INSERT INTO {} ({}) SELECT {} FROM {}; DROP TABLE {}; {}",
        quote_table(&temp_table),
        column_definitions,
        quote_table(&temp_table),
        column_list,
        source_column_list,
        quote_table(table),
        quote_table(table),
        rename_statement
    )
}

pub(super) fn mysql_rebuild_table_statement(
    database: &str,
    table: &str,
    columns: &[TableColumnDetail],
    source_names: &[String],
) -> String {
    let temp_table = format!("{table}__aktsql_reorder");
    let backup_table = format!("{table}__aktsql_backup");
    let column_definitions = columns
        .iter()
        .map(|column| {
            reordered_column_definition(DatabaseDriver::MySql, column, quote_mysql_identifier)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let column_list = columns
        .iter()
        .map(|column| quote_mysql_identifier(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let source_column_list = source_column_list(source_names, columns, quote_mysql_identifier);

    format!(
        "CREATE TABLE {}.{} ({}); INSERT INTO {}.{} ({}) SELECT {} FROM {}.{}; RENAME TABLE {}.{} TO {}.{}, {}.{} TO {}.{}; DROP TABLE {}.{};",
        quote_mysql_identifier(database),
        quote_mysql_identifier(&temp_table),
        column_definitions,
        quote_mysql_identifier(database),
        quote_mysql_identifier(&temp_table),
        column_list,
        source_column_list,
        quote_mysql_identifier(database),
        quote_mysql_identifier(table),
        quote_mysql_identifier(database),
        quote_mysql_identifier(table),
        quote_mysql_identifier(database),
        quote_mysql_identifier(&backup_table),
        quote_mysql_identifier(database),
        quote_mysql_identifier(&temp_table),
        quote_mysql_identifier(database),
        quote_mysql_identifier(table),
        quote_mysql_identifier(database),
        quote_mysql_identifier(&backup_table)
    )
}

pub(super) fn sqlite_rebuild_table_with_added_column_statement(
    table: &str,
    columns: &[TableColumnDetail],
    original_column_names: &[String],
) -> String {
    let temp_table = temporary_table_name(table);
    let column_definitions = columns
        .iter()
        .map(|column| {
            reordered_column_definition(DatabaseDriver::Sqlite, column, quote_plain_identifier)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let column_list = columns
        .iter()
        .map(|column| quote_plain_identifier(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let source_column_list =
        add_column_source_column_list(original_column_names, columns, quote_plain_identifier);

    format!(
        "PRAGMA foreign_keys=OFF; BEGIN TRANSACTION; CREATE TABLE {} ({}); INSERT INTO {} ({}) SELECT {} FROM {}; DROP TABLE {}; ALTER TABLE {} RENAME TO {}; COMMIT; PRAGMA foreign_keys=ON;",
        quote_plain_identifier(&temp_table),
        column_definitions,
        quote_plain_identifier(&temp_table),
        column_list,
        source_column_list,
        quote_plain_identifier(table),
        quote_plain_identifier(table),
        quote_plain_identifier(&temp_table),
        quote_plain_identifier(last_table_part(table))
    )
}

pub(super) fn sqlite_rebuild_table_statement(
    table: &str,
    columns: &[TableColumnDetail],
    source_names: &[String],
) -> String {
    let temp_table = temporary_table_name(table);
    let column_definitions = columns
        .iter()
        .map(|column| {
            reordered_column_definition(DatabaseDriver::Sqlite, column, quote_plain_identifier)
        })
        .collect::<Vec<_>>()
        .join(", ");
    let column_list = columns
        .iter()
        .map(|column| quote_plain_identifier(&column.name))
        .collect::<Vec<_>>()
        .join(", ");
    let source_column_list = source_column_list(source_names, columns, quote_plain_identifier);

    format!(
        "PRAGMA foreign_keys=OFF; BEGIN TRANSACTION; CREATE TABLE {} ({}); INSERT INTO {} ({}) SELECT {} FROM {}; DROP TABLE {}; ALTER TABLE {} RENAME TO {}; COMMIT; PRAGMA foreign_keys=ON;",
        quote_plain_identifier(&temp_table),
        column_definitions,
        quote_plain_identifier(&temp_table),
        column_list,
        source_column_list,
        quote_plain_identifier(table),
        quote_plain_identifier(table),
        quote_plain_identifier(&temp_table),
        quote_plain_identifier(last_table_part(table))
    )
}

pub(super) fn source_column_list(
    source_names: &[String],
    columns: &[TableColumnDetail],
    quote_column: fn(&str) -> String,
) -> String {
    columns
        .iter()
        .enumerate()
        .map(|(index, column)| {
            source_names
                .get(index)
                .map(String::as_str)
                .filter(|value| !value.trim().is_empty())
                .unwrap_or(column.name.as_str())
        })
        .map(quote_column)
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn add_column_source_column_list(
    original_column_names: &[String],
    columns: &[TableColumnDetail],
    quote_column: fn(&str) -> String,
) -> String {
    let original = original_column_names
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    columns
        .iter()
        .map(|column| {
            if original.contains(&column.name.to_ascii_lowercase()) {
                quote_column(&column.name)
            } else {
                inserted_column_expression(column)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn inserted_column_expression(column: &TableColumnDetail) -> String {
    let default_value = column.default_value.trim();
    if !default_value.is_empty() && !default_value.eq_ignore_ascii_case("NULL") {
        return normalized_column_default(column, default_value);
    }

    String::from("NULL")
}

pub(super) fn temporary_table_name(table: &str) -> String {
    if let Some((prefix, table_name)) = table.rsplit_once('.') {
        format!("{prefix}.{table_name}__aktsql_reorder")
    } else {
        format!("{table}__aktsql_reorder")
    }
}

pub(in crate::app) fn last_table_part(table: &str) -> &str {
    table.rsplit('.').next().unwrap_or(table)
}

pub(super) fn reordered_column_definition(
    driver: DatabaseDriver,
    column: &TableColumnDetail,
    quote_column: fn(&str) -> String,
) -> String {
    let mut parts = vec![
        quote_column(&column.name),
        normalized_reorder_type(driver, &column.data_type),
    ];
    let tail = table_column_definition_tail(column);
    if !tail.is_empty() {
        parts.push(tail);
    }
    if matches!(
        driver,
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb
    ) && column.extra.to_ascii_lowercase().contains("auto_increment")
        && !parts
            .iter()
            .any(|part| part.to_ascii_lowercase().contains("primary key"))
    {
        parts.push(String::from("PRIMARY KEY"));
    }
    parts.join(" ")
}

pub(super) fn normalized_reorder_type(driver: DatabaseDriver, data_type: &str) -> String {
    let data_type = data_type.trim();
    if data_type.is_empty() {
        match driver {
            DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
                String::from("TEXT")
            }
            DatabaseDriver::SqlServer => String::from("NVARCHAR(MAX)"),
            DatabaseDriver::Oracle => String::from("CLOB"),
            _ => String::from("TEXT"),
        }
    } else {
        data_type.to_owned()
    }
}

pub(super) fn mongodb_reorder_fields_command(table: &str, columns: &[TableColumnDetail]) -> String {
    let projected_fields = columns
        .iter()
        .map(|column| {
            format!(
                "\"{}\":\"${}\"",
                json_escape(&column.name),
                json_escape(&column.name)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!(
        "{{\"update\":\"{}\",\"updates\":[{{\"q\":{{}},\"u\":[{{\"$replaceRoot\":{{\"newRoot\":{{{}}}}}}}],\"multi\":true}}]}}",
        json_escape(table),
        projected_fields
    )
}

pub(super) fn alter_column_definition(
    driver: DatabaseDriver,
    draft: &AlterTableDraft,
) -> Result<String, String> {
    let column = draft.column_name().trim();
    let column_type = draft.column_type().trim();
    let extra = draft.column_definition().trim();

    if !column.is_empty() && !column_type.is_empty() {
        let quoted_column = match driver {
            DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
                quote_mysql_identifier(column)
            }
            DatabaseDriver::SqlServer => quote_sql_server_identifier(column),
            _ => quote_plain_identifier(column),
        };
        let extra_clause = if extra.is_empty() {
            String::new()
        } else {
            format!(" {extra}")
        };
        return Ok(format!("{quoted_column} {column_type}{extra_clause}"));
    }

    if !extra.is_empty() {
        Ok(extra.to_owned())
    } else {
        Err(String::from("Column name and data type are required."))
    }
}

pub(super) fn mysql_column_position_clause(draft: &AlterTableDraft) -> String {
    match draft.column_position().trim().to_uppercase().as_str() {
        "FIRST" => String::from(" FIRST"),
        "AFTER" => {
            let after = draft.after_column().trim();
            if after.is_empty() {
                String::new()
            } else {
                format!(" AFTER {}", quote_mysql_identifier(after))
            }
        }
        _ => String::new(),
    }
}
