use super::*;

impl Akt {
    pub fn selected(&self) -> Section {
        self.selected
    }

    pub fn theme(&self) -> ThemeMode {
        self.theme
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn ui_font_preference(&self) -> UiFontPreference {
        self.ui_font_preference
    }

    pub fn connection_manager(&self) -> &ConnectionManager {
        &self.connection_manager
    }

    pub fn query_workspace(&self) -> &QueryWorkspace {
        &self.query_workspace
    }

    pub fn query_running(&self) -> bool {
        self.query_running
    }

    pub fn schema_loading(&self) -> bool {
        self.schema_loading
    }

    pub fn connection_testing(&self) -> bool {
        self.connection_testing
    }

    pub fn connection_connecting(&self) -> bool {
        self.connection_connecting
    }

    pub fn test_result_open(&self) -> bool {
        self.test_result_open
    }

    pub fn pending_delete_profile_id(&self) -> Option<usize> {
        self.pending_delete_profile_id
    }

    pub fn pending_schema_deletion(&self) -> Option<&PendingSchemaDeletion> {
        self.pending_schema_deletion.as_ref()
    }

    pub fn create_database_draft(&self) -> Option<&CreateDatabaseDraft> {
        self.create_database_draft.as_ref()
    }

    pub fn create_table_draft(&self) -> Option<&CreateTableDraft> {
        self.create_table_draft.as_ref()
    }

    pub fn rename_database_draft(&self) -> Option<&RenameDatabaseDraft> {
        self.rename_database_draft.as_ref()
    }

    pub fn alter_database_charset_draft(&self) -> Option<&AlterDatabaseCharsetDraft> {
        self.alter_database_charset_draft.as_ref()
    }

    pub fn rename_table_draft(&self) -> Option<&RenameTableDraft> {
        self.rename_table_draft.as_ref()
    }

    pub fn alter_table_draft(&self) -> Option<&AlterTableDraft> {
        self.alter_table_draft.as_ref()
    }

    pub fn selected_alter_table_column(&self) -> Option<usize> {
        self.selected_alter_table_column
    }

    pub fn create_table_tab(&self) -> CreateTableTab {
        self.create_table_tab
    }

    pub fn alter_table_tab(&self) -> AlterTableTab {
        self.alter_table_tab
    }

    pub fn table_detail_target(&self) -> Option<&str> {
        self.table_detail_target.as_deref()
    }

    pub fn table_detail_loading(&self) -> bool {
        self.table_detail_loading
    }

    pub fn table_details(&self) -> Option<&TableDetails> {
        self.table_details.as_ref()
    }

    pub fn database_detail_target(&self) -> Option<&str> {
        self.database_detail_target.as_deref()
    }

    pub fn database_detail_loading(&self) -> bool {
        self.database_detail_loading
    }

    pub fn database_details(&self) -> Option<&DatabaseDetails> {
        self.database_details.as_ref()
    }

    pub fn schema_object_expanded(&self, index: usize) -> bool {
        self.expanded_schema_objects.contains(&index)
    }

    pub fn schema_object_menu_open(&self, index: usize) -> bool {
        self.schema_object_menu == Some(index)
    }

    pub fn database_workspace_active(&self) -> bool {
        self.database_workspace_active
    }

    pub fn result_row_count(&self) -> usize {
        self.result_row_count
    }

    pub fn result_latency_ms(&self) -> Option<u128> {
        self.result_latency_ms
    }

    pub fn memory_label(&self) -> &str {
        &self.memory_label
    }

    pub fn status_message(&self) -> &str {
        &self.status_message
    }
}
