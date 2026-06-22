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
    RefreshData,
}

impl QuickAction {
    pub const PRIMARY: [QuickAction; 2] = [QuickAction::NewConnection, QuickAction::RefreshData];

    pub fn label(self) -> &'static str {
        match self {
            QuickAction::NewConnection => "New Connection",
            QuickAction::RefreshData => "Refresh",
        }
    }

    pub fn status_message(self) -> &'static str {
        match self {
            QuickAction::NewConnection => "New connection draft opened.",
            QuickAction::RefreshData => "Refresh requested.",
        }
    }
}
