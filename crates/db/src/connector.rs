use engine::{ConnectionForm, ConnectionTestReport, DatabaseDriver};
use mongodb::bson::doc;
use mongodb::sync::Client as MongoClient;
use mysql::prelude::Queryable;
use postgres::NoTls;
use std::time::{Duration, Instant};

pub fn test_connection(form: ConnectionForm) -> Result<ConnectionTestReport, Vec<String>> {
    form.validate()?;

    let start = Instant::now();
    let target = connection_target(&form);
    let probe = match form.driver {
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
        connect_latency_ms: probe.connect_latency_ms,
        roundtrip_latency_ms: probe.roundtrip_latency_ms,
        metadata_latency_ms: probe.metadata_latency_ms,
        server_version: probe.server_version,
        encoding: probe.encoding,
    })
}

#[derive(Debug, Clone)]
struct ConnectionProbe {
    connect_latency_ms: u128,
    roundtrip_latency_ms: u128,
    metadata_latency_ms: Option<u128>,
    server_version: Option<String>,
    encoding: Option<String>,
}

fn test_sqlite(form: &ConnectionForm) -> Result<ConnectionProbe, String> {
    let connect_start = Instant::now();
    let connection = rusqlite::Connection::open(form.location.trim())
        .map_err(|error| format!("SQLite connection failed: {error}"))?;
    let connect_latency_ms = connect_start.elapsed().as_millis();

    let roundtrip_start = Instant::now();
    let value: i64 = connection
        .query_row("SELECT 1", [], |row| row.get(0))
        .map_err(|error| format!("SQLite ping query failed: {error}"))?;
    if value != 1 {
        return Err(String::from(
            "SQLite ping query returned an unexpected value.",
        ));
    }
    let roundtrip_latency_ms = roundtrip_start.elapsed().as_millis();

    let metadata_start = Instant::now();
    connection
        .execute_batch("PRAGMA schema_version;")
        .map_err(|error| format!("SQLite metadata query failed: {error}"))?;

    Ok(ConnectionProbe {
        connect_latency_ms,
        roundtrip_latency_ms,
        metadata_latency_ms: Some(metadata_start.elapsed().as_millis()),
        server_version: Some(String::from("SQLite")),
        encoding: Some(String::from("UTF-8")),
    })
}

fn test_mysql(form: &ConnectionForm) -> Result<ConnectionProbe, String> {
    let opts = mysql::Opts::from_url(&mysql_url(form))
        .map_err(|error| format!("MySQL connection options failed: {error}"))?;
    let pool =
        mysql::Pool::new(opts).map_err(|error| format!("MySQL pool creation failed: {error}"))?;
    let connect_start = Instant::now();
    let mut connection = pool
        .get_conn()
        .map_err(|error| mysql_connection_error_message(form, &error.to_string()))?;
    let connect_latency_ms = connect_start.elapsed().as_millis();

    let roundtrip_start = Instant::now();
    let value: Option<u8> = connection
        .query_first("select 1")
        .map_err(|error| format!("MySQL ping query failed: {error}"))?;
    let roundtrip_latency_ms = roundtrip_start.elapsed().as_millis();

    if value != Some(1) {
        return Err(String::from(
            "MySQL ping query returned an unexpected value.",
        ));
    }

    let metadata_start = Instant::now();
    let metadata: Option<(String, String, String)> = connection
        .query_first("select @@version, @@character_set_connection, @@collation_connection")
        .map_err(|error| format!("MySQL metadata query failed: {error}"))?;

    let (server_version, charset, collation) = metadata.unwrap_or_else(|| {
        (
            String::from("unknown"),
            String::from("unknown"),
            String::from("unknown"),
        )
    });

    Ok(ConnectionProbe {
        connect_latency_ms,
        roundtrip_latency_ms,
        metadata_latency_ms: Some(metadata_start.elapsed().as_millis()),
        server_version: Some(server_version),
        encoding: Some(format!("{charset}/{collation}")),
    })
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

fn test_postgres(form: &ConnectionForm) -> Result<ConnectionProbe, String> {
    let mut config = postgres::Config::new();
    config
        .host(form.location.trim())
        .port(parse_port(form)?)
        .user(form.username.trim())
        .password(form.password.as_str())
        .dbname(database_name(form).as_str())
        .connect_timeout(timeout(form));

    let connect_start = Instant::now();
    let mut client = config
        .connect(NoTls)
        .map_err(|error| format!("PostgreSQL connection failed: {error}"))?;
    let connect_latency_ms = connect_start.elapsed().as_millis();

    let roundtrip_start = Instant::now();
    let row = client
        .query_one("select 1::int", &[])
        .map_err(|error| format!("PostgreSQL ping query failed: {error}"))?;
    let value: i32 = row.get(0);
    let roundtrip_latency_ms = roundtrip_start.elapsed().as_millis();

    if value != 1 {
        return Err(String::from(
            "PostgreSQL ping query returned an unexpected value.",
        ));
    }

    let metadata_start = Instant::now();
    let row = client
        .query_one(
            "select version(), current_setting('server_encoding'), current_schema()",
            &[],
        )
        .map_err(|error| format!("PostgreSQL metadata query failed: {error}"))?;
    let server_version: String = row.get(0);
    let server_encoding: String = row.get(1);
    let current_schema: String = row.get(2);

    Ok(ConnectionProbe {
        connect_latency_ms,
        roundtrip_latency_ms,
        metadata_latency_ms: Some(metadata_start.elapsed().as_millis()),
        server_version: Some(server_version),
        encoding: Some(format!("{server_encoding}/{current_schema}")),
    })
}

fn test_mongodb(form: &ConnectionForm) -> Result<ConnectionProbe, String> {
    let connect_start = Instant::now();
    let options = mongodb::options::ClientOptions::parse(mongodb_url(form))
        .map_err(|error| format!("MongoDB connection options failed: {error}"))?;
    let client = MongoClient::with_options(options)
        .map_err(|error| format!("MongoDB client creation failed: {error}"))?;
    let connect_latency_ms = connect_start.elapsed().as_millis();

    let roundtrip_start = Instant::now();
    client
        .database(database_name(form).as_str())
        .run_command(doc! { "ping": 1 }, None)
        .map_err(|error| format!("MongoDB ping failed: {error}"))?;
    let roundtrip_latency_ms = roundtrip_start.elapsed().as_millis();

    let metadata_start = Instant::now();
    let build_info = client
        .database("admin")
        .run_command(doc! { "buildInfo": 1 }, None)
        .map_err(|error| format!("MongoDB metadata query failed: {error}"))?;

    Ok(ConnectionProbe {
        connect_latency_ms,
        roundtrip_latency_ms,
        metadata_latency_ms: Some(metadata_start.elapsed().as_millis()),
        server_version: build_info.get_str("version").ok().map(str::to_owned),
        encoding: Some(String::from("BSON")),
    })
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
