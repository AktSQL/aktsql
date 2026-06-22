use db::{
    execute_sql, load_database_details, load_schema_object_columns, load_schema_objects_with_scope,
};
use db::{test_connection, ConnectionTestReport};
use engine::ConnectionForm;
use engine::{
    DatabaseDetails, QueryExecutionOutcome, SchemaLoadOutcome, SchemaObject, TableDetails,
};

pub(super) async fn run_connection_test_task(
    form: ConnectionForm,
) -> Result<ConnectionTestReport, Vec<String>> {
    match tokio::task::spawn_blocking(move || test_connection(form)).await {
        Ok(outcome) => outcome,
        Err(error) => Err(vec![format!("Connection task failed: {error}")]),
    }
}

pub(super) async fn run_execute_sql_task(
    form: ConnectionForm,
    sql: String,
) -> QueryExecutionOutcome {
    match tokio::task::spawn_blocking(move || execute_sql(form, sql)).await {
        Ok(outcome) => outcome,
        Err(error) => {
            QueryExecutionOutcome::Failure(vec![format!("Database task failed: {error}")])
        }
    }
}

pub(super) async fn run_schema_load_task(
    form: ConnectionForm,
    expand_selected_database_in_tree: bool,
) -> SchemaLoadOutcome {
    match tokio::task::spawn_blocking(move || {
        load_schema_objects_with_scope(form, expand_selected_database_in_tree)
    })
    .await
    {
        Ok(outcome) => outcome,
        Err(error) => SchemaLoadOutcome::Failure(vec![format!("Schema task failed: {error}")]),
    }
}

pub(super) async fn run_schema_columns_load_task(
    form: ConnectionForm,
    object_name: String,
) -> Result<Vec<SchemaObject>, Vec<String>> {
    match tokio::task::spawn_blocking(move || load_schema_object_columns(form, object_name)).await {
        Ok(outcome) => outcome,
        Err(error) => Err(vec![format!("Schema column task failed: {error}")]),
    }
}

pub(super) async fn run_database_detail_task(
    form: ConnectionForm,
    database: String,
) -> Result<DatabaseDetails, Vec<String>> {
    match tokio::task::spawn_blocking(move || load_database_details(form, database)).await {
        Ok(outcome) => outcome,
        Err(error) => Err(vec![format!("Database detail task failed: {error}")]),
    }
}

pub(super) async fn run_table_detail_task(
    form: ConnectionForm,
    table: String,
) -> Result<TableDetails, Vec<String>> {
    match tokio::task::spawn_blocking(move || db::load_table_details(form, table)).await {
        Ok(outcome) => outcome,
        Err(error) => Err(vec![format!("Table detail task failed: {error}")]),
    }
}
