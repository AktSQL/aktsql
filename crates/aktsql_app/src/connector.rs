use crate::connection::{ConnectionForm, DatabaseDriver};
use mongodb::bson::doc;
use mongodb::sync::Client as MongoClient;
use mysql::prelude::Queryable;
use postgres::NoTls;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct ConnectionTestReport {
    pub driver: DatabaseDriver,
    pub target: String,
    pub elapsed_ms: u128,
}

pub fn test_connection(form: ConnectionForm) -> Result<ConnectionTestReport, Vec<String>> {
    form.validate()?;

    let start = Instant::now();
    let target = connection_target(&form);
    match form.driver {
        DatabaseDriver::Sqlite => test_sqlite(&form),
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            test_mysql(&form)
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => test_postgres(&form),
        DatabaseDriver::MongoDb => test_mongodb(&form),
        driver => Err(format!(
            "{driver} connectivity is not wired yet. Current live test support: MySQL, MariaDB, TiDB, PostgreSQL, CockroachDB, MongoDB, SQLite."
        )),
    }
    .map_err(|error| vec![error])?;

    Ok(ConnectionTestReport {
        driver: form.driver,
        target,
        elapsed_ms: start.elapsed().as_millis(),
    })
}

fn test_sqlite(form: &ConnectionForm) -> Result<String, String> {
    rusqlite::Connection::open(form.location.trim())
        .and_then(|connection| connection.execute_batch("PRAGMA schema_version;"))
        .map(|_| String::from("SQLite connection opened successfully."))
        .map_err(|error| format!("SQLite connection failed: {error}"))
}

fn test_mysql(form: &ConnectionForm) -> Result<String, String> {
    let opts = mysql::Opts::from_url(&mysql_url(form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let mut connection = pool
        .get_conn()
        .map_err(|error| mysql_connection_error_message(form, &error.to_string()))?;
    let value: Option<u8> = connection
        .query_first("select 1")
        .map_err(|error| format!("MySQL ping query failed: {error}"))?;

    match value {
        Some(1) => Ok(String::from("MySQL ping query returned 1.")),
        _ => Err(String::from(
            "MySQL ping query returned an unexpected value.",
        )),
    }
}

fn mysql_connection_error_message(form: &ConnectionForm, error: &str) -> String {
    if error.contains("1045") || error.contains("Access denied") {
        return format!(
            "MySQL authentication failed for user '{}'. Check username/password. For the bundled Docker service use root/root123 or myuser/mypassword123; placeholder text is not an entered password. Raw error: {error}",
            form.username.trim()
        );
    }

    format!("MySQL connection failed: {error}")
}

fn test_postgres(form: &ConnectionForm) -> Result<String, String> {
    let mut config = postgres::Config::new();
    config
        .host(form.location.trim())
        .port(parse_port(form)?)
        .user(form.username.trim())
        .password(form.password.as_str())
        .dbname(database_name(form).as_str())
        .connect_timeout(timeout(form));

    let mut client = config
        .connect(NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let row = client
        .query_one("select 1::int", &[])
        .map_err(|error| format!("PostgreSQL ping query failed: {error}"))?;
    let value: i32 = row.get(0);

    if value == 1 {
        Ok(String::from("PostgreSQL ping query returned 1."))
    } else {
        Err(String::from(
            "PostgreSQL ping query returned an unexpected value.",
        ))
    }
}

fn test_mongodb(form: &ConnectionForm) -> Result<String, String> {
    let options = mongodb::options::ClientOptions::parse(mongodb_url(form))
        .map_err(|error| format!("MongoDB connection options failed: {error}"))?;
    let client = MongoClient::with_options(options)
        .map_err(|error| format!("MongoDB client creation failed: {error}"))?;
    client
        .database(database_name(form).as_str())
        .run_command(doc! { "ping": 1 }, None)
        .map(|_| String::from("MongoDB ping command succeeded."))
        .map_err(|error| format!("MongoDB ping failed: {error}"))
}

fn mysql_url(form: &ConnectionForm) -> String {
    let database = database_name(form);
    format!(
        "mysql://{}:{}@{}:{}/{}",
        encode_url_part(form.username.trim()),
        encode_url_part(form.password.as_str()),
        form.location.trim(),
        form.port.trim(),
        encode_url_part(&database)
    )
}

fn mongodb_url(form: &ConnectionForm) -> String {
    let database = database_name(form);
    format!(
        "mongodb://{}:{}@{}:{}/{}?authSource=admin&serverSelectionTimeoutMS={}",
        encode_url_part(form.username.trim()),
        encode_url_part(form.password.as_str()),
        form.location.trim(),
        form.port.trim(),
        encode_url_part(&database),
        timeout(form).as_millis()
    )
}

fn parse_port(form: &ConnectionForm) -> Result<u16, String> {
    form.port
        .trim()
        .parse::<u16>()
        .map_err(|error| format!("Invalid port: {error}"))
}

fn timeout(form: &ConnectionForm) -> Duration {
    let seconds = form
        .timeout_seconds
        .trim()
        .parse::<u64>()
        .ok()
        .filter(|seconds| *seconds > 0)
        .unwrap_or(30);

    Duration::from_secs(seconds)
}

fn database_name(form: &ConnectionForm) -> String {
    let database = form.database.trim();

    if !database.is_empty() {
        return database.to_owned();
    }

    match form.driver {
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => String::from("postgres"),
        DatabaseDriver::MongoDb => String::from("admin"),
        _ => String::new(),
    }
}

fn connection_target(form: &ConnectionForm) -> String {
    if form.driver.requires_port() {
        format!("{}:{}", form.location.trim(), form.port.trim())
    } else {
        form.location.trim().to_owned()
    }
}

fn encode_url_part(value: &str) -> String {
    value
        .bytes()
        .flat_map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                vec![byte as char]
            }
            _ => format!("%{byte:02X}").chars().collect(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mysql_connection_succeeds() {
        test_connection(live_form(
            DatabaseDriver::MySql,
            "3306",
            "root",
            "root123",
            "myapp_db",
        ))
        .expect("mysql should be reachable");
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_postgres_connection_succeeds() {
        test_connection(live_form(
            DatabaseDriver::PostgreSql,
            "5432",
            "postgres",
            "postgres123",
            "myapp_db",
        ))
        .expect("postgres should be reachable");
    }

    #[test]
    #[ignore = "requires local Docker database services"]
    fn live_mongodb_connection_succeeds() {
        test_connection(live_form(
            DatabaseDriver::MongoDb,
            "27017",
            "admin",
            "admin123",
            "myapp_db",
        ))
        .expect("mongodb should be reachable");
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
