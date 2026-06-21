use super::*;
use std::fs;

impl Akt {
    pub(in crate::app) fn persist_connections(&self, success: &str) -> String {
        match persistence::save_connection_profiles(self.connection_manager.profiles()) {
            Ok(()) => String::from(success),
            Err(error) => error,
        }
    }

    pub(in crate::app) fn next_connection_task_token(&mut self) -> u64 {
        self.connection_task_token = self.connection_task_token.wrapping_add(1);
        self.connection_task_token
    }

    pub(in crate::app) fn cancel_connection_activity(&mut self) {
        self.connection_testing = false;
        self.connection_connecting = false;
        self.test_result_open = false;
        self.connection_task_token = self.connection_task_token.wrapping_add(1);
    }

    pub(in crate::app) fn save_and_connect_current(&mut self) -> Task<Message> {
        if self.connection_connecting {
            self.status_message = String::from("Connection is already in progress.");
            return Task::none();
        }

        let profile_id = match self.connection_manager.save_current() {
            Ok(id) => id,
            Err(errors) => {
                self.status_message = format!(
                    "Connection profile has {} validation issue(s).",
                    errors.len()
                );
                return Task::none();
            }
        };

        if let Err(error) =
            persistence::save_connection_profiles(self.connection_manager.profiles())
        {
            self.status_message = error;
            return Task::none();
        }

        self.connection_manager.select_profile(profile_id);
        self.start_connection_task()
    }

    pub(in crate::app) fn connect_current_profile(&mut self) -> Task<Message> {
        if self.connection_connecting {
            self.status_message = String::from("Connection is already in progress.");
            return Task::none();
        }

        self.start_connection_task()
    }

    pub(in crate::app) fn start_connection_task(&mut self) -> Task<Message> {
        let form = match self.connection_manager.current_form_for_test() {
            Ok(form) => form,
            Err(errors) => {
                self.status_message = format!(
                    "Connection profile has {} validation issue(s).",
                    errors.len()
                );
                return Task::none();
            }
        };

        let target = if form.driver.requires_port() {
            format!("{}:{}", form.location.trim(), form.port.trim())
        } else {
            form.location.trim().to_owned()
        };

        let token = self.next_connection_task_token();
        self.connection_connecting = true;
        self.status_message = format!("Connecting {} at {target}...", form.driver);

        Task::perform(run_connection_test_task(form), move |outcome| {
            Message::ConnectionConnectFinished(token, outcome)
        })
    }

    pub(in crate::app) fn export_last_result_csv(&self) -> String {
        let Some(result) = self.query_workspace.last_result() else {
            return String::from("No result set is available for CSV export.");
        };

        if result.columns.is_empty() {
            return String::from("Current result has no tabular rows to export.");
        }

        let path = "/tmp/aktsql-last-result.csv";
        let mut csv = String::new();
        csv.push_str(&csv_row(&result.columns));

        for row in &result.rows {
            csv.push_str(&csv_row(row));
        }

        match fs::write(path, csv) {
            Ok(()) => format!("CSV exported to {path}."),
            Err(error) => format!("CSV export failed: {error}"),
        }
    }

    pub(in crate::app) fn set_query_sql(&mut self, sql: &str) {
        self.query_editor = text_editor::Content::with_text(sql);
        self.query_workspace.set_sql(sql.to_owned());
        self.query_result_order_by.clear();
        self.table_rows_page = None;
    }

    pub(in crate::app) fn execute_query(&mut self) -> Task<Message> {
        if self.query_running {
            self.status_message = String::from("Query is already running.");
            return Task::none();
        }

        if self.connection_manager.selected_profile_id().is_none() {
            self.status_message =
                String::from("No active connection. Connect a saved profile before querying.");
            return Task::none();
        }

        let form = self.connection_manager.form().clone();
        let sql = match ordered_query_sql(
            form.driver,
            self.query_workspace.sql(),
            &self.query_result_order_by,
        ) {
            Ok(sql) => sql,
            Err(error) => {
                self.status_message = error;
                return Task::none();
            }
        };

        self.query_running = true;
        self.status_message = String::from("Executing query...");

        Task::perform(
            run_execute_sql_task(form, sql),
            Message::QueryExecutionFinished,
        )
    }

    pub(in crate::app) fn sort_result_by_column(&mut self, column_index: usize) -> Task<Message> {
        if self.query_running {
            self.status_message = String::from("Query is already running.");
            return Task::none();
        }

        let Some(result) = self.query_workspace.last_result() else {
            self.status_message = String::from("No result set is available for sorting.");
            return Task::none();
        };
        let Some(column_name) = result.columns.get(column_index).cloned() else {
            self.status_message = String::from("Sort column was not found in the result set.");
            return Task::none();
        };

        if let Some(page) = self.table_rows_page.as_mut() {
            toggle_order_key(&mut page.order_by, column_index, column_name);
            return self.load_table_rows_page(0);
        }

        toggle_order_key(&mut self.query_result_order_by, column_index, column_name);
        self.selected = Section::QueryExplorer;
        self.result_focus = true;
        self.execute_query()
    }

    pub(in crate::app) fn refresh_query_schema(&mut self) -> Task<Message> {
        if self.schema_loading {
            self.status_message = String::from("Schema refresh is already running.");
            return Task::none();
        }

        if self.connection_manager.selected_profile_id().is_none() {
            self.status_message = String::from(
                "No active connection. Connect a saved profile before refreshing schema.",
            );
            return Task::none();
        }

        let form = self.connection_manager.form().clone();
        let expand_selected_database_in_tree = self.database_expanded_from_tree;

        self.schema_loading = true;
        self.status_message = String::from("Refreshing schema...");

        Task::perform(
            run_schema_load_task(form, expand_selected_database_in_tree),
            Message::QuerySchemaRefreshed,
        )
    }

    pub(in crate::app) fn refresh_database_details(&mut self, database: String) -> Task<Message> {
        if self.connection_manager.selected_profile_id().is_none() {
            self.status_message = String::from(
                "No active connection. Connect a saved profile before inspecting metadata.",
            );
            return Task::none();
        }

        let mut form = self.connection_manager.form().clone();
        if form.driver != DatabaseDriver::Sqlite {
            form.database = database.clone();
        }

        self.selected = Section::Databases;
        self.database_workspace_active = true;
        self.clear_table_inspector_context();
        self.database_detail_target = Some(database.clone());
        self.database_detail_loading = true;
        self.database_details = None;
        self.status_message = format!("Loading database details for {database}...");

        Task::perform(
            run_database_detail_task(form, database.clone()),
            move |outcome| Message::DatabaseDetailsLoaded(database.clone(), outcome),
        )
    }

    pub(in crate::app) fn refresh_table_details(&mut self, table: String) -> Task<Message> {
        if self.connection_manager.selected_profile_id().is_none() {
            self.status_message = String::from(
                "No active connection. Connect a saved profile before inspecting table metadata.",
            );
            return Task::none();
        }

        let form = self.connection_manager.form().clone();
        self.selected = Section::Tables;
        self.clear_database_inspector_context();
        self.table_detail_target = Some(table.clone());
        self.table_detail_loading = true;
        self.table_details = None;
        self.status_message = format!("Loading table details for {table}...");

        Task::perform(run_table_detail_task(form, table.clone()), move |outcome| {
            Message::TableDetailsLoaded(table.clone(), outcome)
        })
    }

    pub(in crate::app) fn clear_database_inspector_context(&mut self) {
        self.database_detail_target = None;
        self.database_detail_loading = false;
        self.database_details = None;
    }

    pub(in crate::app) fn clear_table_inspector_context(&mut self) {
        self.table_detail_target = None;
        self.table_detail_loading = false;
        self.table_details = None;
        self.alter_table_draft = None;
        self.alter_table_tab = AlterTableTab::Columns;
    }

    pub(in crate::app) fn load_table_rows_page(&mut self, delta: i32) -> Task<Message> {
        if self.query_running {
            self.status_message = String::from("Query is already running.");
            return Task::none();
        }

        let Some(current) = self.table_rows_page.clone() else {
            self.status_message = String::from("No table row browser is active.");
            return Task::none();
        };
        let page = if delta.is_negative() {
            current.page.saturating_sub(delta.unsigned_abs() as usize)
        } else {
            current.page.saturating_add(delta as usize)
        };
        let mut form = self.connection_manager.form().clone();
        let sql = match select_rows_statement(
            &mut form,
            &current.table,
            page,
            current.page_size,
            &current.order_by,
        ) {
            Ok(sql) => sql,
            Err(error) => {
                self.status_message = error;
                return Task::none();
            }
        };

        self.selected = Section::QueryExplorer;
        self.result_focus = true;
        self.query_running = true;
        self.status_message = format!(
            "Loading rows {}-{} from {}...",
            page.saturating_mul(current.page_size).saturating_add(1),
            page.saturating_add(1).saturating_mul(current.page_size),
            current.table
        );

        let table = current.table.clone();
        Task::perform(run_execute_sql_task(form, sql), move |outcome| {
            Message::TableRowsLoaded(table.clone(), page, outcome)
        })
    }
}
