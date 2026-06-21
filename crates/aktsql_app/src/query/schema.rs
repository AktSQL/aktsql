use super::*;
use mongodb::sync::Client as MongoClient;
use mysql::prelude::Queryable;

pub(super) fn load_schema(
    form: &ConnectionForm,
    expand_selected_database_in_tree: bool,
) -> Result<Vec<SchemaObject>, Vec<String>> {
    form.validate()?;

    let result = match form.driver {
        DatabaseDriver::Sqlite => load_sqlite_schema(form),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            load_mysql_schema(form, expand_selected_database_in_tree)
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            load_postgres_schema(form, expand_selected_database_in_tree)
        }
        DatabaseDriver::MongoDb => load_mongodb_schema(form, expand_selected_database_in_tree),
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => Err(format!(
            "{} schema browsing needs a native driver dependency before it can inspect metadata.",
            form.driver
        )),
    };

    result.map_err(|error| vec![error])
}

fn load_sqlite_schema(form: &ConnectionForm) -> Result<Vec<SchemaObject>, String> {
    let connection = rusqlite::Connection::open(form.location.trim())
        .map_err(|error| format!("SQLite connection failed: {error}"))?;
    let database_label = form
        .location
        .trim()
        .rsplit('/')
        .next()
        .filter(|value| !value.is_empty())
        .unwrap_or("main");
    let mut objects = vec![SchemaObject::new(
        database_label.to_owned(),
        SchemaObjectKind::Database,
        String::from(
            "select type, name from sqlite_master where type in ('table', 'view') order by name;",
        ),
    )];
    let mut statement = connection
        .prepare(
            "select type, name
             from sqlite_master
             where type in ('table', 'view', 'index')
               and name not like 'sqlite_%'
             order by case type when 'table' then 0 when 'view' then 1 else 2 end, name",
        )
        .map_err(|error| format!("SQLite schema prepare failed: {error}"))?;
    let rows = statement
        .query_map([], |row| {
            let object_type: String = row.get(0)?;
            let name: String = row.get(1)?;

            Ok((object_type, name))
        })
        .map_err(|error| format!("SQLite schema query failed: {error}"))?;

    for row in rows {
        let (object_type, name) =
            row.map_err(|error| format!("SQLite schema row read failed: {error}"))?;

        if let Some(kind) = SchemaObjectKind::from_sqlite_type(&object_type) {
            let preview = match kind {
                SchemaObjectKind::Table | SchemaObjectKind::View => {
                    format!("select * from {} limit 100;", quote_sql_identifier(&name))
                }
                SchemaObjectKind::Index => {
                    format!("pragma index_info({});", quote_sql_identifier(&name))
                }
                SchemaObjectKind::Database
                | SchemaObjectKind::Collection
                | SchemaObjectKind::Column => {
                    format!("select * from {} limit 100;", quote_sql_identifier(&name))
                }
            };
            objects.push(SchemaObject::new(name, kind, preview));
        }
    }

    Ok(objects)
}

pub(super) fn load_schema_columns(
    form: &ConnectionForm,
    object_name: &str,
) -> Result<Vec<SchemaObject>, Vec<String>> {
    form.validate()?;

    let result = match form.driver {
        DatabaseDriver::Sqlite => load_sqlite_schema_columns(form, object_name),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            load_mysql_schema_columns(form, object_name)
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            load_postgres_schema_columns(form, object_name)
        }
        DatabaseDriver::MongoDb => Ok(Vec::new()),
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => Ok(Vec::new()),
    };

    result.map_err(|error| vec![error])
}

fn load_sqlite_schema_columns(
    form: &ConnectionForm,
    object_name: &str,
) -> Result<Vec<SchemaObject>, String> {
    let connection = rusqlite::Connection::open(form.location.trim())
        .map_err(|error| format!("SQLite connection failed: {error}"))?;

    load_sqlite_columns(&connection, object_name)
}

fn load_sqlite_columns(
    connection: &rusqlite::Connection,
    table_name: &str,
) -> Result<Vec<SchemaObject>, String> {
    let sql = format!(
        "select name, type from pragma_table_info({}) order by cid",
        sql_string_literal(table_name)
    );
    let mut statement = connection
        .prepare(&sql)
        .map_err(|error| format!("SQLite column prepare failed: {error}"))?;
    let rows = statement
        .query_map([], |row| {
            let name: String = row.get(0)?;
            let column_type: String = row.get(1)?;

            Ok((name, column_type))
        })
        .map_err(|error| format!("SQLite column query failed: {error}"))?;
    let mut columns = Vec::new();

    for row in rows {
        let (name, column_type) =
            row.map_err(|error| format!("SQLite column read failed: {error}"))?;
        columns.push(SchemaObject::new(
            column_label(&name, &column_type),
            SchemaObjectKind::Column,
            format!(
                "select {} from {} limit 100;",
                quote_sql_identifier(&name),
                quote_sql_identifier(table_name)
            ),
        ));
    }

    Ok(columns)
}

fn load_mysql_schema(
    form: &ConnectionForm,
    expand_selected_database_in_tree: bool,
) -> Result<Vec<SchemaObject>, String> {
    let opts = mysql::Opts::from_url(&mysql_url(form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let mut connection = pool
        .get_conn()
        .map_err(|error| format!("MySQL connection failed: {error}"))?;
    let selected_database = form.database.trim();

    if selected_database.is_empty() {
        return load_mysql_databases(&mut connection);
    }

    let rows: Vec<(String, String)> = connection
        .exec(
            "select table_name, table_type
             from information_schema.tables
             where table_schema = ?
             order by case table_type when 'BASE TABLE' then 0 when 'VIEW' then 1 else 2 end,
                      table_name
             limit 300",
            (selected_database,),
        )
        .map_err(|error| format!("MySQL schema query failed: {error}"))?;

    let mut selected_children = Vec::new();

    for (name, table_type) in rows {
        let kind = if table_type.eq_ignore_ascii_case("VIEW") {
            SchemaObjectKind::View
        } else {
            SchemaObjectKind::Table
        };
        let preview = format!(
            "select * from {}.{} limit 100;",
            quote_mysql_identifier(selected_database),
            quote_mysql_identifier(&name)
        );
        selected_children.push(SchemaObject::new(name.clone(), kind, preview));
    }

    if !expand_selected_database_in_tree {
        return Ok(selected_children);
    }

    Ok(expand_selected_database(
        load_mysql_databases(&mut connection)?,
        selected_database,
        selected_children,
    ))
}

fn load_mysql_schema_columns(
    form: &ConnectionForm,
    table_name: &str,
) -> Result<Vec<SchemaObject>, String> {
    let database = form.database.trim();
    if database.is_empty() {
        return Ok(Vec::new());
    }

    let opts = mysql::Opts::from_url(&mysql_url(form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let mut connection = pool
        .get_conn()
        .map_err(|error| format!("MySQL connection failed: {error}"))?;
    let rows: Vec<(String, String)> = connection
        .exec(
            "select column_name, column_type
             from information_schema.columns
             where table_schema = ?
               and table_name = ?
             order by ordinal_position
             limit 512",
            (database, table_name),
        )
        .map_err(|error| format!("MySQL column query failed: {error}"))?;

    Ok(rows
        .into_iter()
        .map(|(column_name, column_type)| {
            SchemaObject::new(
                column_label(&column_name, &column_type),
                SchemaObjectKind::Column,
                format!(
                    "select {} from {}.{} limit 100;",
                    quote_mysql_identifier(&column_name),
                    quote_mysql_identifier(database),
                    quote_mysql_identifier(table_name)
                ),
            )
        })
        .collect())
}

fn load_mysql_databases(connection: &mut mysql::PooledConn) -> Result<Vec<SchemaObject>, String> {
    let databases: Vec<String> = connection
        .query(
            "select schema_name
             from information_schema.schemata
             where schema_name not in ('information_schema', 'mysql', 'performance_schema', 'sys')
             order by schema_name
             limit 300",
        )
        .map_err(|error| format!("MySQL database listing failed: {error}"))?;

    Ok(databases
        .into_iter()
        .map(|database| {
            SchemaObject::new(
                database.clone(),
                SchemaObjectKind::Database,
                mysql_database_preview(&database),
            )
        })
        .collect())
}

fn load_postgres_schema(
    form: &ConnectionForm,
    expand_selected_database_in_tree: bool,
) -> Result<Vec<SchemaObject>, String> {
    let config = postgres_config(form)?;
    let mut client = config
        .connect(postgres::NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let selected_database = form.database.trim();

    if selected_database.is_empty() {
        return load_postgres_databases(&mut client);
    }

    let rows = client
        .query(
            "select table_schema, table_name, table_type
             from information_schema.tables
             where table_schema not in ('pg_catalog', 'information_schema')
             order by table_schema, table_name
             limit 300",
            &[],
        )
        .map_err(|error| format!("PostgreSQL schema query failed: {error}"))?;

    let mut tables = Vec::new();

    for row in rows {
        let schema: String = row.get(0);
        let name: String = row.get(1);
        let table_type: String = row.get(2);
        let qualified = format!(
            "{}.{}",
            quote_sql_identifier(&schema),
            quote_sql_identifier(&name)
        );
        let kind = if table_type.eq_ignore_ascii_case("VIEW") {
            SchemaObjectKind::View
        } else {
            SchemaObjectKind::Table
        };
        tables.push(SchemaObject::new(
            format!("{schema}.{name}"),
            kind,
            format!("select * from {qualified} limit 100;"),
        ));
    }

    let selected_children = tables;

    if !expand_selected_database_in_tree {
        return Ok(selected_children);
    }

    Ok(expand_selected_database(
        load_postgres_databases(&mut client)?,
        selected_database,
        selected_children,
    ))
}

fn load_postgres_schema_columns(
    form: &ConnectionForm,
    object_name: &str,
) -> Result<Vec<SchemaObject>, String> {
    let Some((schema, table)) = postgres_table_ref(object_name) else {
        return Ok(Vec::new());
    };

    let mut client = postgres_config(form)?
        .connect(postgres::NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let rows = client
        .query(
            "select column_name, data_type
             from information_schema.columns
             where table_schema = $1
               and table_name = $2
             order by ordinal_position
             limit 512",
            &[&schema, &table],
        )
        .map_err(|error| format!("PostgreSQL column query failed: {error}"))?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let column: String = row.get(0);
            let data_type: String = row.get(1);
            let qualified = format!(
                "{}.{}",
                quote_sql_identifier(&schema),
                quote_sql_identifier(&table)
            );

            SchemaObject::new(
                column_label(&column, &data_type),
                SchemaObjectKind::Column,
                format!(
                    "select {} from {qualified} limit 100;",
                    quote_sql_identifier(&column)
                ),
            )
        })
        .collect())
}

fn postgres_table_ref(object_name: &str) -> Option<(String, String)> {
    object_name
        .split_once('.')
        .map(|(schema, table)| (schema.trim().to_owned(), table.trim().to_owned()))
        .filter(|(schema, table)| !schema.is_empty() && !table.is_empty())
}

fn load_postgres_databases(client: &mut postgres::Client) -> Result<Vec<SchemaObject>, String> {
    let rows = client
        .query(
            "select datname
             from pg_database
             where datistemplate = false
             order by datname
             limit 300",
            &[],
        )
        .map_err(|error| format!("PostgreSQL database listing failed: {error}"))?;

    Ok(rows
        .into_iter()
        .map(|row| {
            let database: String = row.get(0);
            SchemaObject::new(
                database,
                SchemaObjectKind::Database,
                postgres_database_preview(),
            )
        })
        .collect())
}

fn load_mongodb_schema(
    form: &ConnectionForm,
    expand_selected_database_in_tree: bool,
) -> Result<Vec<SchemaObject>, String> {
    let options = mongodb::options::ClientOptions::parse(mongodb_url(form))
        .map_err(|error| format!("MongoDB connection options failed: {error}"))?;
    let client = MongoClient::with_options(options)
        .map_err(|error| format!("MongoDB client creation failed: {error}"))?;
    let selected_database = form.database.trim();

    if selected_database.is_empty() {
        return load_mongodb_databases(&client);
    }

    let database = client.database(selected_database);
    let names = database
        .list_collection_names(None)
        .map_err(|error| format!("MongoDB collection listing failed: {error}"))?;
    let selected_children = names
        .into_iter()
        .map(|name| {
            SchemaObject::new(
                name.clone(),
                SchemaObjectKind::Collection,
                format!(
                    "{{ \"find\": \"{}\", \"filter\": {{}}, \"limit\": 100 }}",
                    json_escape(&name)
                ),
            )
        })
        .collect();

    if !expand_selected_database_in_tree {
        return Ok(selected_children);
    }

    Ok(expand_selected_database(
        load_mongodb_databases(&client)?,
        selected_database,
        selected_children,
    ))
}

fn load_mongodb_databases(client: &MongoClient) -> Result<Vec<SchemaObject>, String> {
    let names = client
        .list_database_names(None, None)
        .map_err(|error| format!("MongoDB database listing failed: {error}"))?;

    Ok(names
        .into_iter()
        .map(|name| {
            SchemaObject::new(
                name,
                SchemaObjectKind::Database,
                String::from("{ \"listCollections\": 1 }"),
            )
        })
        .collect())
}

fn expand_selected_database(
    databases: Vec<SchemaObject>,
    selected_database: &str,
    selected_children: Vec<SchemaObject>,
) -> Vec<SchemaObject> {
    let mut objects = Vec::new();
    let mut selected_found = false;

    for database in databases {
        let is_selected = database.name == selected_database;
        selected_found |= is_selected;
        objects.push(database);

        if is_selected {
            objects.extend(selected_children.iter().cloned());
        }
    }

    if !selected_found {
        objects.push(SchemaObject::new(
            selected_database.to_owned(),
            SchemaObjectKind::Database,
            String::new(),
        ));
        objects.extend(selected_children);
    }

    objects
}
