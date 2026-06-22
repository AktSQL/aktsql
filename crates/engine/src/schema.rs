use crate::connection::DatabaseDriver;

static MYSQL_INDEX_TYPES: [&str; 5] = ["BTREE", "UNIQUE BTREE", "FULLTEXT", "SPATIAL", "HASH"];
static POSTGRES_INDEX_TYPES: [&str; 7] = [
    "BTREE",
    "UNIQUE BTREE",
    "HASH",
    "GIN",
    "GIST",
    "SPGIST",
    "BRIN",
];
static SQLITE_INDEX_TYPES: [&str; 2] = ["INDEX", "UNIQUE"];
static SQL_SERVER_INDEX_TYPES: [&str; 5] = [
    "NONCLUSTERED",
    "UNIQUE NONCLUSTERED",
    "CLUSTERED",
    "UNIQUE CLUSTERED",
    "COLUMNSTORE",
];
static ORACLE_INDEX_TYPES: [&str; 3] = ["NORMAL", "UNIQUE", "BITMAP"];
static MONGODB_INDEX_TYPES: [&str; 5] = ["ASCENDING", "DESCENDING", "TEXT", "HASHED", "2DSPHERE"];
static SQL_CONSTRAINT_TYPES: [&str; 4] = ["PRIMARY KEY", "UNIQUE", "CHECK", "FOREIGN KEY"];
static SQLITE_CONSTRAINT_TYPES: [&str; 0] = [];
static MONGODB_CONSTRAINT_TYPES: [&str; 0] = [];

pub fn index_type_options(driver: DatabaseDriver) -> Vec<String> {
    index_type_slice(driver)
        .iter()
        .map(|value| String::from(*value))
        .collect()
}

pub fn default_index_type(driver: DatabaseDriver) -> &'static str {
    index_type_slice(driver).first().copied().unwrap_or("INDEX")
}

pub fn normalized_index_type(driver: DatabaseDriver, value: &str) -> String {
    index_type_options(driver)
        .into_iter()
        .find(|option| option.eq_ignore_ascii_case(value.trim()))
        .unwrap_or_else(|| String::from(default_index_type(driver)))
}

pub fn constraint_type_options(driver: DatabaseDriver) -> Vec<String> {
    constraint_type_slice(driver)
        .iter()
        .map(|value| String::from(*value))
        .collect()
}

pub fn default_constraint_type(driver: DatabaseDriver) -> &'static str {
    constraint_type_slice(driver)
        .first()
        .copied()
        .unwrap_or("CHECK")
}

pub fn normalized_constraint_type(driver: DatabaseDriver, value: &str) -> String {
    constraint_type_options(driver)
        .into_iter()
        .find(|option| option.eq_ignore_ascii_case(value.trim()))
        .unwrap_or_else(|| String::from(default_constraint_type(driver)))
}

pub fn mysql_index_type_sql(index_type: &str) -> (&'static str, &'static str) {
    match index_type.to_ascii_uppercase().as_str() {
        "UNIQUE BTREE" => ("UNIQUE ", " USING BTREE"),
        "FULLTEXT" => ("FULLTEXT ", ""),
        "SPATIAL" => ("SPATIAL ", ""),
        "HASH" => ("", " USING HASH"),
        _ => ("", " USING BTREE"),
    }
}

pub fn postgres_index_type_sql(index_type: &str) -> (&'static str, &'static str) {
    match index_type.to_ascii_uppercase().as_str() {
        "UNIQUE BTREE" => ("UNIQUE ", "btree"),
        "HASH" => ("", "hash"),
        "GIN" => ("", "gin"),
        "GIST" => ("", "gist"),
        "SPGIST" => ("", "spgist"),
        "BRIN" => ("", "brin"),
        _ => ("", "btree"),
    }
}

pub fn sqlite_index_type_sql(index_type: &str) -> &'static str {
    if index_type.eq_ignore_ascii_case("UNIQUE") {
        "UNIQUE "
    } else {
        ""
    }
}

pub fn sql_server_index_type_sql(index_type: &str) -> &'static str {
    match index_type.to_ascii_uppercase().as_str() {
        "UNIQUE NONCLUSTERED" => "UNIQUE NONCLUSTERED",
        "CLUSTERED" => "CLUSTERED",
        "UNIQUE CLUSTERED" => "UNIQUE CLUSTERED",
        "COLUMNSTORE" => "COLUMNSTORE",
        _ => "NONCLUSTERED",
    }
}

pub fn oracle_index_type_sql(index_type: &str) -> &'static str {
    match index_type.to_ascii_uppercase().as_str() {
        "UNIQUE" => "UNIQUE ",
        "BITMAP" => "BITMAP ",
        _ => "",
    }
}

pub fn mongodb_index_keys(
    columns: &str,
    index_type: &str,
    json_escape: fn(&str) -> String,
) -> String {
    let value = match index_type.to_ascii_uppercase().as_str() {
        "DESCENDING" => "-1",
        "TEXT" => "\"text\"",
        "HASHED" => "\"hashed\"",
        "2DSPHERE" => "\"2dsphere\"",
        _ => "1",
    };

    columns
        .split(',')
        .map(str::trim)
        .filter(|column| !column.is_empty())
        .map(|column| format!("\"{}\":{}", json_escape(column), value))
        .collect::<Vec<_>>()
        .join(",")
}

pub fn append_index_column(target: &mut String, column_name: &str) {
    let column_name = column_name.trim();
    if column_name.is_empty() {
        return;
    }

    let mut columns = target
        .split(',')
        .map(str::trim)
        .filter(|column| !column.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();

    if columns
        .iter()
        .any(|column| column.eq_ignore_ascii_case(column_name))
    {
        *target = columns.join(", ");
        return;
    }

    columns.push(column_name.to_owned());
    *target = columns.join(", ");
}

pub fn toggle_index_column(target: &mut String, column_name: &str) {
    let column_name = column_name.trim();
    if column_name.is_empty() {
        return;
    }

    let mut columns = target
        .split(',')
        .map(str::trim)
        .filter(|column| !column.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();

    if let Some(index) = columns
        .iter()
        .position(|column| column.eq_ignore_ascii_case(column_name))
    {
        columns.remove(index);
    } else {
        columns.push(column_name.to_owned());
    }

    *target = columns.join(", ");
}

fn index_type_slice(driver: DatabaseDriver) -> &'static [&'static str] {
    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            MYSQL_INDEX_TYPES.as_slice()
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => POSTGRES_INDEX_TYPES.as_slice(),
        DatabaseDriver::Sqlite => SQLITE_INDEX_TYPES.as_slice(),
        DatabaseDriver::SqlServer => SQL_SERVER_INDEX_TYPES.as_slice(),
        DatabaseDriver::Oracle => ORACLE_INDEX_TYPES.as_slice(),
        DatabaseDriver::MongoDb => MONGODB_INDEX_TYPES.as_slice(),
    }
}

fn constraint_type_slice(driver: DatabaseDriver) -> &'static [&'static str] {
    match driver {
        DatabaseDriver::Sqlite => SQLITE_CONSTRAINT_TYPES.as_slice(),
        DatabaseDriver::MongoDb => MONGODB_CONSTRAINT_TYPES.as_slice(),
        _ => SQL_CONSTRAINT_TYPES.as_slice(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_index_column_keeps_comma_list_unique() {
        let mut columns = String::from("name, created_at");

        append_index_column(&mut columns, "NAME");
        assert_eq!(columns, "name, created_at");

        append_index_column(&mut columns, "email");
        assert_eq!(columns, "name, created_at, email");
    }
}
