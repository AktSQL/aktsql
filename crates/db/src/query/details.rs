use super::*;
use std::fs;

mod helpers;
use helpers::*;

pub(super) fn load_sqlite_database_details(
    form: &ConnectionForm,
    database: &str,
) -> Result<DatabaseDetails, String> {
    let path = form.location.trim();
    let connection = rusqlite::Connection::open(path)
        .map_err(|error| format!("SQLite connection failed: {error}"))?;
    let metadata = fs::metadata(path).ok();
    let file_size = metadata
        .as_ref()
        .map(|metadata| format_bytes(metadata.len()))
        .unwrap_or_else(|| String::from("unknown"));
    let readonly = metadata
        .as_ref()
        .map(|metadata| metadata.permissions().readonly().to_string())
        .unwrap_or_else(|| String::from("unknown"));
    let encoding = sqlite_pragma_string(&connection, "PRAGMA encoding")?;
    let journal_mode = sqlite_pragma_string(&connection, "PRAGMA journal_mode")?;
    let page_size = sqlite_pragma_i64(&connection, "PRAGMA page_size")?;
    let page_count = sqlite_pragma_i64(&connection, "PRAGMA page_count")?;
    let user_version = sqlite_pragma_i64(&connection, "PRAGMA user_version")?;
    let table_count = sqlite_count(
        &connection,
        "select count(*) from sqlite_schema where type = 'table' and name not like 'sqlite_%'",
    )?;
    let view_count = sqlite_count(
        &connection,
        "select count(*) from sqlite_schema where type = 'view'",
    )?;
    let index_count = sqlite_count(
        &connection,
        "select count(*) from sqlite_schema where type = 'index' and name not like 'sqlite_%'",
    )?;
    let label = if database.trim().is_empty() {
        path.to_owned()
    } else {
        database.to_owned()
    };

    Ok(DatabaseDetails {
        database: label,
        driver: DatabaseDriver::Sqlite,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("database file", path),
                    detail_field("driver", "SQLite"),
                    detail_field("encoding", encoding),
                    detail_field("read only", readonly),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("file size", file_size),
                    detail_field("page size", page_size.to_string()),
                    detail_field("page count", page_count.to_string()),
                    detail_field("journal mode", journal_mode),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("tables", table_count.to_string()),
                    detail_field("views", view_count.to_string()),
                    detail_field("indexes", index_count.to_string()),
                    detail_field("user version", user_version.to_string()),
                ],
            ),
        ],
    })
}

pub(super) fn load_mysql_database_details(
    form: &ConnectionForm,
    database: &str,
) -> Result<DatabaseDetails, String> {
    let mut detail_form = form.clone();
    detail_form.database = database.to_owned();
    let opts = mysql::Opts::from_url(&mysql_url(&detail_form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let mut connection = pool
        .get_conn()
        .map_err(|error| format!("MySQL connection failed: {error}"))?;
    let schema: Option<(String, String, String)> = connection
        .exec_first(
            "select schema_name, default_character_set_name, default_collation_name
             from information_schema.schemata
             where schema_name = ?",
            (database,),
        )
        .map_err(|error| format!("MySQL database detail query failed: {error}"))?;
    let Some((schema_name, charset, collation)) = schema else {
        return Err(format!("MySQL database {database} was not found."));
    };
    let counts: Option<(String, String, String, String)> = connection
        .exec_first(
            "select cast(count(*) as char),
                    cast(coalesce(sum(case when table_type = 'BASE TABLE' then 1 else 0 end), 0) as char),
                    cast(coalesce(sum(case when table_type = 'VIEW' then 1 else 0 end), 0) as char),
                    cast(coalesce(sum(data_length + index_length), 0) as char)
             from information_schema.tables
             where table_schema = ?",
            (database,),
        )
        .map_err(|error| format!("MySQL object detail query failed: {error}"))?;
    let (objects, tables, views, bytes) = counts.unwrap_or_else(|| {
        (
            String::from("0"),
            String::from("0"),
            String::from("0"),
            String::from("0"),
        )
    });
    let runtime: Option<(String, String, String)> = connection
        .query_first("select @@version, @@version_comment, @@sql_mode")
        .map_err(|error| format!("MySQL runtime detail query failed: {error}"))?;
    let (version, version_comment, sql_mode) = runtime.unwrap_or_else(|| {
        (
            String::from("unknown"),
            String::from("unknown"),
            String::from("unknown"),
        )
    });

    Ok(DatabaseDetails {
        database: schema_name.clone(),
        driver: form.driver,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("database", schema_name),
                    detail_field("driver", form.driver.to_string()),
                    detail_field("server", version),
                    detail_field("version comment", version_comment),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("default charset", charset),
                    detail_field("default collation", collation),
                    detail_field(
                        "data + index size",
                        format_bytes(bytes.parse().unwrap_or(0)),
                    ),
                    detail_field("engine policy", "per table"),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("schema objects", objects),
                    detail_field("base tables", tables),
                    detail_field("views", views),
                    detail_field("routines", "inspect information_schema.routines"),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Runtime,
                vec![
                    detail_field(
                        "host",
                        format!("{}:{}", form.location.trim(), form.port.trim()),
                    ),
                    detail_field("user", form.username.clone()),
                    detail_field("sql mode", sql_mode),
                    detail_field("timeout", format!("{}s", form.timeout_seconds.trim())),
                ],
            ),
        ],
    })
}

pub(super) fn load_postgres_database_details(
    form: &ConnectionForm,
    database: &str,
) -> Result<DatabaseDetails, String> {
    let mut detail_form = form.clone();
    detail_form.database = database.to_owned();
    let mut client = postgres_config(&detail_form)?
        .connect(postgres::NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let row = client
        .query_opt(
            "select d.datname,
                    pg_encoding_to_char(d.encoding),
                    d.datcollate,
                    d.datctype,
                    pg_catalog.pg_get_userbyid(d.datdba),
                    d.datconnlimit::text,
                    pg_size_pretty(pg_database_size(d.datname))
             from pg_database d
             where d.datname = $1",
            &[&database],
        )
        .map_err(|error| format!("PostgreSQL database detail query failed: {error}"))?;
    let Some(row) = row else {
        return Err(format!("PostgreSQL database {database} was not found."));
    };
    let database_name: String = row.get(0);
    let encoding: String = row.get(1);
    let collate: String = row.get(2);
    let ctype: String = row.get(3);
    let owner: String = row.get(4);
    let connection_limit: String = row.get(5);
    let size: String = row.get(6);
    let counts = client
        .query_one(
            "select count(*)::text,
                    count(*) filter (where table_type = 'BASE TABLE')::text,
                    count(*) filter (where table_type = 'VIEW')::text
             from information_schema.tables
             where table_schema not in ('pg_catalog', 'information_schema')",
            &[],
        )
        .map_err(|error| format!("PostgreSQL object detail query failed: {error}"))?;
    let server_version = client
        .query_one("select version()", &[])
        .map_err(|error| format!("PostgreSQL runtime detail query failed: {error}"))?
        .get::<usize, String>(0);

    Ok(DatabaseDetails {
        database: database_name.clone(),
        driver: form.driver,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("database", database_name),
                    detail_field("driver", form.driver.to_string()),
                    detail_field("owner", owner),
                    detail_field("encoding", encoding),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("size", size),
                    detail_field("collate", collate),
                    detail_field("ctype", ctype),
                    detail_field("connection limit", connection_limit),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("schema objects", counts.get::<usize, String>(0)),
                    detail_field("base tables", counts.get::<usize, String>(1)),
                    detail_field("views", counts.get::<usize, String>(2)),
                    detail_field("schemas", "inspect pg_namespace"),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Runtime,
                vec![
                    detail_field(
                        "host",
                        format!("{}:{}", form.location.trim(), form.port.trim()),
                    ),
                    detail_field("user", form.username.clone()),
                    detail_field("server version", server_version),
                    detail_field("timeout", format!("{}s", form.timeout_seconds.trim())),
                ],
            ),
        ],
    })
}

pub(super) fn load_mongodb_database_details(
    form: &ConnectionForm,
    database: &str,
) -> Result<DatabaseDetails, String> {
    let mut detail_form = form.clone();
    detail_form.database = database.to_owned();
    let options = mongodb::options::ClientOptions::parse(mongodb_url(&detail_form))
        .map_err(|error| format!("MongoDB connection options failed: {error}"))?;
    let client = MongoClient::with_options(options)
        .map_err(|error| format!("MongoDB client creation failed: {error}"))?;
    let db = client.database(database);
    let stats = db
        .run_command(bson::doc! { "dbStats": 1 }, None)
        .map_err(|error| format!("MongoDB database detail query failed: {error}"))?;
    let build_info = client
        .database("admin")
        .run_command(bson::doc! { "buildInfo": 1 }, None)
        .unwrap_or_else(|_| Document::new());

    Ok(DatabaseDetails {
        database: database.to_owned(),
        driver: DatabaseDriver::MongoDb,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("database", database),
                    detail_field("driver", "MongoDB"),
                    detail_field("server version", bson_value(&build_info, "version")),
                    detail_field("storage engine", bson_document_value(&stats, "wiredTiger")),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("data size", bson_size(&stats, "dataSize")),
                    detail_field("storage size", bson_size(&stats, "storageSize")),
                    detail_field("index size", bson_size(&stats, "indexSize")),
                    detail_field("scale factor", bson_value(&stats, "scaleFactor")),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("collections", bson_value(&stats, "collections")),
                    detail_field("views", bson_value(&stats, "views")),
                    detail_field("documents", bson_value(&stats, "objects")),
                    detail_field("indexes", bson_value(&stats, "indexes")),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Runtime,
                vec![
                    detail_field(
                        "host",
                        format!("{}:{}", form.location.trim(), form.port.trim()),
                    ),
                    detail_field("user", form.username.clone()),
                    detail_field("auth source", "admin"),
                    detail_field("timeout", format!("{}s", form.timeout_seconds.trim())),
                ],
            ),
        ],
    })
}

pub(super) fn fallback_database_details(form: &ConnectionForm, database: &str) -> DatabaseDetails {
    DatabaseDetails {
        database: database.to_owned(),
        driver: form.driver,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("database", database),
                    detail_field("driver", form.driver.to_string()),
                    detail_field("metadata", "native driver pending"),
                    detail_field("mode", "read-only fallback"),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Runtime,
                vec![
                    detail_field(
                        "host",
                        format!("{}:{}", form.location.trim(), form.port.trim()),
                    ),
                    detail_field("user", form.username.clone()),
                    detail_field("timeout", format!("{}s", form.timeout_seconds.trim())),
                    detail_field("notes", form.notes.clone()),
                ],
            ),
        ],
    }
}

pub(super) fn load_sqlite_table_details(
    form: &ConnectionForm,
    table: &str,
) -> Result<TableDetails, String> {
    let connection = rusqlite::Connection::open(form.location.trim())
        .map_err(|error| format!("SQLite connection failed: {error}"))?;
    let table_sql = connection
        .query_row(
            "select sql from sqlite_schema where name = ?1 and type in ('table', 'view')",
            [table],
            |row| row.get::<_, Option<String>>(0),
        )
        .map_err(|error| format!("SQLite table detail query failed: {error}"))?
        .unwrap_or_default();
    let columns = sqlite_table_columns(&connection, table)?;
    let indexes = sqlite_table_indexes(&connection, table)?;
    let row_count = sqlite_count(
        &connection,
        format!("select count(*) from {}", quote_sql_identifier(table)).as_str(),
    )
    .map(|value| value.to_string())
    .unwrap_or_else(|_| String::from("unknown"));

    Ok(TableDetails {
        table: table.to_owned(),
        driver: DatabaseDriver::Sqlite,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("table", table),
                    detail_field("driver", "SQLite"),
                    detail_field("object type", "table/view"),
                    detail_field("row count", row_count),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("columns", columns.len().to_string()),
                    detail_field("indexes", indexes.len().to_string()),
                    detail_field("database file", form.location.clone()),
                    detail_field("ddl source", "sqlite_schema.sql"),
                ],
            ),
        ],
        columns,
        indexes,
        create_statement: table_sql,
    })
}

fn sqlite_table_columns(
    connection: &rusqlite::Connection,
    table: &str,
) -> Result<Vec<TableColumnDetail>, String> {
    let mut statement = connection
        .prepare(format!("PRAGMA table_xinfo({})", quote_sql_identifier(table)).as_str())
        .map_err(|error| format!("SQLite table_xinfo prepare failed: {error}"))?;
    let rows = statement
        .query_map([], |row| {
            let not_null: i64 = row.get(3)?;
            let hidden: i64 = row.get(6).unwrap_or(0);
            Ok(TableColumnDetail {
                name: row.get::<_, String>(1)?,
                data_type: row.get::<_, String>(2)?,
                nullable: if not_null == 0 { "YES" } else { "NO" }.to_owned(),
                default_value: row.get::<_, Option<String>>(4)?.unwrap_or_default(),
                extra: if hidden == 0 {
                    String::new()
                } else {
                    format!("hidden={hidden}")
                },
            })
        })
        .map_err(|error| format!("SQLite table_xinfo query failed: {error}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("SQLite table_xinfo row read failed: {error}"))
}

fn sqlite_table_indexes(
    connection: &rusqlite::Connection,
    table: &str,
) -> Result<Vec<TableIndexDetail>, String> {
    let mut statement = connection
        .prepare(format!("PRAGMA index_list({})", quote_sql_identifier(table)).as_str())
        .map_err(|error| format!("SQLite index_list prepare failed: {error}"))?;
    let rows = statement
        .query_map([], |row| {
            let unique: i64 = row.get(2)?;
            Ok(TableIndexDetail {
                name: row.get::<_, String>(1)?,
                columns: String::from("inspect index_xinfo"),
                unique: if unique == 1 { "YES" } else { "NO" }.to_owned(),
            })
        })
        .map_err(|error| format!("SQLite index_list query failed: {error}"))?;

    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("SQLite index_list row read failed: {error}"))
}

pub(super) fn load_mysql_table_details(
    form: &ConnectionForm,
    table: &str,
) -> Result<TableDetails, String> {
    let database = form.database.trim();
    if database.is_empty() {
        return Err(String::from("Select a database before describing a table."));
    }

    let opts = mysql::Opts::from_url(&mysql_url(form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let mut connection = pool
        .get_conn()
        .map_err(|error| format!("MySQL connection failed: {error}"))?;
    let table_row: Option<(String, String, Option<String>, Option<String>, Option<u64>, Option<u64>, Option<u64>)> = connection
        .exec_first(
            "select table_name, table_type, engine, table_collation, table_rows, data_length, index_length
             from information_schema.tables
             where table_schema = ? and table_name = ?",
            (database, table),
        )
        .map_err(|error| format!("MySQL table detail query failed: {error}"))?;
    let Some((
        table_name,
        table_type,
        engine,
        table_collation,
        table_rows,
        data_length,
        index_length,
    )) = table_row
    else {
        return Err(format!("MySQL table {database}.{table} was not found."));
    };
    let columns: Vec<TableColumnDetail> = connection
        .exec_map(
            "select column_name, column_type, is_nullable, coalesce(column_default, ''), extra
             from information_schema.columns
             where table_schema = ? and table_name = ?
             order by ordinal_position",
            (database, table),
            |(name, data_type, nullable, default_value, extra)| TableColumnDetail {
                name,
                data_type,
                nullable,
                default_value,
                extra,
            },
        )
        .map_err(|error| format!("MySQL column detail query failed: {error}"))?;
    let indexes: Vec<TableIndexDetail> = connection
        .exec_map(
            "select index_name,
                    group_concat(column_name order by seq_in_index separator ', '),
                    if(non_unique = 0, 'YES', 'NO')
             from information_schema.statistics
             where table_schema = ? and table_name = ?
             group by index_name, non_unique
             order by index_name",
            (database, table),
            |(name, columns, unique)| TableIndexDetail {
                name,
                columns,
                unique,
            },
        )
        .map_err(|error| format!("MySQL index detail query failed: {error}"))?;
    let create_statement = connection
        .query_first::<(String, String), _>(format!(
            "SHOW CREATE TABLE {}.{}",
            quote_mysql_identifier(database),
            quote_mysql_identifier(table)
        ))
        .map_err(|error| format!("MySQL SHOW CREATE TABLE failed: {error}"))?
        .map(|(_, create_statement)| create_statement)
        .unwrap_or_default();

    Ok(TableDetails {
        table: table_name.clone(),
        driver: form.driver,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("table", table_name),
                    detail_field("database", database),
                    detail_field("object type", table_type),
                    detail_field("engine", engine.unwrap_or_default()),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("collation", table_collation.unwrap_or_default()),
                    detail_field("estimated rows", table_rows.unwrap_or(0).to_string()),
                    detail_field("data size", format_bytes(data_length.unwrap_or(0))),
                    detail_field("index size", format_bytes(index_length.unwrap_or(0))),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("columns", columns.len().to_string()),
                    detail_field("indexes", indexes.len().to_string()),
                    detail_field("ddl source", "SHOW CREATE TABLE"),
                    detail_field("row read page", "100"),
                ],
            ),
        ],
        columns,
        indexes,
        create_statement,
    })
}

pub(super) fn load_postgres_table_details(
    form: &ConnectionForm,
    table: &str,
) -> Result<TableDetails, String> {
    let (schema, name) = split_postgres_table(table);
    let mut client = postgres_config(form)?
        .connect(postgres::NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let table_row = client
        .query_opt(
            "select table_schema, table_name, table_type
             from information_schema.tables
             where table_schema = $1 and table_name = $2",
            &[&schema, &name],
        )
        .map_err(|error| format!("PostgreSQL table detail query failed: {error}"))?;
    let Some(table_row) = table_row else {
        return Err(format!("PostgreSQL table {schema}.{name} was not found."));
    };
    let table_type: String = table_row.get(2);
    let column_rows = client
        .query(
            "select column_name, data_type, is_nullable, coalesce(column_default, ''), coalesce(character_maximum_length::text, '')
             from information_schema.columns
             where table_schema = $1 and table_name = $2
             order by ordinal_position",
            &[&schema, &name],
        )
        .map_err(|error| format!("PostgreSQL column detail query failed: {error}"))?;
    let columns = column_rows
        .into_iter()
        .map(|row| TableColumnDetail {
            name: row.get(0),
            data_type: {
                let data_type: String = row.get(1);
                let max_len: String = row.get(4);
                if max_len.is_empty() {
                    data_type
                } else {
                    format!("{data_type}({max_len})")
                }
            },
            nullable: row.get(2),
            default_value: row.get(3),
            extra: String::new(),
        })
        .collect::<Vec<_>>();
    let index_rows = client
        .query(
            "select indexname, indexdef
             from pg_indexes
             where schemaname = $1 and tablename = $2
             order by indexname",
            &[&schema, &name],
        )
        .map_err(|error| format!("PostgreSQL index detail query failed: {error}"))?;
    let indexes = index_rows
        .into_iter()
        .map(|row| {
            let indexdef: String = row.get(1);
            TableIndexDetail {
                name: row.get(0),
                columns: indexdef,
                unique: String::from("see definition"),
            }
        })
        .collect::<Vec<_>>();
    let stats = client
        .query_opt(
            "select coalesce(c.reltuples::bigint, 0)::text,
                    pg_size_pretty(pg_total_relation_size(c.oid)),
                    coalesce(obj_description(c.oid), '')
             from pg_class c
             join pg_namespace n on n.oid = c.relnamespace
             where n.nspname = $1 and c.relname = $2",
            &[&schema, &name],
        )
        .map_err(|error| format!("PostgreSQL table stats query failed: {error}"))?;
    let (estimated_rows, total_size, comment) = stats
        .map(|row| {
            (
                row.get::<usize, String>(0),
                row.get::<usize, String>(1),
                row.get::<usize, String>(2),
            )
        })
        .unwrap_or_else(|| (String::from("0"), String::from("unknown"), String::new()));

    Ok(TableDetails {
        table: format!("{schema}.{name}"),
        driver: form.driver,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("table", format!("{schema}.{name}")),
                    detail_field("schema", schema.clone()),
                    detail_field("object type", table_type),
                    detail_field("comment", comment),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("estimated rows", estimated_rows),
                    detail_field("total size", total_size),
                    detail_field("database", form.database.clone()),
                    detail_field("ddl source", "pg_catalog reconstruction"),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("columns", columns.len().to_string()),
                    detail_field("indexes", indexes.len().to_string()),
                    detail_field("constraints", "inspect pg_constraint"),
                    detail_field("row read page", "100"),
                ],
            ),
        ],
        columns,
        indexes,
        create_statement: postgres_create_table_statement(&mut client, &schema, &name)?,
    })
}

pub(super) fn load_mongodb_table_details(
    form: &ConnectionForm,
    table: &str,
) -> Result<TableDetails, String> {
    let options = mongodb::options::ClientOptions::parse(mongodb_url(form))
        .map_err(|error| format!("MongoDB connection options failed: {error}"))?;
    let client = MongoClient::with_options(options)
        .map_err(|error| format!("MongoDB client creation failed: {error}"))?;
    let database = client.database(database_name(form).as_str());
    let stats = database
        .run_command(bson::doc! { "collStats": table }, None)
        .map_err(|error| format!("MongoDB collection detail query failed: {error}"))?;
    let indexes = database
        .collection::<Document>(table)
        .list_indexes(None)
        .map_err(|error| format!("MongoDB index listing failed: {error}"))?
        .filter_map(Result::ok)
        .map(|model| TableIndexDetail {
            name: model
                .options
                .and_then(|options| options.name)
                .unwrap_or_default(),
            columns: format!("{:?}", model.keys),
            unique: String::from("see options"),
        })
        .collect::<Vec<_>>();

    Ok(TableDetails {
        table: table.to_owned(),
        driver: DatabaseDriver::MongoDb,
        sections: vec![
            detail_section(
                DatabaseDetailSectionKind::Core,
                vec![
                    detail_field("collection", table),
                    detail_field("database", database_name(form)),
                    detail_field("type", "collection"),
                    detail_field("documents", bson_value(&stats, "count")),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Storage,
                vec![
                    detail_field("size", bson_size(&stats, "size")),
                    detail_field("storage size", bson_size(&stats, "storageSize")),
                    detail_field("total index size", bson_size(&stats, "totalIndexSize")),
                    detail_field("avg object size", bson_value(&stats, "avgObjSize")),
                ],
            ),
            detail_section(
                DatabaseDetailSectionKind::Objects,
                vec![
                    detail_field("indexes", indexes.len().to_string()),
                    detail_field("nindexes", bson_value(&stats, "nindexes")),
                    detail_field("capped", bson_value(&stats, "capped")),
                    detail_field("row read page", "100"),
                ],
            ),
        ],
        columns: Vec::new(),
        indexes,
        create_statement: format!("{{ \"create\": \"{}\" }}", json_escape(table)),
    })
}

pub(super) fn fallback_table_details(form: &ConnectionForm, table: &str) -> TableDetails {
    TableDetails {
        table: table.to_owned(),
        driver: form.driver,
        sections: vec![detail_section(
            DatabaseDetailSectionKind::Core,
            vec![
                detail_field("table", table),
                detail_field("driver", form.driver.to_string()),
                detail_field("metadata", "native driver pending"),
                detail_field("mode", "read-only fallback"),
            ],
        )],
        columns: Vec::new(),
        indexes: Vec::new(),
        create_statement: String::from("Native driver metadata is not connected yet."),
    }
}
