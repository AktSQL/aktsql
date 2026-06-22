use engine::{ConnectionForm, DatabaseDriver};
use mongodb::bson::{self, Bson, Document};
use mongodb::sync::Client as MongoClient;
use mysql::prelude::Queryable;

mod connect;
mod details;
mod executor;
mod result;
mod schema;
use connect::*;
use details::*;
use engine::*;
use executor::execute_query;
use result::*;
use schema::{load_schema, load_schema_columns};

const MAX_RESULT_ROWS: usize = 200;

pub fn execute_sql(form: ConnectionForm, sql: String) -> QueryExecutionOutcome {
    match execute_query(&form, &sql) {
        Ok(result) => QueryExecutionOutcome::Success(result),
        Err(messages) => QueryExecutionOutcome::Failure(messages),
    }
}

#[cfg(test)]
pub fn load_schema_objects(form: ConnectionForm) -> SchemaLoadOutcome {
    load_schema_objects_with_scope(form, false)
}

pub fn load_schema_objects_with_scope(
    form: ConnectionForm,
    expand_selected_database_in_tree: bool,
) -> SchemaLoadOutcome {
    match load_schema(&form, expand_selected_database_in_tree) {
        Ok(objects) => SchemaLoadOutcome::Success(objects),
        Err(messages) => SchemaLoadOutcome::Failure(messages),
    }
}

pub fn load_schema_object_columns(
    form: ConnectionForm,
    object_name: String,
) -> Result<Vec<SchemaObject>, Vec<String>> {
    load_schema_columns(&form, &object_name)
}

pub fn load_database_details(
    form: ConnectionForm,
    database: String,
) -> Result<DatabaseDetails, Vec<String>> {
    let database = database.trim().to_owned();
    if database.is_empty() && form.driver != DatabaseDriver::Sqlite {
        return Err(vec![String::from("Database name is required.")]);
    }

    form.validate()?;

    let result = match form.driver {
        DatabaseDriver::Sqlite => load_sqlite_database_details(&form, &database),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            load_mysql_database_details(&form, &database)
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            load_postgres_database_details(&form, &database)
        }
        DatabaseDriver::MongoDb => load_mongodb_database_details(&form, &database),
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => {
            Ok(fallback_database_details(&form, &database))
        }
    };

    result.map_err(|error| vec![error])
}

pub fn load_table_details(
    form: ConnectionForm,
    table: String,
) -> Result<TableDetails, Vec<String>> {
    let table = table.trim().to_owned();
    if table.is_empty() {
        return Err(vec![String::from("Table name is required.")]);
    }

    form.validate()?;

    let result = match form.driver {
        DatabaseDriver::Sqlite => load_sqlite_table_details(&form, &table),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            load_mysql_table_details(&form, &table)
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            load_postgres_table_details(&form, &table)
        }
        DatabaseDriver::MongoDb => load_mongodb_table_details(&form, &table),
        DatabaseDriver::SqlServer | DatabaseDriver::Oracle => {
            Ok(fallback_table_details(&form, &table))
        }
    };

    result.map_err(|error| vec![error])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mongodb_query_requires_json_object() {
        assert!(parse_mongodb_commands("{ \"ping\": 1 }").is_ok());
        assert!(parse_mongodb_commands("[{ \"ping\": 1 }, { \"buildInfo\": 1 }]").is_ok());
        assert!(parse_mongodb_commands("[1, 2, 3]").is_err());
    }

    #[test]
    fn database_schema_object_uses_database_kind() {
        let object = SchemaObject::new(
            String::from("myapp_db"),
            SchemaObjectKind::Database,
            mysql_database_preview("myapp_db"),
        );

        assert_eq!(object.kind, SchemaObjectKind::Database);
        assert_eq!(object.display_label(), "DB myapp_db");
    }

    #[test]
    fn sqlite_schema_loads_database_and_tables_without_columns() {
        let path = "/tmp/aktsql-schema-column-test.sqlite";
        let _ = std::fs::remove_file(path);
        let connection = rusqlite::Connection::open(path).expect("sqlite file should open");
        connection
            .execute_batch("create table users (id integer primary key, name text not null);")
            .expect("fixture table should be created");

        let mut form = ConnectionForm::for_driver(DatabaseDriver::Sqlite);
        form.location = String::from(path);

        let outcome = load_schema_objects(form);
        let SchemaLoadOutcome::Success(objects) = outcome else {
            panic!("sqlite schema should load");
        };

        assert!(objects
            .iter()
            .any(|object| object.kind == SchemaObjectKind::Database));
        assert!(objects
            .iter()
            .any(|object| object.kind == SchemaObjectKind::Table && object.name == "users"));
        assert!(!objects
            .iter()
            .any(|object| object.kind == SchemaObjectKind::Column));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn sqlite_schema_columns_load_lazily_for_one_table() {
        let path = "/tmp/aktsql-schema-column-lazy-test.sqlite";
        let _ = std::fs::remove_file(path);
        let connection = rusqlite::Connection::open(path).expect("sqlite file should open");
        connection
            .execute_batch("create table users (id integer primary key, name text not null);")
            .expect("fixture table should be created");

        let mut form = ConnectionForm::for_driver(DatabaseDriver::Sqlite);
        form.location = String::from(path);

        let columns =
            load_schema_object_columns(form, String::from("users")).expect("columns should load");
        assert!(
            columns
                .iter()
                .any(|object| object.kind == SchemaObjectKind::Column
                    && object.name == "id : INTEGER")
        );

        let _ = std::fs::remove_file(path);
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mysql_query_succeeds() {
        let outcome = execute_sql(
            live_form(DatabaseDriver::MySql, "3306", "root", "root123", "myapp_db"),
            String::from("select 1 as ok;"),
        );

        assert!(matches!(outcome, QueryExecutionOutcome::Success(_)));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_postgres_query_succeeds() {
        let outcome = execute_sql(
            live_form(
                DatabaseDriver::PostgreSql,
                "5432",
                "postgres",
                "postgres123",
                "myapp_db",
            ),
            String::from("select 1 as ok;"),
        );

        assert!(matches!(outcome, QueryExecutionOutcome::Success(_)));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mongodb_command_succeeds() {
        let outcome = execute_sql(
            live_form(
                DatabaseDriver::MongoDb,
                "27017",
                "admin",
                "admin123",
                "myapp_db",
            ),
            String::from("{ \"ping\": 1 }"),
        );

        assert!(matches!(outcome, QueryExecutionOutcome::Success(_)));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mysql_schema_without_database_lists_databases() {
        let mut form = live_form(DatabaseDriver::MySql, "3306", "root", "root123", "");
        form.database.clear();

        let outcome = load_schema_objects(form);
        let SchemaLoadOutcome::Success(objects) = outcome else {
            panic!("mysql schema should load");
        };

        assert!(objects
            .iter()
            .any(|object| object.kind == SchemaObjectKind::Database));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_postgres_schema_without_database_lists_databases() {
        let mut form = live_form(
            DatabaseDriver::PostgreSql,
            "5432",
            "postgres",
            "postgres123",
            "",
        );
        form.database.clear();

        let outcome = load_schema_objects(form);
        let SchemaLoadOutcome::Success(objects) = outcome else {
            panic!("postgres schema should load");
        };

        assert!(objects
            .iter()
            .any(|object| object.kind == SchemaObjectKind::Database));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mongodb_schema_without_database_lists_databases() {
        let mut form = live_form(DatabaseDriver::MongoDb, "27017", "admin", "admin123", "");
        form.database.clear();

        let outcome = load_schema_objects(form);
        let SchemaLoadOutcome::Success(objects) = outcome else {
            panic!("mongodb schema should load");
        };

        assert!(objects
            .iter()
            .any(|object| object.kind == SchemaObjectKind::Database));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mysql_database_details_loads_metadata() {
        let details = load_database_details(
            live_form(DatabaseDriver::MySql, "3306", "root", "root123", "myapp_db"),
            String::from("myapp_db"),
        )
        .expect("mysql database details should load");

        assert_eq!(details.database, "myapp_db");
        assert!(details
            .sections
            .iter()
            .any(|section| section.kind == DatabaseDetailSectionKind::Storage));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_postgres_database_details_loads_metadata() {
        let details = load_database_details(
            live_form(
                DatabaseDriver::PostgreSql,
                "5432",
                "postgres",
                "postgres123",
                "myapp_db",
            ),
            String::from("myapp_db"),
        )
        .expect("postgres database details should load");

        assert_eq!(details.database, "myapp_db");
        assert!(details
            .sections
            .iter()
            .any(|section| section.kind == DatabaseDetailSectionKind::Runtime));
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mongodb_database_details_loads_metadata() {
        let details = load_database_details(
            live_form(
                DatabaseDriver::MongoDb,
                "27017",
                "admin",
                "admin123",
                "myapp_db",
            ),
            String::from("myapp_db"),
        )
        .expect("mongodb database details should load");

        assert_eq!(details.database, "myapp_db");
        assert!(details
            .sections
            .iter()
            .any(|section| section.kind == DatabaseDetailSectionKind::Objects));
    }

    fn live_form(
        driver: DatabaseDriver,
        port: &str,
        username: &str,
        password: &str,
        database: &str,
    ) -> ConnectionForm {
        let mut form = ConnectionForm::for_driver(driver);
        form.name = format!("Live {driver}");
        form.location = String::from("127.0.0.1");
        form.port = String::from(port);
        form.username = String::from(username);
        form.password = String::from(password);
        form.database = String::from(database);
        form.timeout_seconds = String::from("5");
        form
    }
}
