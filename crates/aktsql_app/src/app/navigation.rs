#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseAction {
    CreateDatabase,
    CreateTable,
    ShowDatabase,
    AlterDatabaseName,
    AlterDatabaseCharset,
    DropDatabase,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseEditField {
    NewName,
    Charset,
    Collation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableAction {
    SelectRows,
    DescribeTable,
    RenameTable,
    AlterTable,
    TruncateTable,
    DropTable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableEditField {
    NewName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum Section {
    Databases,
    QueryExplorer,
    Tables,
    Functions,
    History,
    Settings,
    Support,
}

impl Section {
    pub fn label(self) -> &'static str {
        match self {
            Section::Databases => "Databases",
            Section::QueryExplorer => "Query Explorer",
            Section::Tables => "Tables",
            Section::Functions => "Functions",
            Section::History => "History",
            Section::Settings => "Settings",
            Section::Support => "Support",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuickAction {
    NewConnection,
    NewQuery,
    ImportSql,
    ExportSql,
    RefreshData,
}

impl QuickAction {
    pub const PRIMARY: [QuickAction; 5] = [
        QuickAction::NewConnection,
        QuickAction::NewQuery,
        QuickAction::ImportSql,
        QuickAction::ExportSql,
        QuickAction::RefreshData,
    ];

    pub fn label(self) -> &'static str {
        match self {
            QuickAction::NewConnection => "New Connection",
            QuickAction::NewQuery => "New Query",
            QuickAction::ImportSql => "Import SQL",
            QuickAction::ExportSql => "Export SQL",
            QuickAction::RefreshData => "Refresh",
        }
    }

    pub(super) fn status_message(self) -> &'static str {
        match self {
            QuickAction::NewConnection => {
                "Connection manager requested. Engine-specific parameters are next."
            }
            QuickAction::NewQuery => {
                "Query editor requested. SQL completion and formatting are next."
            }
            QuickAction::ImportSql => {
                "SQL import requested. File selection and execution preview are next."
            }
            QuickAction::ExportSql => "SQL export requested. Schema/data export options are next.",
            QuickAction::RefreshData => "Refresh requested. No active result set is connected yet.",
        }
    }
}
