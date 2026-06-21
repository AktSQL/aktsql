use super::*;
use rusqlite::types::ValueRef;

pub(super) fn parse_mongodb_commands(command: &str) -> Result<Vec<Document>, String> {
    let value = serde_json::from_str::<serde_json::Value>(command).map_err(|error| {
        format!("MongoDB query must be a JSON command document or array: {error}")
    })?;
    let bson = bson::to_bson(&value)
        .map_err(|error| format!("MongoDB JSON conversion failed: {error}"))?;

    match bson {
        Bson::Document(document) => Ok(vec![document]),
        Bson::Array(values) => values
            .into_iter()
            .map(|value| match value {
                Bson::Document(document) => Ok(document),
                _ => Err(String::from(
                    "MongoDB command arrays must contain JSON objects only.",
                )),
            })
            .collect(),
        _ => Err(String::from(
            "MongoDB query must be a JSON object or object array.",
        )),
    }
}

pub(super) fn split_sql_statements(sql: &str) -> Vec<String> {
    sql.split(';')
        .map(str::trim)
        .filter(|statement| !statement.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

pub(super) fn mongodb_document_result(document: Document, elapsed_ms: u128) -> QueryResult {
    if let Some(Bson::Document(cursor)) = document.get("cursor") {
        if let Some(Bson::Array(batch)) = cursor.get("firstBatch") {
            return mongodb_batch_result(batch, elapsed_ms);
        }
    }

    let columns = document.keys().cloned().collect::<Vec<_>>();
    let rows = vec![document
        .values()
        .map(bson_value_to_string)
        .collect::<Vec<_>>()];

    tabular_result("MongoDB", columns, rows, false, elapsed_ms)
}

pub(super) fn mongodb_batch_result(batch: &[Bson], elapsed_ms: u128) -> QueryResult {
    let mut columns = Vec::<String>::new();
    let mut rows = Vec::new();
    let mut truncated = false;

    for value in batch {
        if rows.len() == MAX_RESULT_ROWS {
            truncated = true;
            break;
        }

        if let Bson::Document(document) = value {
            for key in document.keys() {
                if !columns.contains(key) {
                    columns.push(key.clone());
                }
            }
        }
    }

    for value in batch.iter().take(MAX_RESULT_ROWS) {
        if let Bson::Document(document) = value {
            rows.push(
                columns
                    .iter()
                    .map(|column| {
                        document
                            .get(column)
                            .map(bson_value_to_string)
                            .unwrap_or_else(|| String::from("NULL"))
                    })
                    .collect(),
            );
        }
    }

    tabular_result("MongoDB", columns, rows, truncated, elapsed_ms)
}

pub(super) fn statement_result(
    driver: &str,
    rows_affected: usize,
    elapsed_ms: u128,
) -> QueryResult {
    QueryResult {
        columns: Vec::new(),
        rows: Vec::new(),
        rows_affected: Some(rows_affected),
        elapsed_ms,
        message: format!("{driver} statement executed. {rows_affected} row(s) affected."),
        truncated: false,
    }
}

pub(super) fn tabular_result(
    driver: &str,
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
    truncated: bool,
    elapsed_ms: u128,
) -> QueryResult {
    let message = if truncated {
        format!(
            "{driver} query returned {} row(s) in {elapsed_ms} ms; display capped at {MAX_RESULT_ROWS}.",
            rows.len()
        )
    } else {
        format!(
            "{driver} query returned {} row(s) in {elapsed_ms} ms.",
            rows.len()
        )
    };

    QueryResult {
        columns,
        rows,
        rows_affected: None,
        elapsed_ms,
        message,
        truncated,
    }
}

pub(super) fn quote_sql_identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

pub(super) fn quote_mysql_identifier(value: &str) -> String {
    format!("`{}`", value.replace('`', "``"))
}

pub(super) fn mysql_database_preview(database: &str) -> String {
    format!(
        "select table_name, table_type\nfrom information_schema.tables\nwhere table_schema = {}\norder by table_name\nlimit 300;",
        sql_string_literal(database)
    )
}

pub(super) fn postgres_database_preview() -> String {
    String::from(
        "select table_schema, table_name, table_type\nfrom information_schema.tables\nwhere table_schema not in ('pg_catalog', 'information_schema')\norder by table_schema, table_name\nlimit 300;",
    )
}

pub(super) fn sql_string_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

pub(super) fn column_label(name: &str, data_type: &str) -> String {
    let data_type = data_type.trim();

    if data_type.trim().is_empty() {
        name.to_owned()
    } else {
        format!("{name} : {data_type}")
    }
}

pub(super) fn mysql_value_to_string(value: &mysql::Value) -> String {
    match value {
        mysql::Value::NULL => String::from("NULL"),
        mysql::Value::Bytes(value) => String::from_utf8_lossy(value).into_owned(),
        mysql::Value::Int(value) => value.to_string(),
        mysql::Value::UInt(value) => value.to_string(),
        mysql::Value::Float(value) => value.to_string(),
        mysql::Value::Double(value) => value.to_string(),
        mysql::Value::Date(year, month, day, hour, minute, second, micros) => {
            format!("{year:04}-{month:02}-{day:02} {hour:02}:{minute:02}:{second:02}.{micros:06}")
        }
        mysql::Value::Time(negative, days, hours, minutes, seconds, micros) => format!(
            "{}{days} {hours:02}:{minutes:02}:{seconds:02}.{micros:06}",
            if *negative { "-" } else { "" }
        ),
    }
}

pub(super) fn bson_value_to_string(value: &Bson) -> String {
    match value {
        Bson::Null => String::from("NULL"),
        Bson::String(value) => value.clone(),
        Bson::ObjectId(value) => value.to_hex(),
        _ => value.clone().into_relaxed_extjson().to_string(),
    }
}

pub(super) fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub(super) fn value_to_string(
    value: Result<ValueRef<'_>, rusqlite::Error>,
) -> Result<String, String> {
    let value = value.map_err(|error| format!("SQLite value read failed: {error}"))?;

    Ok(match value {
        ValueRef::Null => String::from("NULL"),
        ValueRef::Integer(value) => value.to_string(),
        ValueRef::Real(value) => value.to_string(),
        ValueRef::Text(value) => String::from_utf8_lossy(value).into_owned(),
        ValueRef::Blob(value) => format!("<blob: {} bytes>", value.len()),
    })
}
