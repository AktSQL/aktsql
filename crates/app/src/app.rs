use crate::persistence;
use crate::system_metrics;
use engine::{
    i18n, ConnectionField, ConnectionManager, DatabaseDetails, DatabaseDriver, Language, Message,
    QueryExecutionOutcome, QueryWorkspace, SchemaObjectKind, TableColumnDetail, TableDetails,
    ThemeMode,
};
use iced::{keyboard, time, window, Subscription, Task};
use std::collections::{BTreeSet, HashSet};
use std::time::Duration;

mod accessors;
mod defaults;
mod runtime;
mod schema_edit;
mod tasks;
mod util;
pub use engine::{
    AlterDatabaseCharsetDraft, AlterTableDraft, AlterTableField, AlterTableOperation,
    AlterTableTab, CreateDatabaseDraft, CreateDatabaseField, CreateTableColumnDraft,
    CreateTableColumnField, CreateTableConstraintDraft, CreateTableConstraintField,
    CreateTableDraft, CreateTableField, CreateTableIndexDraft, CreateTableIndexField,
    CreateTableTab, PendingSchemaDeletion, RenameDatabaseDraft, RenameTableDraft,
    SchemaDeletionKind, TableRowsPage, UiFontPreference,
};
#[cfg(test)]
pub use engine::{ConnectionForm, ConnectionTestReport, ResultSortKey, SortDirection};
pub use engine::{
    DatabaseAction, DatabaseEditField, QuickAction, Section, TableAction, TableEditField,
};
use runtime::*;
use schema_edit::*;
use sql::*;
use util::*;

#[derive(Debug)]
pub struct Akt {
    selected: Section,
    theme: ThemeMode,
    language: Language,
    ui_font_preference: UiFontPreference,
    connection_manager: ConnectionManager,
    query_workspace: QueryWorkspace,
    result_row_count: usize,
    result_latency_ms: Option<u128>,
    query_running: bool,
    schema_loading: bool,
    schema_mutation_running: bool,
    connection_testing: bool,
    connection_connecting: bool,
    connection_task_token: u64,
    test_result_open: bool,
    pending_delete_profile_id: Option<usize>,
    pending_schema_deletion: Option<PendingSchemaDeletion>,
    result_focus: bool,
    database_workspace_active: bool,
    expanded_schema_objects: BTreeSet<usize>,
    loading_schema_object_columns: HashSet<String>,
    schema_object_menu: Option<usize>,
    database_expanded_from_tree: bool,
    create_database_draft: Option<CreateDatabaseDraft>,
    create_table_draft: Option<CreateTableDraft>,
    rename_database_draft: Option<RenameDatabaseDraft>,
    alter_database_charset_draft: Option<AlterDatabaseCharsetDraft>,
    rename_table_draft: Option<RenameTableDraft>,
    alter_table_draft: Option<AlterTableDraft>,
    selected_alter_table_column: Option<usize>,
    create_table_tab: CreateTableTab,
    alter_table_tab: AlterTableTab,
    table_rows_page: Option<TableRowsPage>,
    table_detail_target: Option<String>,
    table_detail_loading: bool,
    table_details: Option<TableDetails>,
    database_detail_target: Option<String>,
    database_detail_loading: bool,
    database_details: Option<DatabaseDetails>,
    memory_label: String,
    status_message: String,
}

impl Akt {
    fn persist_preferences(&mut self) {
        let preferences = persistence::AppPreferences {
            language: self.language.config_value().to_owned(),
            theme: self.theme.config_value().to_owned(),
            ui_font: self.ui_font_preference.config_value().to_owned(),
        };

        if let Err(error) = persistence::save_preferences(&preferences) {
            self.status_message = error;
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::on_key_press(handle_key_press),
            time::every(Duration::from_secs(1)).map(|_| Message::SystemMetricsTick),
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectSection(section) => {
                let section = if section == Section::QueryExplorer {
                    Section::Databases
                } else {
                    section
                };
                self.selected = section;
                if section == Section::Databases
                    && self.connection_manager.selected_profile_id().is_none()
                {
                    self.database_workspace_active = false;
                }
                self.status_message = format!("{} workspace selected.", section.label());

                if matches!(section, Section::Tables)
                    && self.query_workspace.schema_objects().is_empty()
                    && self.connection_manager.selected_profile_id().is_some()
                {
                    return self.refresh_query_schema();
                }
            }
            Message::ToggleTheme => {
                self.theme = self.theme.toggle();
                self.persist_preferences();
                self.status_message = format!("Theme switched to {}.", self.theme.label());
            }
            Message::LanguageSelected(language) => {
                self.language = language;
                self.persist_preferences();
                self.status_message = format!("Language switched to {}.", self.language.label());
            }
            Message::FontPreferenceSelected(font_preference) => {
                self.ui_font_preference = font_preference;
                self.persist_preferences();
                self.status_message = format!(
                    "UI font set to {}. Restart Akt to apply it as the window default.",
                    self.ui_font_preference.ui_font_name()
                );
            }
            Message::RunQuickAction(action) => {
                if action == QuickAction::NewConnection {
                    self.selected = Section::Databases;
                    self.database_workspace_active = false;
                    self.connection_manager.new_profile();
                } else if action == QuickAction::RefreshData
                    && self.selected == Section::QueryExplorer
                {
                    return self.refresh_query_schema();
                } else if action == QuickAction::RefreshData && self.selected == Section::Databases
                {
                    return self.refresh_query_schema();
                }

                self.status_message = String::from(action.status_message());
            }
            Message::ConnectionDriverSelected(driver) => {
                self.connection_manager.set_driver(driver);
                self.status_message = format!("{driver} defaults applied.");
            }
            Message::ConnectionFieldChanged(field, value) => {
                self.connection_manager.set_field(field, value);
                if field == ConnectionField::Database {
                    self.database_expanded_from_tree = false;
                }
            }
            Message::ConnectionSslToggled(enabled) => {
                self.connection_manager.set_ssl_enabled(enabled);
                self.status_message = if enabled {
                    String::from("SSL enabled for connection draft.")
                } else {
                    String::from("SSL disabled for connection draft.")
                };
            }
            Message::ConnectionSshToggled(enabled) => {
                self.connection_manager.set_ssh_tunnel_enabled(enabled);
                self.status_message = if enabled {
                    String::from("SSH tunnel enabled for connection draft.")
                } else {
                    String::from("SSH tunnel disabled for connection draft.")
                };
            }
            Message::NewConnectionProfile => {
                self.selected = Section::Databases;
                self.database_workspace_active = false;
                self.database_expanded_from_tree = false;
                self.connection_manager.new_profile();
                self.pending_delete_profile_id = None;
                self.cancel_connection_activity();
                self.status_message = String::from("New connection draft created.");
            }
            Message::ConnectionSearchChanged(query) => {
                self.connection_manager.set_search_query(query);
            }
            Message::ReloadConnectionProfiles => {
                let profiles = persistence::load_connection_profiles().unwrap_or_default();
                self.connection_manager = ConnectionManager::with_profiles(profiles);
                self.status_message = format!(
                    "Connection profiles refreshed: {} visible.",
                    self.connection_manager.visible_profiles().len()
                );
            }
            Message::ToggleConnectionFilter => {
                self.connection_manager.toggle_list_filter();
                self.status_message = format!(
                    "Connection filter: {}.",
                    self.connection_manager.list_filter_label()
                );
            }
            Message::RequestTestConnection => {
                if self.connection_testing {
                    self.status_message =
                        String::from(i18n::texts(self.language).test_connection_running);
                    return Task::none();
                }

                self.test_result_open = true;

                match self.connection_manager.current_form_for_test() {
                    Ok(form) => {
                        let token = self.next_connection_task_token();
                        self.connection_testing = true;
                        self.status_message =
                            String::from(i18n::texts(self.language).test_connection_running);

                        return Task::perform(run_connection_test_task(form), move |outcome| {
                            Message::ConnectionTestFinished(token, outcome)
                        });
                    }
                    Err(errors) => {
                        self.status_message = format!(
                            "Connection profile has {} validation issue(s).",
                            errors.len()
                        );
                    }
                }
            }
            Message::CloseTestResult => {
                self.test_result_open = false;
            }
            Message::ConnectionTestFinished(token, outcome) => {
                if token != self.connection_task_token {
                    return Task::none();
                }

                self.connection_testing = false;
                self.status_message = match outcome {
                    Ok(report) => format!(
                        "{} · {}",
                        i18n::texts(self.language).test_connection_succeeded,
                        report.status_summary()
                    ),
                    Err(_) => String::from(i18n::texts(self.language).test_connection_failed),
                };
            }
            Message::SaveConnectionProfile => {
                return self.save_and_connect_current();
            }
            Message::ConnectConnectionProfile => {
                return self.connect_current_profile();
            }
            Message::ConnectionConnectFinished(token, outcome) => {
                if token != self.connection_task_token {
                    return Task::none();
                }

                self.connection_connecting = false;
                match outcome {
                    Ok(report) => {
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        self.database_expanded_from_tree = false;
                        self.result_focus = false;
                        self.status_message = format!("Connected · {}", report.status_summary());
                        return self.refresh_query_schema();
                    }
                    Err(errors) => {
                        self.status_message = format!(
                            "Connection failed with {} issue(s): {}",
                            errors.len(),
                            errors.join(" ")
                        );
                    }
                }
            }
            Message::SelectConnectionProfile(id) => {
                if self.connection_manager.select_profile(id) {
                    self.database_workspace_active = false;
                    self.database_expanded_from_tree = false;
                    self.pending_delete_profile_id = None;
                    self.test_result_open = false;
                    self.cancel_connection_activity();
                    self.status_message = String::from("Connection profile loaded.");
                } else {
                    self.status_message = String::from("Connection profile was not found.");
                }
            }
            Message::RequestDeleteConnection(id) => {
                self.pending_delete_profile_id = Some(id);
                self.status_message =
                    String::from("Connection profile delete confirmation requested.");
            }
            Message::CancelDeleteConnection => {
                self.pending_delete_profile_id = None;
                self.status_message = String::from("Delete cancelled.");
            }
            Message::ConfirmDeleteConnection => {
                let Some(id) = self.pending_delete_profile_id.take() else {
                    self.status_message = String::from("No connection is pending deletion.");
                    return Task::none();
                };

                if self.connection_manager.selected_profile_id() == Some(id) {
                    self.database_workspace_active = false;
                }

                if self.connection_manager.delete_profile(id) {
                    self.status_message = self.persist_connections("Connection profile deleted.");
                } else {
                    self.status_message = String::from("Connection profile was not found.");
                }
            }
            Message::CancelSchemaDelete => {
                self.pending_schema_deletion = None;
                self.status_message = String::from("DROP cancelled.");
            }
            Message::ConfirmSchemaDelete => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let Some(pending) = self.pending_schema_deletion.clone() else {
                    self.status_message = String::from("No schema object is pending deletion.");
                    return Task::none();
                };

                self.schema_mutation_running = true;
                self.status_message = match pending.kind {
                    SchemaDeletionKind::Database => {
                        format!("Dropping database {}...", pending.name)
                    }
                    SchemaDeletionKind::Table => format!("Dropping table {}...", pending.name),
                };

                return Task::perform(
                    run_execute_sql_task(pending.form, pending.sql),
                    move |outcome| {
                        Message::SchemaDeleteFinished(pending.name.clone(), pending.kind, outcome)
                    },
                );
            }
            Message::SchemaDeleteFinished(name, kind, outcome) => {
                self.schema_mutation_running = false;

                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        self.pending_schema_deletion = None;
                        self.schema_object_menu = None;
                        self.schema_loading = false;
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);

                        if kind == SchemaDeletionKind::Database
                            && self.connection_manager.form().database == name
                        {
                            self.database_expanded_from_tree = false;
                            self.connection_manager
                                .set_field(ConnectionField::Database, String::new());
                        }

                        self.status_message = match kind {
                            SchemaDeletionKind::Database => {
                                format!("DROP DATABASE {name} succeeded.")
                            }
                            SchemaDeletionKind::Table => {
                                format!("DROP TABLE {name} succeeded.")
                            }
                        };
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("DROP {name} failed.")
                        } else {
                            format!("DROP {name} failed: {detail}")
                        };
                    }
                }
            }
            Message::RefreshQuerySchema => {
                if !(self.selected == Section::Databases && self.database_workspace_active) {
                    self.selected = Section::QueryExplorer;
                }
                return self.refresh_query_schema();
            }
            Message::QuerySchemaRefreshed(outcome) => {
                self.schema_loading = false;
                self.expanded_schema_objects.clear();
                self.loading_schema_object_columns.clear();
                self.schema_object_menu = None;
                let summary = self.query_workspace.apply_schema_outcome(outcome);

                self.status_message = summary.status_message;
            }
            Message::QuerySchemaObjectColumnsLoaded(object_name, outcome) => {
                self.loading_schema_object_columns.remove(&object_name);
                let Some(index) = self
                    .query_workspace
                    .schema_objects()
                    .iter()
                    .position(|object| {
                        object.name == object_name
                            && matches!(
                                object.kind,
                                SchemaObjectKind::Table | SchemaObjectKind::View
                            )
                    })
                else {
                    return Task::none();
                };

                if !self.expanded_schema_objects.contains(&index) {
                    return Task::none();
                }

                match outcome {
                    Ok(columns) => {
                        let column_count = columns.len();
                        let inserted = self.query_workspace.insert_schema_children(index, columns);
                        shift_schema_indices_after(
                            &mut self.expanded_schema_objects,
                            index,
                            inserted,
                        );
                        shift_schema_index_after(&mut self.schema_object_menu, index, inserted);
                        self.status_message = if inserted == 0 && column_count > 0 {
                            String::from("Fields are already loaded.")
                        } else {
                            format!("Loaded {column_count} field(s).")
                        };
                    }
                    Err(messages) => {
                        self.expanded_schema_objects.remove(&index);
                        self.status_message = if messages.is_empty() {
                            String::from("Field loading failed.")
                        } else {
                            format!("Field loading failed: {}", messages.join(" "))
                        };
                    }
                }
            }
            Message::DatabaseDetailsLoaded(database, outcome) => {
                self.database_detail_loading = false;
                match outcome {
                    Ok(details) => {
                        self.database_details = Some(details);
                        self.status_message = format!("Database details loaded for {database}.");
                    }
                    Err(messages) => {
                        self.database_details = None;
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Database detail load failed for {database}.")
                        } else {
                            format!("Database detail load failed for {database}: {detail}")
                        };
                    }
                }
            }
            Message::UseQuerySchemaObject(index) => {
                let Some(object) = self.query_workspace.schema_objects().get(index).cloned() else {
                    self.status_message = String::from("Schema object was not found.");
                    return Task::none();
                };

                if object.kind == SchemaObjectKind::Database
                    && self.connection_manager.form().driver != DatabaseDriver::Sqlite
                {
                    self.schema_object_menu = None;
                    if self.database_expanded_from_tree
                        && self.connection_manager.form().database == object.name
                    {
                        self.database_expanded_from_tree = false;
                        self.connection_manager
                            .set_field(ConnectionField::Database, String::new());
                        self.status_message = format!("Database collapsed: {}.", object.name);
                    } else {
                        self.database_expanded_from_tree = true;
                        self.connection_manager
                            .set_field(ConnectionField::Database, object.name.clone());
                        self.status_message = format!("Database expanded: {}.", object.name);
                    }
                    return self.refresh_query_schema();
                }

                self.schema_object_menu = None;
                self.status_message = format!("Selected {}.", object.display_label());
            }
            Message::ToggleSchemaObject(index) => {
                let Some(object) = self.query_workspace.schema_objects().get(index).cloned() else {
                    self.status_message = String::from("Schema object was not found.");
                    return Task::none();
                };

                if !matches!(
                    object.kind,
                    SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
                ) {
                    self.status_message = format!("{} cannot be expanded.", object.display_label());
                    return Task::none();
                }

                if !self.expanded_schema_objects.insert(index) {
                    self.expanded_schema_objects.remove(&index);
                    self.schema_object_menu = None;
                    self.status_message = format!("Collapsed {}.", object.display_label());
                } else {
                    self.schema_object_menu = None;
                    self.status_message = format!("Expanded {}.", object.display_label());
                    if matches!(
                        object.kind,
                        SchemaObjectKind::Table | SchemaObjectKind::View
                    ) && !self.query_workspace.schema_object_has_children(index)
                        && self
                            .loading_schema_object_columns
                            .insert(object.name.clone())
                    {
                        let form = self.connection_manager.form().clone();
                        let object_name = object.name.clone();
                        return Task::perform(
                            run_schema_columns_load_task(form, object_name.clone()),
                            move |outcome| {
                                Message::QuerySchemaObjectColumnsLoaded(
                                    object_name.clone(),
                                    outcome,
                                )
                            },
                        );
                    }
                }
            }
            Message::ToggleSchemaObjectMenu(index) => {
                let Some(object) = self.query_workspace.schema_objects().get(index) else {
                    self.status_message = String::from("Schema object was not found.");
                    return Task::none();
                };

                if matches!(
                    object.kind,
                    SchemaObjectKind::Database
                        | SchemaObjectKind::Table
                        | SchemaObjectKind::View
                        | SchemaObjectKind::Collection
                ) {
                    self.schema_object_menu = if self.schema_object_menu == Some(index) {
                        None
                    } else {
                        Some(index)
                    };
                    self.status_message = format!("Object actions opened for {}.", object.name);
                }
            }
            Message::RequestCreateTable => {
                let form = self.connection_manager.form();
                let driver = form.driver;
                let database = form.database.trim().to_owned();

                if database.is_empty() && driver != DatabaseDriver::Sqlite {
                    self.selected = Section::Tables;
                    self.status_message =
                        String::from("Select a database before creating a table.");
                    return Task::none();
                }

                self.create_table_tab = CreateTableTab::Columns;
                self.create_table_draft = Some(CreateTableDraft::for_driver(driver));
                self.selected = Section::Tables;
                self.result_focus = false;
                self.status_message = if database.is_empty() {
                    String::from("Create table form opened.")
                } else {
                    format!("Create table form opened for database {database}.")
                };
            }
            Message::RunDatabaseAction(action) => {
                self.schema_object_menu = None;
                let form = self.connection_manager.form();
                let driver = form.driver;
                let database = form.database.trim().to_owned();

                if action == DatabaseAction::CreateDatabase {
                    self.create_database_draft = Some(CreateDatabaseDraft::for_driver(driver));
                    self.selected = Section::Databases;
                    self.status_message = String::from("Create database form opened.");
                    return Task::none();
                }

                if action == DatabaseAction::CreateTable {
                    return self.update(Message::RequestCreateTable);
                }

                if action == DatabaseAction::DropDatabase {
                    match pending_database_deletion(
                        self.connection_manager.form().clone(),
                        &database,
                    ) {
                        Ok(pending) => {
                            self.pending_schema_deletion = Some(pending);
                            self.selected = Section::Databases;
                            self.database_workspace_active = true;
                            self.status_message =
                                format!("Delete database confirmation requested for {database}.");
                        }
                        Err(error) => self.status_message = error,
                    }
                    return Task::none();
                }

                if matches!(
                    action,
                    DatabaseAction::ShowDatabase
                        | DatabaseAction::AlterDatabaseName
                        | DatabaseAction::AlterDatabaseCharset
                        | DatabaseAction::DropDatabase
                ) && database.is_empty()
                    && driver != DatabaseDriver::Sqlite
                {
                    self.selected = Section::Databases;
                    self.status_message =
                        String::from("Select a database before running this database action.");
                    return Task::none();
                }

                if action == DatabaseAction::ShowDatabase {
                    return self
                        .refresh_database_details(database_action_target(driver, &database));
                }

                self.result_focus = false;
                self.status_message = match action {
                    DatabaseAction::CreateDatabase => String::from("Create database form opened."),
                    DatabaseAction::CreateTable => {
                        String::from(i18n::texts(self.language).create_table)
                    }
                    DatabaseAction::ShowDatabase => {
                        self.database_detail_target =
                            Some(database_action_target(driver, &database));
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        format!(
                            "Database details shown for {}.",
                            database_action_target(driver, &database)
                        )
                    }
                    DatabaseAction::AlterDatabaseName => {
                        self.rename_database_draft = Some(RenameDatabaseDraft::new(
                            database_action_target(driver, &database),
                        ));
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        format!(
                            "Rename database form opened for {}.",
                            database_action_target(driver, &database)
                        )
                    }
                    DatabaseAction::AlterDatabaseCharset => {
                        self.alter_database_charset_draft =
                            Some(AlterDatabaseCharsetDraft::for_form(
                                self.connection_manager.form(),
                                database_action_target(driver, &database),
                            ));
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        format!(
                            "Alter database charset form opened for {}.",
                            database_action_target(driver, &database)
                        )
                    }
                    DatabaseAction::DropDatabase => format!(
                        "Delete database confirmation opened for {}.",
                        database_action_target(driver, &database)
                    ),
                };
            }
            Message::RunDatabaseObjectAction(action, index) => {
                self.schema_object_menu = None;
                let Some(object) = self.query_workspace.schema_objects().get(index).cloned() else {
                    self.status_message = String::from("Schema object was not found.");
                    return Task::none();
                };

                if object.kind != SchemaObjectKind::Database {
                    self.status_message = format!(
                        "{} does not support database actions.",
                        object.display_label()
                    );
                    return Task::none();
                }

                self.connection_manager
                    .set_field(ConnectionField::Database, object.name.clone());
                self.database_expanded_from_tree = true;

                match action {
                    DatabaseAction::CreateTable => {
                        return self.update(Message::RequestCreateTable);
                    }
                    DatabaseAction::ShowDatabase => {
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        self.clear_table_inspector_context();
                        return self.refresh_database_details(object.name.clone());
                    }
                    DatabaseAction::AlterDatabaseName => {
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        self.rename_database_draft =
                            Some(RenameDatabaseDraft::new(object.name.clone()));
                        self.status_message =
                            format!("Rename database form opened for {}.", object.name);
                    }
                    DatabaseAction::AlterDatabaseCharset | DatabaseAction::DropDatabase => {
                        if action == DatabaseAction::DropDatabase {
                            match pending_database_deletion(
                                self.connection_manager.form().clone(),
                                &object.name,
                            ) {
                                Ok(pending) => {
                                    self.pending_schema_deletion = Some(pending);
                                    self.selected = Section::Databases;
                                    self.database_workspace_active = true;
                                    self.status_message = format!(
                                        "Delete database confirmation requested for {}.",
                                        object.name
                                    );
                                }
                                Err(error) => self.status_message = error,
                            }
                            return Task::none();
                        }

                        self.alter_database_charset_draft =
                            Some(AlterDatabaseCharsetDraft::for_form(
                                self.connection_manager.form(),
                                object.name.clone(),
                            ));
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        self.status_message =
                            format!("Alter database charset form opened for {}.", object.name);
                    }
                    DatabaseAction::CreateDatabase => {
                        return self.update(Message::RunDatabaseAction(action));
                    }
                }
            }
            Message::DatabaseEditFieldChanged(field, value) => {
                let driver = self.connection_manager.form().driver;
                match field {
                    DatabaseEditField::NewName => {
                        let Some(draft) = self.rename_database_draft.as_mut() else {
                            self.status_message = String::from("Rename database form is not open.");
                            return Task::none();
                        };
                        draft.new_name = value;
                    }
                    DatabaseEditField::Charset => {
                        let Some(draft) = self.alter_database_charset_draft.as_mut() else {
                            self.status_message =
                                String::from("Alter database charset form is not open.");
                            return Task::none();
                        };
                        draft.charset = value;
                        draft.collation =
                            String::from(driver.default_collation_for_charset(&draft.charset));
                    }
                    DatabaseEditField::Collation => {
                        let Some(draft) = self.alter_database_charset_draft.as_mut() else {
                            self.status_message =
                                String::from("Alter database charset form is not open.");
                            return Task::none();
                        };
                        draft.collation = value;
                    }
                }
            }
            Message::CancelDatabaseEdit => {
                self.rename_database_draft = None;
                self.alter_database_charset_draft = None;
                self.status_message = String::from("Database edit cancelled.");
            }
            Message::SubmitRenameDatabase => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let Some(draft) = self.rename_database_draft.as_ref() else {
                    self.status_message = String::from("Rename database form is not open.");
                    return Task::none();
                };

                let old_name = draft.database().trim().to_owned();
                let new_name = draft.new_name().trim().to_owned();
                if new_name.is_empty() {
                    self.status_message = String::from("New database name is required.");
                    return Task::none();
                }
                if old_name == new_name {
                    self.status_message = String::from("New database name must be different.");
                    return Task::none();
                }

                let form = self.connection_manager.form().clone();
                let child_objects = self
                    .query_workspace
                    .schema_objects()
                    .iter()
                    .filter(|object| {
                        matches!(
                            object.kind,
                            SchemaObjectKind::Table
                                | SchemaObjectKind::View
                                | SchemaObjectKind::Collection
                        )
                    })
                    .map(|object| object.name.clone())
                    .collect::<Vec<_>>();
                let mut form = form;
                let sql = match rename_database_statement(
                    &mut form,
                    &old_name,
                    &new_name,
                    &child_objects,
                    self.connection_manager.form().charset.as_str(),
                    self.connection_manager.form().collation.as_str(),
                ) {
                    Ok(sql) => sql,
                    Err(error) => {
                        self.status_message = error;
                        return Task::none();
                    }
                };

                self.schema_mutation_running = true;
                self.selected = Section::Databases;
                self.database_workspace_active = true;
                self.status_message = format!("Renaming database {old_name} to {new_name}...");
                return Task::perform(run_execute_sql_task(form, sql), move |outcome| {
                    Message::RenameDatabaseFinished(old_name.clone(), new_name.clone(), outcome)
                });
            }
            Message::RenameDatabaseFinished(old_name, new_name, outcome) => {
                self.schema_mutation_running = false;
                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        self.rename_database_draft = None;
                        self.connection_manager
                            .set_field(ConnectionField::Database, new_name.clone());
                        if self.database_detail_target.as_deref() == Some(old_name.as_str()) {
                            self.database_detail_target = Some(new_name.clone());
                        }
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);
                        self.status_message = format!("Database {old_name} renamed to {new_name}.");
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Rename database {old_name} failed.")
                        } else {
                            format!("Rename database {old_name} failed: {detail}")
                        };
                    }
                }
            }
            Message::SubmitAlterDatabaseCharset => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let Some(draft) = self.alter_database_charset_draft.as_ref() else {
                    self.status_message = String::from("Alter database charset form is not open.");
                    return Task::none();
                };
                let database = draft.database().trim().to_owned();
                if database.is_empty() {
                    self.status_message = String::from("Database name is required.");
                    return Task::none();
                }

                let mut form = self.connection_manager.form().clone();
                let sql = match alter_database_charset_statement(
                    form.driver,
                    &database,
                    draft.charset(),
                    draft.collation(),
                ) {
                    Ok(sql) => sql,
                    Err(error) => {
                        self.status_message = error;
                        return Task::none();
                    }
                };
                form.charset = draft.charset().to_owned();
                form.collation = draft.collation().to_owned();

                self.schema_mutation_running = true;
                self.selected = Section::Databases;
                self.database_workspace_active = true;
                self.status_message = format!("Altering database charset for {database}...");
                return Task::perform(run_execute_sql_task(form, sql), move |outcome| {
                    Message::AlterDatabaseCharsetFinished(database.clone(), outcome)
                });
            }
            Message::AlterDatabaseCharsetFinished(database, outcome) => {
                self.schema_mutation_running = false;
                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        if let Some(draft) = self.alter_database_charset_draft.take() {
                            self.connection_manager
                                .set_field(ConnectionField::Charset, draft.charset);
                            self.connection_manager
                                .set_field(ConnectionField::Collation, draft.collation);
                        }
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);
                        self.status_message = format!("Database charset altered for {database}.");
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Alter database charset for {database} failed.")
                        } else {
                            format!("Alter database charset for {database} failed: {detail}")
                        };
                    }
                }
            }
            Message::CreateDatabaseFieldChanged(field, value) => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.create_database_draft.as_mut() else {
                    self.status_message = String::from("Create database form is not open.");
                    return Task::none();
                };

                match field {
                    CreateDatabaseField::Name => draft.name = value,
                    CreateDatabaseField::Charset => {
                        draft.charset = value;
                        draft.collation =
                            String::from(driver.default_collation_for_charset(&draft.charset));
                    }
                    CreateDatabaseField::Collation => draft.collation = value,
                    CreateDatabaseField::Owner => draft.owner = value,
                    CreateDatabaseField::Template => draft.template = value,
                    CreateDatabaseField::Tablespace => draft.tablespace = value,
                    CreateDatabaseField::InitialCollection => draft.initial_collection = value,
                }
            }
            Message::CancelCreateDatabase => {
                self.create_database_draft = None;
                self.status_message = String::from("Create database cancelled.");
            }
            Message::SubmitCreateDatabase => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.create_database_draft.as_ref() else {
                    self.status_message = String::from("Create database form is not open.");
                    return Task::none();
                };
                let name = draft.name.trim().to_owned();

                if name.is_empty() {
                    self.status_message = String::from("Database name is required.");
                    return Task::none();
                }

                if driver == DatabaseDriver::MongoDb && draft.initial_collection.trim().is_empty() {
                    self.status_message = String::from(
                        "MongoDB needs an initial collection to materialize a new database.",
                    );
                    return Task::none();
                }

                if self.connection_manager.selected_profile_id().is_none() {
                    self.status_message = String::from(
                        "No active connection. Connect a saved profile before creating a database.",
                    );
                    return Task::none();
                }

                let form = self.connection_manager.form().clone();
                let command = match create_database_command(form, draft) {
                    Ok(command) => command,
                    Err(error) => {
                        self.status_message = error;
                        return Task::none();
                    }
                };

                self.schema_mutation_running = true;
                self.selected = Section::Databases;
                self.result_focus = false;
                self.status_message = format!("Creating database {name}...");

                let created_database = name.clone();
                return Task::perform(
                    run_execute_sql_task(command.form, command.sql),
                    move |outcome| {
                        Message::CreateDatabaseFinished(created_database.clone(), outcome)
                    },
                );
            }
            Message::CreateDatabaseFinished(name, outcome) => {
                self.schema_mutation_running = false;

                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        self.create_database_draft = None;
                        self.connection_manager
                            .set_field(ConnectionField::Database, name.clone());
                        self.database_expanded_from_tree = false;
                        self.selected = Section::Databases;
                        self.database_workspace_active = true;
                        self.schema_loading = false;
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);
                        self.status_message =
                            format!("Database {name} created in {} ms.", result.elapsed_ms);
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Create database {name} failed.")
                        } else {
                            format!("Create database {name} failed: {detail}")
                        };
                    }
                }
            }
            Message::CreateTableFieldChanged(field, value) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };

                match field {
                    CreateTableField::Name => draft.name = value,
                    CreateTableField::Engine => draft.engine = value,
                    CreateTableField::Charset => {
                        let driver = self.connection_manager.form().driver;
                        draft.charset = value;
                        draft.collation =
                            String::from(driver.default_collation_for_charset(&draft.charset));
                    }
                    CreateTableField::Collation => draft.collation = value,
                    CreateTableField::Comment => draft.comment = value,
                }
            }
            Message::CreateTableColumnFieldChanged(index, field, value) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let Some(column) = draft.columns.get_mut(index) else {
                    self.status_message = String::from("Column row was not found.");
                    return Task::none();
                };

                match field {
                    CreateTableColumnField::Name => column.name = value,
                    CreateTableColumnField::DataType => column.data_type = value,
                    CreateTableColumnField::Nullable => column.nullable = value,
                    CreateTableColumnField::DefaultValue => column.default_value = value,
                    CreateTableColumnField::Extra => column.extra = value,
                }
            }
            Message::CreateTableIndexFieldChanged(index, field, value) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let Some(index_draft) = draft.indexes.get_mut(index) else {
                    self.status_message = String::from("Index row was not found.");
                    return Task::none();
                };

                match field {
                    CreateTableIndexField::Name => index_draft.name = value,
                    CreateTableIndexField::Columns => index_draft.columns = value,
                    CreateTableIndexField::IndexType => index_draft.index_type = value,
                }
            }
            Message::CreateTableConstraintFieldChanged(index, field, value) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let Some(constraint) = draft.constraints.get_mut(index) else {
                    self.status_message = String::from("Constraint row was not found.");
                    return Task::none();
                };

                match field {
                    CreateTableConstraintField::Name => constraint.name = value,
                    CreateTableConstraintField::Kind => constraint.kind = value,
                    CreateTableConstraintField::Expression => constraint.expression = value,
                }
            }
            Message::CreateTableTabSelected(tab) => {
                self.create_table_tab = tab;
            }
            Message::AddCreateTableColumn => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                insert_create_table_column(draft, draft.columns.len(), driver);
            }
            Message::InsertCreateTableColumnAfter(index) => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let insert_at = index.saturating_add(1).min(draft.columns.len());
                insert_create_table_column(draft, insert_at, driver);
            }
            Message::RemoveCreateTableColumn(index) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                if index < draft.columns.len() {
                    draft.columns.remove(index);
                }
            }
            Message::MoveCreateTableColumn(index, delta) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                move_vec_item(&mut draft.columns, index, delta);
            }
            Message::AddCreateTableIndexForColumn(index) => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let Some(column) = draft.columns.get(index) else {
                    self.status_message = String::from("Column row was not found.");
                    return Task::none();
                };
                let column_name = column.name.clone();
                draft.indexes.push(CreateTableIndexDraft::with_type(
                    &index_name_for_column(&draft.name, &column_name),
                    &column_name,
                    engine::schema::default_index_type(driver),
                ));
                self.create_table_tab = CreateTableTab::Indexes;
                self.status_message = format!("INDEX prepared for {column_name}.");
            }
            Message::AddCreateTableIndex => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                draft.indexes.push(CreateTableIndexDraft::with_type(
                    "idx_new_table_column",
                    "new_column",
                    engine::schema::default_index_type(driver),
                ));
            }
            Message::AppendCreateTableIndexColumn(index, column_name) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let Some(index_draft) = draft.indexes.get_mut(index) else {
                    self.status_message = String::from("Index row was not found.");
                    return Task::none();
                };
                engine::schema::append_index_column(&mut index_draft.columns, &column_name);
                self.create_table_tab = CreateTableTab::Indexes;
                self.status_message = format!(
                    "{}: {column_name}",
                    i18n::texts(self.language).index_columns
                );
            }
            Message::RemoveCreateTableIndex(index) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                if index < draft.indexes.len() {
                    draft.indexes.remove(index);
                }
            }
            Message::AddCreateTableConstraint => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                draft.constraints.push(CreateTableConstraintDraft::new(
                    "ck_new_table",
                    "CHECK",
                    "new_column is not null",
                ));
            }
            Message::RemoveCreateTableConstraint(index) => {
                let Some(draft) = self.create_table_draft.as_mut() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                if index < draft.constraints.len() {
                    draft.constraints.remove(index);
                }
            }
            Message::CancelCreateTable => {
                self.create_table_draft = None;
                self.create_table_tab = CreateTableTab::Columns;
                self.status_message = String::from("Create table cancelled.");
            }
            Message::SubmitCreateTable => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let Some(draft) = self.create_table_draft.as_ref() else {
                    self.status_message = String::from("Create table form is not open.");
                    return Task::none();
                };
                let table = draft.name().trim().to_owned();
                if table.is_empty() {
                    self.status_message = String::from("Table name is required.");
                    return Task::none();
                }

                let mut form = self.connection_manager.form().clone();
                let sql = match create_table_statement(&mut form, draft) {
                    Ok(sql) => sql,
                    Err(error) => {
                        self.status_message = error;
                        return Task::none();
                    }
                };

                self.schema_mutation_running = true;
                self.selected = Section::Tables;
                self.status_message = format!("Creating table {table}...");
                return Task::perform(run_execute_sql_task(form, sql), move |outcome| {
                    Message::CreateTableFinished(table.clone(), outcome)
                });
            }
            Message::CreateTableFinished(table, outcome) => {
                self.schema_mutation_running = false;
                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        self.create_table_draft = None;
                        self.create_table_tab = CreateTableTab::Columns;
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);
                        self.status_message = format!("Table {table} created.");
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Create table {table} failed.")
                        } else {
                            format!("Create table {table} failed: {detail}")
                        };
                    }
                }
            }
            Message::RunTableAction(action, index) => {
                self.schema_object_menu = None;
                let Some(object) = self.query_workspace.schema_objects().get(index).cloned() else {
                    self.status_message = String::from("Schema object was not found.");
                    return Task::none();
                };

                if !matches!(
                    object.kind,
                    SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
                ) {
                    self.status_message =
                        format!("{} does not support table actions.", object.display_label());
                    return Task::none();
                }

                match action {
                    TableAction::SelectRows => {
                        self.table_rows_page = Some(TableRowsPage::new(object.name.clone()));
                        self.selected = Section::QueryExplorer;
                        self.result_focus = true;
                        return self.load_table_rows_page(0);
                    }
                    TableAction::DescribeTable => {
                        self.selected = Section::Tables;
                        self.clear_database_inspector_context();
                        self.alter_table_draft = None;
                        return self.refresh_table_details(object.name.clone());
                    }
                    TableAction::RenameTable => {
                        self.selected = Section::Tables;
                        self.rename_table_draft = Some(RenameTableDraft::new(object.name.clone()));
                        self.status_message =
                            format!("Rename table form opened for {}.", object.display_label());
                    }
                    TableAction::AlterTable => {
                        self.selected = Section::Tables;
                        self.clear_database_inspector_context();
                        self.alter_table_tab = AlterTableTab::Columns;
                        self.alter_table_draft =
                            Some(AlterTableDraft::new(object.name.clone(), String::new()));
                        self.status_message =
                            format!("Table designer opened for {}.", object.display_label());
                        return self.refresh_table_details(object.name.clone());
                    }
                    TableAction::DropTable => {
                        self.selected = Section::Tables;
                        match pending_table_deletion(
                            self.connection_manager.form().clone(),
                            &object,
                        ) {
                            Ok(pending) => {
                                self.pending_schema_deletion = Some(pending);
                                self.status_message = format!(
                                    "Delete table confirmation requested for {}.",
                                    object.display_label()
                                );
                            }
                            Err(error) => self.status_message = error,
                        }
                    }
                }
            }
            Message::TableRowsLoaded(table, page, outcome) => {
                self.query_running = false;
                if let Some(current) = self.table_rows_page.as_mut() {
                    if current.table == table {
                        current.page = page;
                    }
                }
                let summary = self.query_workspace.apply_execution_outcome(outcome);
                self.result_row_count = summary.row_count;
                self.result_latency_ms = summary.elapsed_ms;
                self.status_message = summary.status_message;
            }
            Message::TableDetailsLoaded(table, outcome) => {
                self.table_detail_loading = false;
                match outcome {
                    Ok(details) => {
                        if let Some(draft) = self.alter_table_draft.as_mut() {
                            if draft.table == table {
                                draft.create_statement = details.create_statement.clone();
                            }
                        }
                        self.table_details = Some(details);
                        self.status_message = format!("Table details loaded for {table}.");
                    }
                    Err(messages) => {
                        self.table_details = None;
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Table detail load failed for {table}.")
                        } else {
                            format!("Table detail load failed for {table}: {detail}")
                        };
                    }
                }
            }
            Message::TableEditFieldChanged(field, value) => match field {
                TableEditField::NewName => {
                    let Some(draft) = self.rename_table_draft.as_mut() else {
                        self.status_message = String::from("Rename table form is not open.");
                        return Task::none();
                    };
                    draft.new_name = value;
                }
            },
            Message::AlterTableTabSelected(tab) => {
                let Some(draft) = self.alter_table_draft.as_mut() else {
                    self.status_message = String::from("Table designer is not open.");
                    return Task::none();
                };
                self.alter_table_tab = tab;
                match tab {
                    AlterTableTab::Columns => {
                        if matches!(
                            draft.operation,
                            AlterTableOperation::AddIndex | AlterTableOperation::AddConstraint
                        ) {
                            draft.operation = AlterTableOperation::RenameColumn;
                        }
                    }
                    AlterTableTab::Indexes => {
                        draft.operation = AlterTableOperation::AddIndex;
                        draft.index_type = String::from(engine::schema::default_index_type(
                            self.connection_manager.form().driver,
                        ));
                    }
                    AlterTableTab::Constraints => {
                        let driver = self.connection_manager.form().driver;
                        draft.operation = AlterTableOperation::AddConstraint;
                        if draft.constraint_name.trim().is_empty() {
                            draft.constraint_name =
                                format!("ck_{}", sanitize_identifier_part(draft.table()));
                        }
                        draft.constraint_kind =
                            String::from(engine::schema::default_constraint_type(driver));
                    }
                }
            }
            Message::SelectAlterTableColumn(index) => {
                self.selected_alter_table_column = Some(index);
                self.status_message = format!("Column row {} selected.", index + 1);
            }
            Message::InsertAlterTableColumnAfterSelection => {
                if let Some(index) = self.selected_alter_table_column {
                    self.selected_alter_table_column = Some(index.saturating_add(1));
                    self.insert_alter_table_column_after(index);
                } else {
                    self.add_alter_table_column();
                }
            }
            Message::RemoveAlterTableColumn(index) => {
                self.remove_alter_table_column(index);
                self.selected_alter_table_column = None;
            }
            Message::RemoveSelectedAlterTableColumn => {
                let Some(index) = self.selected_alter_table_column else {
                    self.status_message = String::from("Select a column row before removing.");
                    return Task::none();
                };
                self.remove_alter_table_column(index);
                self.selected_alter_table_column = None;
            }
            Message::RevertSelectedAlterTableChange => {
                self.revert_selected_alter_table_change();
            }
            Message::RevertAllAlterTableChanges => {
                self.revert_all_alter_table_changes();
            }
            Message::MoveSelectedAlterTableColumn(delta) => {
                let Some(index) = self.selected_alter_table_column else {
                    self.status_message = String::from("Select a column row before moving.");
                    return Task::none();
                };
                let next_index = self.alter_table_column_move_target(index, delta);
                self.prepare_alter_table_column_move(index, delta);
                self.selected_alter_table_column = Some(next_index.unwrap_or(index));
            }
            Message::AddAlterTableIndexForColumn(index) => {
                self.selected_alter_table_column = Some(index);
                self.add_alter_table_index_for_column(index);
            }
            Message::ToggleAlterTableIndexColumn(column_name) => {
                let Some(draft) = self.alter_table_draft.as_mut() else {
                    self.status_message = String::from("Table designer is not open.");
                    return Task::none();
                };
                draft.operation = AlterTableOperation::AddIndex;
                engine::schema::toggle_index_column(&mut draft.index_columns, &column_name);
                if draft.index_name.trim().is_empty() {
                    draft.index_name = index_name_for_column(draft.table(), &column_name);
                }
                self.alter_table_tab = AlterTableTab::Indexes;
                self.status_message = format!(
                    "{}: {column_name}",
                    i18n::texts(self.language).index_columns
                );
            }
            Message::PrepareAlterTableIndex => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.alter_table_draft.as_mut() else {
                    self.status_message = String::from("Table designer is not open.");
                    return Task::none();
                };
                draft.operation = AlterTableOperation::AddIndex;
                draft.index_name = index_name_for_column(draft.table(), "column");
                draft.index_columns.clear();
                draft.index_type = String::from(engine::schema::default_index_type(driver));
                self.alter_table_tab = AlterTableTab::Indexes;
                self.status_message = String::from("ADD INDEX draft prepared.");
            }
            Message::ClearAlterTableIndex => {
                let driver = self.connection_manager.form().driver;
                let Some(draft) = self.alter_table_draft.as_mut() else {
                    self.status_message = String::from("Table designer is not open.");
                    return Task::none();
                };
                draft.index_name.clear();
                draft.index_columns.clear();
                draft.index_type = String::from(engine::schema::default_index_type(driver));
                self.status_message = String::from("ADD INDEX draft cleared.");
            }
            Message::AlterTableColumnFieldChanged(index, field, value) => {
                self.update_alter_table_column(index, field, value);
            }
            Message::AlterTableFieldChanged(field, value) => {
                let Some(draft) = self.alter_table_draft.as_mut() else {
                    self.status_message = String::from("Table designer is not open.");
                    return Task::none();
                };

                match field {
                    AlterTableField::ColumnName => draft.column_name = value,
                    AlterTableField::NewColumnName => draft.new_column_name = value,
                    AlterTableField::ColumnType => draft.column_type = value,
                    AlterTableField::ColumnDefinition => draft.column_definition = value,
                    AlterTableField::IndexName => {
                        draft.operation = AlterTableOperation::AddIndex;
                        draft.index_name = value;
                    }
                    AlterTableField::IndexColumns => {
                        draft.operation = AlterTableOperation::AddIndex;
                        draft.index_columns = value;
                    }
                    AlterTableField::IndexType => {
                        draft.operation = AlterTableOperation::AddIndex;
                        draft.index_type = value;
                    }
                    AlterTableField::ConstraintName => {
                        draft.operation = AlterTableOperation::AddConstraint;
                        draft.constraint_name = value;
                    }
                    AlterTableField::ConstraintKind => {
                        draft.operation = AlterTableOperation::AddConstraint;
                        draft.constraint_kind = value;
                    }
                    AlterTableField::ConstraintExpression => {
                        draft.operation = AlterTableOperation::AddConstraint;
                        draft.constraint_expression = value;
                    }
                    AlterTableField::ColumnPosition => draft.column_position = value,
                    AlterTableField::AfterColumn => draft.after_column = value,
                }
            }
            Message::CancelTableEdit => {
                self.rename_table_draft = None;
                self.alter_table_draft = None;
                self.alter_table_tab = AlterTableTab::Columns;
                self.status_message = String::from("Table edit cancelled.");
            }
            Message::SubmitRenameTable => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let Some(draft) = self.rename_table_draft.as_ref() else {
                    self.status_message = String::from("Rename table form is not open.");
                    return Task::none();
                };
                let old_name = draft.table().trim().to_owned();
                let new_name = draft.new_name().trim().to_owned();
                if new_name.is_empty() {
                    self.status_message = String::from("New table name is required.");
                    return Task::none();
                }
                if old_name == new_name {
                    self.status_message = String::from("New table name must be different.");
                    return Task::none();
                }

                let mut form = self.connection_manager.form().clone();
                let sql = match rename_table_statement(&mut form, &old_name, &new_name) {
                    Ok(sql) => sql,
                    Err(error) => {
                        self.status_message = error;
                        return Task::none();
                    }
                };

                self.schema_mutation_running = true;
                self.selected = Section::Tables;
                self.status_message = format!("Renaming table {old_name} to {new_name}...");
                return Task::perform(run_execute_sql_task(form, sql), move |outcome| {
                    Message::RenameTableFinished(old_name.clone(), new_name.clone(), outcome)
                });
            }
            Message::RenameTableFinished(old_name, new_name, outcome) => {
                self.schema_mutation_running = false;
                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        self.rename_table_draft = None;
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);
                        self.status_message = format!("Table {old_name} renamed to {new_name}.");
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Rename table {old_name} failed.")
                        } else {
                            format!("Rename table {old_name} failed: {detail}")
                        };
                    }
                }
            }
            Message::SubmitAlterTable => {
                if self.schema_mutation_running {
                    self.status_message = String::from("Schema change is already running.");
                    return Task::none();
                }

                let Some(draft) = self.alter_table_draft.as_ref() else {
                    self.status_message = String::from("Table designer is not open.");
                    return Task::none();
                };
                let mut form = self.connection_manager.form().clone();
                let table = draft.table().to_owned();
                let sql = match alter_table_statement(&mut form, draft) {
                    Ok(sql) => sql,
                    Err(error) => {
                        self.status_message = error;
                        return Task::none();
                    }
                };

                self.schema_mutation_running = true;
                self.selected = Section::Tables;
                self.status_message = format!("Applying structure change to {table}...");
                return Task::perform(run_execute_sql_task(form, sql), move |outcome| {
                    Message::AlterTableFinished(table.clone(), outcome)
                });
            }
            Message::AlterTableFinished(table, outcome) => {
                self.schema_mutation_running = false;
                match outcome {
                    QueryExecutionOutcome::Success(result) => {
                        self.alter_table_draft = None;
                        self.alter_table_tab = AlterTableTab::Columns;
                        self.result_row_count = result.row_count();
                        self.result_latency_ms = Some(result.elapsed_ms);
                        self.status_message = format!("Structure change applied to {table}.");
                        return self.refresh_query_schema();
                    }
                    QueryExecutionOutcome::Failure(messages) => {
                        let detail = messages.join(" ");
                        self.status_message = if detail.is_empty() {
                            format!("Structure change failed for {table}.")
                        } else {
                            format!("Structure change failed for {table}: {detail}")
                        };
                    }
                }
            }
            Message::SystemMetricsTick => {
                let memory_label = system_metrics::process_memory_label();
                if self.memory_label != memory_label {
                    self.memory_label = memory_label;
                }
            }
            Message::DragWindow => {
                return window::get_latest().and_then(window::drag);
            }
            Message::MinimizeWindow => {
                return window::get_latest().and_then(|id| window::minimize(id, true));
            }
            Message::CloseWindow => {
                return window::get_latest().and_then(window::close);
            }
        }

        Task::none()
    }
}

#[cfg(test)]
mod tests;
