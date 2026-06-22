pub(super) fn mysql_charset_or_default(value: &str) -> &str {
    if value.is_empty() {
        "utf8mb4"
    } else {
        value
    }
}

pub(super) fn mysql_collation_or_default(value: &str) -> &str {
    if value.is_empty() {
        "utf8mb4_unicode_ci"
    } else {
        value
    }
}

pub(super) fn postgres_encoding_or_default(value: &str) -> &str {
    if value.is_empty() {
        "UTF8"
    } else {
        value
    }
}

pub(super) fn sql_server_collation_or_default(value: &str) -> &str {
    if value.is_empty() {
        "Latin1_General_100_CI_AS_SC_UTF8"
    } else {
        value
    }
}

pub(super) fn quote_plain_identifier(value: &str) -> String {
    format!("\"{}\"", value.replace('"', "\"\""))
}

pub(super) fn quote_dotted_identifier(value: &str) -> String {
    value
        .split('.')
        .map(quote_plain_identifier)
        .collect::<Vec<_>>()
        .join(".")
}

pub(super) fn quote_mysql_identifier(value: &str) -> String {
    format!("`{}`", value.replace('`', "``"))
}

pub(super) fn quote_sql_server_identifier(value: &str) -> String {
    format!("[{}]", value.replace(']', "]]"))
}

pub(super) fn quote_sql_server_table(value: &str) -> String {
    value
        .split('.')
        .map(|part| format!("[{}]", part.replace(']', "]]")))
        .collect::<Vec<_>>()
        .join(".")
}

pub(super) fn sql_string_literal(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

pub(super) fn json_escape(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}
