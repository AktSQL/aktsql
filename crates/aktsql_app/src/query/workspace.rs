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
