use super::*;

pub(in crate::app) struct CreateDatabaseCommand {
    pub(in crate::app) form: ConnectionForm,
    pub(in crate::app) sql: String,
}

pub(in crate::app) fn create_database_command(
    mut form: ConnectionForm,
    draft: &CreateDatabaseDraft,
) -> Result<CreateDatabaseCommand, String> {
    let database = draft.name.trim();
    let sql = match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            create_database_template(form.driver, database, draft.charset(), draft.collation())
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            create_postgres_database_statement(draft)
        }
        DatabaseDriver::MongoDb => {
            form.database = database.to_owned();
            format!(
                "{{ \"create\": \"{}\" }}",
                json_escape(draft.initial_collection().trim())
            )
        }
        DatabaseDriver::Sqlite => {
            return Err(String::from(
                "SQLite databases are files. Create or select a SQLite file connection instead.",
            ));
        }
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => {
            return Err(format!(
                "{} database creation needs a native driver dependency before direct execution is available.",
                form.driver
            ));
        }
    };

    Ok(CreateDatabaseCommand { form, sql })
}

pub(in crate::app) fn pending_database_deletion(
    mut form: ConnectionForm,
    database: &str,
) -> Result<PendingSchemaDeletion, String> {
    let database = database.trim();
    if database.is_empty() && form.driver != DatabaseDriver::Sqlite {
        return Err(String::from("Select a database before deleting it."));
    }

    let sql = match form.driver {
        DatabaseDriver::Sqlite => {
            return Err(String::from(
                "SQLite databases are files. Delete the file only after closing the connection.",
            ));
        }
        DatabaseDriver::MongoDb => {
            form.database = database.to_owned();
            String::from("{ \"dropDatabase\": 1 }")
        }
        _ => drop_database_template(form.driver, database),
    };

    Ok(PendingSchemaDeletion {
        kind: SchemaDeletionKind::Database,
        name: database.to_owned(),
        form,
        sql,
    })
}

pub(in crate::app) fn pending_table_deletion(
    form: ConnectionForm,
    object: &crate::query::SchemaObject,
) -> Result<PendingSchemaDeletion, String> {
    let table = object.name.trim();
    if table.is_empty() {
        return Err(String::from("Table name is required."));
    }

    let sql = match form.driver {
        DatabaseDriver::MongoDb => format!("{{ \"drop\": \"{}\" }}", json_escape(table)),
        DatabaseDriver::Sqlite => format!("DROP TABLE {};", quote_plain_identifier(table)),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before deleting a table."));
            }
            format!(
                "DROP TABLE {}.{};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table)
            )
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            format!("DROP TABLE {};", quote_dotted_identifier(table))
        }
        DatabaseDriver::SqlServer => format!("DROP TABLE {};", quote_sql_server_table(table)),
        DatabaseDriver::Oracle => format!("DROP TABLE {};", quote_plain_identifier(table)),
    };

    if form.driver == DatabaseDriver::MongoDb && form.database.trim().is_empty() {
        return Err(String::from(
            "Select a MongoDB database before deleting a collection.",
        ));
    }

    Ok(PendingSchemaDeletion {
        kind: SchemaDeletionKind::Table,
        name: table.to_owned(),
        form,
        sql,
    })
}

pub(in crate::app) fn rename_database_statement(
    form: &mut ConnectionForm,
    database: &str,
    new_name: &str,
    child_objects: &[String],
    charset: &str,
    collation: &str,
) -> Result<String, String> {
    let database = database.trim();
    let new_name = new_name.trim();
    if database.is_empty() || new_name.is_empty() {
        return Err(String::from("Database name is required."));
    }

    match form.driver {
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            form.database = String::from("postgres");
            Ok(format!(
            "ALTER DATABASE {} RENAME TO {};",
            quote_plain_identifier(database),
            quote_plain_identifier(new_name)
        ))
        }
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            form.database.clear();
            let new_target = quote_mysql_identifier(new_name);
            let new_rename_prefix = format!(" TO {new_target}.");
            let mut statements = vec![format!(
                "CREATE DATABASE {}\n    CHARACTER SET {}\n    COLLATE {}",
                new_target,
                mysql_charset_or_default(charset),
                mysql_collation_or_default(collation)
            )];
            statements.push(String::from("SET SESSION group_concat_max_len = 1048576"));
            statements.push(format!(
                "SET @aktsql_rename_database_sql = (
    SELECT CASE
        WHEN COUNT(*) = 0 THEN 'DO 0'
        ELSE CONCAT(
            'RENAME TABLE ',
            GROUP_CONCAT(
                CONCAT(
                    '`', REPLACE(TABLE_SCHEMA, '`', '``'), '`.`',
                    REPLACE(TABLE_NAME, '`', '``'), '`',
                    {},
                    '`', REPLACE(TABLE_NAME, '`', '``'), '`'
                )
                SEPARATOR ', '
            )
        )
    END
    FROM information_schema.tables
    WHERE table_schema = {}
)",
                sql_string_literal(&new_rename_prefix),
                sql_string_literal(database)
            ));
            statements.push(String::from(
                "PREPARE aktsql_rename_database_stmt FROM @aktsql_rename_database_sql",
            ));
            statements.push(String::from("EXECUTE aktsql_rename_database_stmt"));
            statements.push(String::from("DEALLOCATE PREPARE aktsql_rename_database_stmt"));
            statements.push(format!("DROP DATABASE {}", quote_mysql_identifier(database)));
            Ok(format!("{};", statements.join(";\n")))
        }
        DatabaseDriver::MongoDb => {
            form.database = String::from("admin");
            if child_objects.is_empty() {
                return Err(String::from(
                    "MongoDB database rename needs loaded collection names. Refresh and expand the database first.",
                ));
            }

            let commands = child_objects
                .iter()
                .map(|collection| {
                    format!(
                        "{{\"renameCollection\":\"{}.{}\",\"to\":\"{}.{}\",\"dropTarget\":false}}",
                        json_escape(database),
                        json_escape(collection),
                        json_escape(new_name),
                        json_escape(collection)
                    )
                })
                .collect::<Vec<_>>()
                .join(",");
            Ok(format!("[{commands}]"))
        }
        DatabaseDriver::Sqlite => Err(String::from(
            "SQLite database rename is a file rename. Close the connection and rename the database file.",
        )),
        DatabaseDriver::SqlServer => {
            form.database = String::from("master");
            Ok(format!(
                "ALTER DATABASE {} MODIFY NAME = {};",
                quote_sql_server_identifier(database),
                quote_sql_server_identifier(new_name)
            ))
        }
        DatabaseDriver::Oracle => Ok(format!(
            "ALTER PLUGGABLE DATABASE {} RENAME GLOBAL_NAME TO {};",
            quote_plain_identifier(database),
            quote_plain_identifier(new_name)
        )),
    }
}

pub(in crate::app) fn rename_table_statement(
    form: &mut ConnectionForm,
    table: &str,
    new_name: &str,
) -> Result<String, String> {
    let table = table.trim();
    let new_name = new_name.trim();
    if table.is_empty() || new_name.is_empty() {
        return Err(String::from("Table name is required."));
    }

    match form.driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            let database = form.database.trim();
            if database.is_empty() {
                return Err(String::from("Select a database before renaming a table."));
            }
            Ok(format!(
                "RENAME TABLE {}.{} TO {}.{};",
                quote_mysql_identifier(database),
                quote_mysql_identifier(table),
                quote_mysql_identifier(database),
                quote_mysql_identifier(new_name)
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => Ok(format!(
            "ALTER TABLE {} RENAME TO {};",
            quote_dotted_identifier(table),
            quote_plain_identifier(new_name)
        )),
        DatabaseDriver::Sqlite => Ok(format!(
            "ALTER TABLE {} RENAME TO {};",
            quote_plain_identifier(table),
            quote_plain_identifier(new_name)
        )),
        DatabaseDriver::MongoDb => {
            let database = form.database.trim().to_owned();
            if database.is_empty() {
                return Err(String::from(
                    "Select a MongoDB database before renaming a collection.",
                ));
            }
            form.database = String::from("admin");
            Ok(format!(
                "{{\"renameCollection\":\"{}.{}\",\"to\":\"{}.{}\",\"dropTarget\":false}}",
                json_escape(&database),
                json_escape(table),
                json_escape(&database),
                json_escape(new_name)
            ))
        }
        DatabaseDriver::SqlServer => Ok(format!(
            "EXEC sp_rename {}, {}, 'OBJECT';",
            sql_string_literal(table),
            sql_string_literal(new_name)
        )),
        DatabaseDriver::Oracle => Ok(format!(
            "ALTER TABLE {} RENAME TO {};",
            quote_plain_identifier(table),
            quote_plain_identifier(new_name)
        )),
    }
}

pub(in crate::app) fn alter_database_charset_statement(
    driver: DatabaseDriver,
    database: &str,
    charset: &str,
    collation: &str,
) -> Result<String, String> {
    let database = database.trim();
    if database.is_empty() {
        return Err(String::from("Database name is required."));
    }

    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            Ok(edit_database_charset_template(
                driver, database, charset, collation,
            ))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => Err(String::from(
            "PostgreSQL-compatible database encoding cannot be changed in-place. Create a new database with the target encoding and migrate data.",
        )),
        DatabaseDriver::MongoDb => Err(String::from(
            "MongoDB stores strings as UTF-8 BSON and has no database-level charset setting.",
        )),
        DatabaseDriver::Sqlite => Err(String::from(
            "SQLite PRAGMA encoding only applies before database schema creation.",
        )),
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => Err(format!(
            "{} charset changes need a native driver before direct execution is available.",
            driver
        )),
    }
}

pub(in crate::app) fn create_postgres_database_statement(draft: &CreateDatabaseDraft) -> String {
    let mut options = vec![format!(
        "ENCODING '{}'",
        postgres_encoding_or_default(draft.charset())
    )];

    let owner = draft.owner().trim();
    if !owner.is_empty() {
        options.push(format!("OWNER {}", quote_plain_identifier(owner)));
    }

    let template = draft.template().trim();
    if !template.is_empty() {
        options.push(format!("TEMPLATE {}", quote_plain_identifier(template)));
    }

    let tablespace = draft.tablespace().trim();
    if !tablespace.is_empty() {
        options.push(format!("TABLESPACE {}", quote_plain_identifier(tablespace)));
    }

    format!(
        "CREATE DATABASE {}\n    WITH {};",
        quote_plain_identifier(draft.name().trim()),
        options.join("\n    ")
    )
}

pub(in crate::app) fn create_database_template(
    driver: DatabaseDriver,
    database: &str,
    charset: &str,
    collation: &str,
) -> String {
    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => format!(
            "CREATE DATABASE `{}`\n    CHARACTER SET {}\n    COLLATE {};",
            database.replace('`', "``"),
            mysql_charset_or_default(charset),
            mysql_collation_or_default(collation)
        ),
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => format!(
            "CREATE DATABASE {}\n    ENCODING '{}';",
            quote_plain_identifier(database),
            postgres_encoding_or_default(charset)
        ),
        DatabaseDriver::SqlServer => format!(
            "CREATE DATABASE [{}]\nCOLLATE {};",
            database.replace(']', "]]"),
            sql_server_collation_or_default(collation)
        ),
        DatabaseDriver::Oracle => String::from(
            "-- Oracle database character set is normally selected at database creation time with DBCA or CREATE DATABASE.\n-- Review NLS_CHARACTERSET and NLS_NCHAR_CHARACTERSET before changing production systems.",
        ),
        DatabaseDriver::MongoDb => format!("use {};\ndb.createCollection(\"new_collection\");", database),
        DatabaseDriver::Sqlite => String::from(
            "-- SQLite stores text as UTF-8 or UTF-16 per database encoding before schema creation.\nPRAGMA encoding = 'UTF-8';\nCREATE TABLE new_table (\n    id INTEGER PRIMARY KEY AUTOINCREMENT\n);",
        ),
    }
}

pub(in crate::app) fn edit_database_charset_template(
    driver: DatabaseDriver,
    database: &str,
    charset: &str,
    collation: &str,
) -> String {
    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => format!(
            "ALTER DATABASE `{}`\n    CHARACTER SET {}\n    COLLATE {};",
            database.replace('`', "``"),
            mysql_charset_or_default(charset),
            mysql_collation_or_default(collation)
        ),
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => format!(
            "-- PostgreSQL/CockroachDB database encoding cannot be changed in-place safely.\n-- Recommended flow: create a new database with the target encoding, migrate data, then switch clients.\nCREATE DATABASE {}_new\n    ENCODING '{}';",
            quote_plain_identifier(database),
            postgres_encoding_or_default(charset)
        ),
        DatabaseDriver::SqlServer => format!(
            "ALTER DATABASE [{}]\nCOLLATE {};",
            database.replace(']', "]]"),
            sql_server_collation_or_default(collation)
        ),
        DatabaseDriver::Oracle => String::from(
            "-- Oracle character set changes require DMU/CSSCAN planning and are not a simple ALTER DATABASE operation.\n-- Inspect current values first:\nSELECT parameter, value FROM nls_database_parameters WHERE parameter IN ('NLS_CHARACTERSET', 'NLS_NCHAR_CHARACTERSET');",
        ),
        DatabaseDriver::MongoDb => String::from(
            "-- MongoDB stores strings as UTF-8 BSON. There is no per-database charset/collation change.\n-- Configure collation per collection or index, for example:\ndb.runCommand({ collMod: \"collection_name\", collation: { locale: \"en\", strength: 2 } });",
        ),
        DatabaseDriver::Sqlite => String::from(
            "-- SQLite PRAGMA encoding only applies before database schema creation.\nPRAGMA encoding;",
        ),
    }
}

pub(in crate::app) fn drop_database_template(driver: DatabaseDriver, database: &str) -> String {
    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            format!("DROP DATABASE `{}`;", database.replace('`', "``"))
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            format!("DROP DATABASE {};", quote_plain_identifier(database))
        }
        DatabaseDriver::SqlServer => {
            format!("DROP DATABASE [{}];", database.replace(']', "]]"))
        }
        DatabaseDriver::Oracle => String::from(
            "-- Oracle DROP DATABASE requires mounted exclusive mode and is intentionally not generated here.",
        ),
        DatabaseDriver::MongoDb => format!("use {};\ndb.dropDatabase();", database),
        DatabaseDriver::Sqlite => {
            String::from("-- SQLite databases are files. Delete the file only after closing all connections.")
        }
    }
}

pub(in crate::app) fn database_action_target(driver: DatabaseDriver, database: &str) -> String {
    if database.is_empty() && driver == DatabaseDriver::Sqlite {
        String::from("SQLite database file")
    } else {
        database.to_owned()
    }
}
