use super::*;

pub(super) fn detail_section(
    kind: DatabaseDetailSectionKind,
    fields: Vec<DatabaseDetailField>,
) -> DatabaseDetailSection {
    DatabaseDetailSection { kind, fields }
}

pub(super) fn detail_field(
    label: impl Into<String>,
    value: impl Into<String>,
) -> DatabaseDetailField {
    DatabaseDetailField {
        label: label.into(),
        value: value.into(),
    }
}

pub(super) fn sqlite_pragma_string(
    connection: &rusqlite::Connection,
    sql: &str,
) -> Result<String, String> {
    connection
        .query_row(sql, [], |row| row.get::<_, String>(0))
        .map_err(|error| format!("SQLite PRAGMA query failed: {error}"))
}

pub(super) fn sqlite_pragma_i64(
    connection: &rusqlite::Connection,
    sql: &str,
) -> Result<i64, String> {
    connection
        .query_row(sql, [], |row| row.get::<_, i64>(0))
        .map_err(|error| format!("SQLite PRAGMA query failed: {error}"))
}

pub(super) fn sqlite_count(connection: &rusqlite::Connection, sql: &str) -> Result<i64, String> {
    connection
        .query_row(sql, [], |row| row.get::<_, i64>(0))
        .map_err(|error| format!("SQLite object count query failed: {error}"))
}

pub(super) fn bson_value(document: &Document, key: &str) -> String {
    document
        .get(key)
        .map(bson_value_to_string)
        .unwrap_or_else(|| String::from("unknown"))
}

pub(super) fn bson_size(document: &Document, key: &str) -> String {
    document
        .get(key)
        .and_then(bson_number_as_u64)
        .map(format_bytes)
        .unwrap_or_else(|| bson_value(document, key))
}

pub(super) fn bson_number_as_u64(value: &Bson) -> Option<u64> {
    match value {
        Bson::Int32(value) => (*value).try_into().ok(),
        Bson::Int64(value) => (*value).try_into().ok(),
        Bson::Double(value) if *value >= 0.0 => Some(*value as u64),
        _ => None,
    }
}

pub(super) fn bson_document_value(document: &Document, key: &str) -> String {
    match document.get(key) {
        Some(Bson::Document(_)) => String::from("present"),
        Some(value) => bson_value_to_string(value),
        None => String::from("unknown"),
    }
}

pub(super) fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0usize;

    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{bytes} {}", UNITS[unit])
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

pub(super) fn split_postgres_table(table: &str) -> (String, String) {
    let mut parts = table.splitn(2, '.');
    let first = parts.next().unwrap_or("public");
    let second = parts.next();

    match second {
        Some(name) => (first.to_owned(), name.to_owned()),
        None => (String::from("public"), first.to_owned()),
    }
}

pub(super) fn postgres_create_table_statement(
    client: &mut postgres::Client,
    schema: &str,
    table: &str,
) -> Result<String, String> {
    let rows = client
        .query(
            "select column_name, data_type, is_nullable, column_default
             from information_schema.columns
             where table_schema = $1 and table_name = $2
             order by ordinal_position",
            &[&schema, &table],
        )
        .map_err(|error| format!("PostgreSQL CREATE TABLE reconstruction failed: {error}"))?;
    let columns = rows
        .into_iter()
        .map(|row| {
            let name: String = row.get(0);
            let data_type: String = row.get(1);
            let nullable: String = row.get(2);
            let default_value: Option<String> = row.get(3);
            let mut line = format!("    {} {}", quote_sql_identifier(&name), data_type);

            if let Some(default_value) = default_value.filter(|value| !value.trim().is_empty()) {
                line.push_str(format!(" DEFAULT {default_value}").as_str());
            }
            if nullable == "NO" {
                line.push_str(" NOT NULL");
            }

            line
        })
        .collect::<Vec<_>>();

    Ok(format!(
        "CREATE TABLE {}.{} (\n{}\n);",
        quote_sql_identifier(schema),
        quote_sql_identifier(table),
        columns.join(",\n")
    ))
}
