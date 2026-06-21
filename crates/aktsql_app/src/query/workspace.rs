use super::*;

#[derive(Debug, Clone)]
pub struct QueryWorkspace {
    sql: String,
    last_result: Option<QueryResult>,
    messages: Vec<String>,
    schema_objects: Vec<SchemaObject>,
    schema_message: String,
}

impl Default for QueryWorkspace {
    fn default() -> Self {
        Self {
            sql: String::from(Self::DEFAULT_SQL),
            last_result: None,
            messages: vec![String::from("Ready to execute a query.")],
            schema_objects: Vec::new(),
            schema_message: String::from("Schema not loaded."),
        }
    }
}

impl QueryWorkspace {
    pub const DEFAULT_SQL: &'static str = "select 1 as ok;";

    pub fn sql(&self) -> &str {
        &self.sql
    }

    pub fn last_result(&self) -> Option<&QueryResult> {
        self.last_result.as_ref()
    }

    pub fn messages(&self) -> &[String] {
        &self.messages
    }

    pub fn schema_objects(&self) -> &[SchemaObject] {
        &self.schema_objects
    }

    pub fn schema_message(&self) -> &str {
        &self.schema_message
    }

    pub fn schema_object_has_children(&self, parent_index: usize) -> bool {
        let Some(parent) = self.schema_objects.get(parent_index) else {
            return false;
        };
        let Some(next) = self.schema_objects.get(parent_index + 1) else {
            return false;
        };

        next.depth() > parent.depth()
    }

    pub fn insert_schema_children(
        &mut self,
        parent_index: usize,
        children: Vec<SchemaObject>,
    ) -> usize {
        if children.is_empty()
            || parent_index >= self.schema_objects.len()
            || self.schema_object_has_children(parent_index)
        {
            return 0;
        }

        let inserted = children.len();
        self.schema_objects
            .splice(parent_index + 1..parent_index + 1, children);
        inserted
    }

    pub fn set_sql(&mut self, sql: String) {
        self.sql = sql;
    }

    pub fn example_sql(&self, form: &ConnectionForm) -> String {
        if let Some(object) = self.schema_objects.iter().find(|object| {
            matches!(
                object.kind,
                SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
            )
        }) {
            return object.sql_preview();
        }

        match form.driver {
            DatabaseDriver::Sqlite => String::from(
                "select type, name\nfrom sqlite_master\nwhere type in ('table', 'view')\norder by name\nlimit 100;",
            ),
            DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => String::from(
                "select table_name, table_type\nfrom information_schema.tables\nwhere table_schema = database()\norder by table_name\nlimit 100;",
            ),
            DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => String::from(
                "select table_schema, table_name, table_type\nfrom information_schema.tables\nwhere table_schema not in ('pg_catalog', 'information_schema')\norder by table_schema, table_name\nlimit 100;",
            ),
            DatabaseDriver::MongoDb => String::from("{ \"listCollections\": 1 }"),
            _ => String::from("select 1 as ok;"),
        }
    }

    pub fn format_sql(sql: &str) -> String {
        let formatted = sql
            .split_whitespace()
            .fold(String::new(), |mut acc, token| {
                let keyword = sql_keyword(token);

                if should_start_sql_line(token) && !acc.is_empty() {
                    acc.push('\n');
                } else if !acc.is_empty() && !acc.ends_with('\n') {
                    acc.push(' ');
                }

                acc.push_str(keyword.unwrap_or(token));
                acc
            });

        if formatted.is_empty() {
            String::new()
        } else if formatted.ends_with(';') {
            formatted
        } else {
            format!("{formatted};")
        }
    }

    pub fn set_sql_for_schema_object(&mut self, object: &SchemaObject) -> String {
        let sql = object.sql_preview();
        self.sql = sql.clone();
        self.messages = vec![format!("Prepared SQL for {}.", object.display_label())];
        sql
    }

    pub fn apply_execution_outcome(
        &mut self,
        outcome: QueryExecutionOutcome,
    ) -> QueryExecutionSummary {
        match outcome {
            QueryExecutionOutcome::Success(result) => {
                let summary = QueryExecutionSummary {
                    row_count: result.row_count(),
                    elapsed_ms: Some(result.elapsed_ms),
                    status_message: result.message.clone(),
                };
                self.messages = vec![result.message.clone()];
                self.last_result = Some(result);
                summary
            }
            QueryExecutionOutcome::Failure(messages) => {
                let status_message = format!("Query failed with {} issue(s).", messages.len());
                self.messages = messages;
                self.last_result = None;
                QueryExecutionSummary {
                    row_count: 0,
                    elapsed_ms: None,
                    status_message,
                }
            }
        }
    }

    pub fn apply_schema_outcome(&mut self, outcome: SchemaLoadOutcome) -> SchemaLoadSummary {
        match outcome {
            SchemaLoadOutcome::Success(objects) => {
                let object_count = objects.len();
                let status_message = format!("Loaded {object_count} schema object(s).");
                self.schema_objects = objects;
                self.schema_message = status_message.clone();

                SchemaLoadSummary { status_message }
            }
            SchemaLoadOutcome::Failure(messages) => {
                let status_message =
                    format!("Schema refresh failed with {} issue(s).", messages.len());
                self.schema_objects.clear();
                self.schema_message = messages.join(" ");
                self.messages = messages;

                SchemaLoadSummary { status_message }
            }
        }
    }
}

fn should_start_sql_line(token: &str) -> bool {
    matches!(
        token
            .trim_matches(|c: char| !c.is_ascii_alphanumeric())
            .to_ascii_lowercase()
            .as_str(),
        "from"
            | "where"
            | "group"
            | "having"
            | "order"
            | "limit"
            | "offset"
            | "join"
            | "left"
            | "right"
            | "inner"
            | "outer"
            | "union"
    )
}

fn sql_keyword(token: &str) -> Option<&'static str> {
    match token
        .trim_matches(|c: char| !c.is_ascii_alphanumeric())
        .to_ascii_lowercase()
        .as_str()
    {
        "select" => Some("select"),
        "from" => Some("from"),
        "where" => Some("where"),
        "and" => Some("and"),
        "or" => Some("or"),
        "group" => Some("group"),
        "by" => Some("by"),
        "having" => Some("having"),
        "order" => Some("order"),
        "limit" => Some("limit"),
        "offset" => Some("offset"),
        "join" => Some("join"),
        "left" => Some("left"),
        "right" => Some("right"),
        "inner" => Some("inner"),
        "outer" => Some("outer"),
        "on" => Some("on"),
        "insert" => Some("insert"),
        "update" => Some("update"),
        "delete" => Some("delete"),
        "values" => Some("values"),
        "set" => Some("set"),
        _ => None,
    }
}
