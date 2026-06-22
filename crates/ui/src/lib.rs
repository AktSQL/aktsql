pub mod i18n;
pub mod theme;
mod view;

use engine::{
    AlterDatabaseCharsetDraft, AlterTableDraft, AlterTableTab, ConnectionManager,
    CreateDatabaseDraft, CreateTableDraft, CreateTableTab, DatabaseDetails, Language,
    PendingSchemaDeletion, QueryWorkspace, RenameDatabaseDraft, RenameTableDraft, Section,
    TableDetails, ThemeMode, UiFontPreference,
};

pub use view::view;

pub type Akt = dyn ViewState;

pub trait ViewState {
    fn selected(&self) -> Section;
    fn theme(&self) -> ThemeMode;
    fn language(&self) -> Language;
    fn ui_font_preference(&self) -> UiFontPreference;
    fn connection_manager(&self) -> &ConnectionManager;
    fn query_workspace(&self) -> &QueryWorkspace;
    fn query_running(&self) -> bool;
    fn schema_loading(&self) -> bool;
    fn connection_testing(&self) -> bool;
    fn connection_connecting(&self) -> bool;
    fn test_result_open(&self) -> bool;
    fn pending_delete_profile_id(&self) -> Option<usize>;
    fn pending_schema_deletion(&self) -> Option<&PendingSchemaDeletion>;
    fn create_database_draft(&self) -> Option<&CreateDatabaseDraft>;
    fn create_table_draft(&self) -> Option<&CreateTableDraft>;
    fn rename_database_draft(&self) -> Option<&RenameDatabaseDraft>;
    fn alter_database_charset_draft(&self) -> Option<&AlterDatabaseCharsetDraft>;
    fn rename_table_draft(&self) -> Option<&RenameTableDraft>;
    fn alter_table_draft(&self) -> Option<&AlterTableDraft>;
    fn selected_alter_table_column(&self) -> Option<usize>;
    fn create_table_tab(&self) -> CreateTableTab;
    fn alter_table_tab(&self) -> AlterTableTab;
    fn table_detail_target(&self) -> Option<&str>;
    fn table_detail_loading(&self) -> bool;
    fn table_details(&self) -> Option<&TableDetails>;
    fn database_detail_target(&self) -> Option<&str>;
    fn database_detail_loading(&self) -> bool;
    fn database_details(&self) -> Option<&DatabaseDetails>;
    fn schema_object_expanded(&self, index: usize) -> bool;
    fn schema_object_menu_open(&self, index: usize) -> bool;
    fn database_workspace_active(&self) -> bool;
    fn result_row_count(&self) -> usize;
    fn result_latency_ms(&self) -> Option<u128>;
    fn memory_label(&self) -> &str;
    fn status_message(&self) -> &str;
}
