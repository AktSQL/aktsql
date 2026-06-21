use super::*;

impl Default for Akt {
    fn default() -> Self {
        let profiles = persistence::load_connection_profiles().unwrap_or_default();
        let language = Language::ZhCn;
        let texts = i18n::texts(language);
        let status_message = if profiles.is_empty() {
            String::from(texts.ready)
        } else {
            format!(
                "{} {} {}",
                texts.loading,
                profiles.len(),
                texts.saved_profiles
            )
        };

        Self {
            selected: Section::Databases,
            theme: ThemeMode::Dark,
            language,
            language_menu_open: false,
            connection_manager: ConnectionManager::with_profiles(profiles),
            query_workspace: QueryWorkspace::default(),
            query_editor: text_editor::Content::with_text(QueryWorkspace::DEFAULT_SQL),
            result_row_count: 0,
            result_latency_ms: None,
            query_result_order_by: Vec::new(),
            query_running: false,
            schema_loading: false,
            schema_mutation_running: false,
            connection_testing: false,
            connection_connecting: false,
            connection_task_token: 0,
            test_result_open: false,
            pending_delete_profile_id: None,
            pending_schema_deletion: None,
            result_focus: false,
            database_workspace_active: false,
            expanded_schema_objects: BTreeSet::new(),
            loading_schema_object_columns: HashSet::new(),
            schema_object_menu: None,
            database_expanded_from_tree: false,
            create_database_draft: None,
            create_table_draft: None,
            rename_database_draft: None,
            alter_database_charset_draft: None,
            rename_table_draft: None,
            alter_table_draft: None,
            selected_alter_table_column: None,
            create_table_tab: CreateTableTab::Columns,
            alter_table_tab: AlterTableTab::Columns,
            table_rows_page: None,
            table_detail_target: None,
            table_detail_loading: false,
            table_details: None,
            database_detail_target: None,
            database_detail_loading: false,
            database_details: None,
            memory_label: system_metrics::process_memory_label(),
            status_message,
        }
    }
}
