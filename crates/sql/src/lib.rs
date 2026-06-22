use engine::*;

mod alter_table;
mod create_table;
mod database;
mod dialect;
mod query;
pub use alter_table::move_column_statement;
pub use alter_table::{alter_table_statement, last_table_part};
pub use create_table::create_table_statement;
pub use database::*;
use dialect::*;
pub use query::ordered_query_sql;
pub use query::select_rows_statement;

pub fn table_column_definition_tail(column: &TableColumnDetail) -> String {
    let mut parts = Vec::new();

    if column.nullable.eq_ignore_ascii_case("NO") {
        parts.push(String::from("NOT NULL"));
    }

    let default_value = column.default_value.trim();
    if !default_value.is_empty() && !default_value.eq_ignore_ascii_case("NULL") {
        if default_value.to_ascii_uppercase().starts_with("DEFAULT ") {
            parts.push(default_value.to_owned());
        } else {
            parts.push(format!(
                "DEFAULT {}",
                normalized_column_default(column, default_value)
            ));
        }
    }

    let extra = sanitized_column_extra(&column.extra);
    if !extra.is_empty() {
        parts.push(extra);
    }

    parts.join(" ")
}

pub(crate) fn normalized_column_default(column: &TableColumnDetail, default_value: &str) -> String {
    let value = default_value.trim();
    if value.is_empty()
        || value.starts_with('\'')
        || value.starts_with('"')
        || value.starts_with('(')
        || value.parse::<f64>().is_ok()
        || is_sql_default_expression(value)
        || !column_type_uses_quoted_default(&column.data_type)
    {
        value.to_owned()
    } else {
        sql_string_literal(value)
    }
}

pub(crate) fn is_sql_default_expression(value: &str) -> bool {
    let upper = value.to_ascii_uppercase();
    matches!(
        upper.as_str(),
        "CURRENT_TIMESTAMP"
            | "CURRENT_DATE"
            | "CURRENT_TIME"
            | "LOCALTIME"
            | "LOCALTIMESTAMP"
            | "TRUE"
            | "FALSE"
    ) || upper.ends_with("()")
        || upper.contains("::")
}

pub(crate) fn column_type_uses_quoted_default(data_type: &str) -> bool {
    let lower = data_type.to_ascii_lowercase();
    [
        "char", "text", "enum", "set", "json", "date", "time", "uuid", "inet", "cidr", "macaddr",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
}

pub(crate) fn sanitized_column_extra(extra: &str) -> String {
    extra
        .split_whitespace()
        .filter(|token| !token.eq_ignore_ascii_case("DEFAULT_GENERATED"))
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn mysql_index_columns_sql(
    columns: &str,
    column_types: Vec<(&str, &str)>,
    index_type: &str,
) -> String {
    columns
        .split(',')
        .map(str::trim)
        .filter(|column| !column.is_empty())
        .map(|column| mysql_index_column_sql(column, &column_types, index_type))
        .collect::<Vec<_>>()
        .join(", ")
}

fn mysql_index_column_sql(column: &str, column_types: &[(&str, &str)], index_type: &str) -> String {
    if column.contains('(') || column.split_whitespace().count() > 1 {
        return column.to_owned();
    }

    let name = unquote_identifier(column);
    if name.is_empty() {
        return column.to_owned();
    }

    let quoted = quote_mysql_identifier(&name);
    if mysql_index_column_needs_prefix(&name, column_types, index_type) {
        format!("{quoted}(191)")
    } else {
        quoted
    }
}

fn mysql_index_column_needs_prefix(
    column: &str,
    column_types: &[(&str, &str)],
    index_type: &str,
) -> bool {
    let upper_index_type = index_type.to_ascii_uppercase();
    if upper_index_type.contains("FULLTEXT") || upper_index_type.contains("SPATIAL") {
        return false;
    }

    column_types
        .iter()
        .find(|(name, _)| name.trim().eq_ignore_ascii_case(column))
        .map(|(_, data_type)| mysql_type_requires_index_prefix(data_type))
        .unwrap_or(false)
}

fn mysql_type_requires_index_prefix(data_type: &str) -> bool {
    let lower = data_type.to_ascii_lowercase();
    lower.contains("text") || lower.contains("blob")
}

fn unquote_identifier(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .trim_matches('"')
        .trim_matches('[')
        .trim_matches(']')
        .to_owned()
}
