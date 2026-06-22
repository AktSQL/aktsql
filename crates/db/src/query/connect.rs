use super::*;
use std::time::Duration;

pub(super) fn mysql_url(form: &ConnectionForm) -> String {
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

pub(super) fn mongodb_url(form: &ConnectionForm) -> String {
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

pub(super) fn postgres_config(form: &ConnectionForm) -> Result<postgres::Config, String> {
    let mut config = postgres::Config::new();
    config
        .host(form.location.trim())
        .port(parse_port(form)?)
        .user(form.username.trim())
        .password(form.password.as_str())
        .dbname(database_name(form).as_str())
        .connect_timeout(timeout(form));

    Ok(config)
}

pub(super) fn parse_port(form: &ConnectionForm) -> Result<u16, String> {
    form.port
        .trim()
        .parse::<u16>()
        .map_err(|error| format!("Invalid port: {error}"))
}

pub(super) fn timeout(form: &ConnectionForm) -> Duration {
    let seconds = form
        .timeout_seconds
        .trim()
        .parse::<u64>()
        .ok()
        .filter(|seconds| *seconds > 0)
        .unwrap_or(30);

    Duration::from_secs(seconds)
}

pub(super) fn database_name(form: &ConnectionForm) -> String {
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

pub(super) fn encode_url_part(value: &str) -> String {
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
