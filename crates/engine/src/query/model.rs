use crate::connection::DatabaseDriver;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub rows_affected: Option<usize>,
    pub elapsed_ms: u128,
    pub message: String,
    pub truncated: bool,
}

impl QueryResult {
    pub fn row_count(&self) -> usize {
        self.rows_affected.unwrap_or(self.rows.len())
    }
}

#[derive(Debug, Clone)]
pub struct DatabaseDetails {
    pub database: String,
    pub driver: DatabaseDriver,
    pub sections: Vec<DatabaseDetailSection>,
}

#[derive(Debug, Clone)]
pub struct DatabaseDetailSection {
    pub kind: DatabaseDetailSectionKind,
    pub fields: Vec<DatabaseDetailField>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseDetailSectionKind {
    Core,
    Storage,
    Objects,
    Runtime,
}

#[derive(Debug, Clone)]
pub struct DatabaseDetailField {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct TableDetails {
    pub table: String,
    pub driver: DatabaseDriver,
    pub sections: Vec<DatabaseDetailSection>,
    pub columns: Vec<TableColumnDetail>,
    pub indexes: Vec<TableIndexDetail>,
    pub create_statement: String,
}

#[derive(Debug, Clone)]
pub struct TableColumnDetail {
    pub name: String,
    pub data_type: String,
    pub nullable: String,
    pub default_value: String,
    pub extra: String,
}

#[derive(Debug, Clone)]
pub struct TableIndexDetail {
    pub name: String,
    pub columns: String,
    pub unique: String,
}

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub name: String,
    pub kind: SchemaObjectKind,
    depth: u8,
}

impl SchemaObject {
    pub fn new(name: String, kind: SchemaObjectKind, _preview: String) -> Self {
        Self {
            name,
            kind,
            depth: kind.default_depth(),
        }
    }

    pub fn display_label(&self) -> String {
        format!("{} {}", self.kind.icon(), self.name)
    }

    pub fn depth(&self) -> u8 {
        self.depth
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaObjectKind {
    Database,
    Table,
    View,
    Index,
    Collection,
    Column,
}

impl SchemaObjectKind {
    pub fn label(self) -> &'static str {
        match self {
            Self::Database => "database",
            Self::Table => "table",
            Self::View => "view",
            Self::Index => "index",
            Self::Collection => "collection",
            Self::Column => "column",
        }
    }

    fn icon(self) -> &'static str {
        match self {
            Self::Database => "DB",
            Self::Table => "TBL",
            Self::View => "VIEW",
            Self::Index => "IDX",
            Self::Collection => "COL",
            Self::Column => "FLD",
        }
    }

    fn default_depth(self) -> u8 {
        match self {
            Self::Database => 0,
            Self::Table | Self::View | Self::Index | Self::Collection => 1,
            Self::Column => 2,
        }
    }

    pub fn from_sqlite_type(value: &str) -> Option<Self> {
        match value {
            "table" => Some(Self::Table),
            "view" => Some(Self::View),
            "index" => Some(Self::Index),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct QueryExecutionSummary {
    pub row_count: usize,
    pub elapsed_ms: Option<u128>,
    pub status_message: String,
}

#[derive(Debug, Clone)]
pub enum QueryExecutionOutcome {
    Success(QueryResult),
    Failure(Vec<String>),
}

#[derive(Debug, Clone)]
pub struct SchemaLoadSummary {
    pub status_message: String,
}

#[derive(Debug, Clone)]
pub enum SchemaLoadOutcome {
    Success(Vec<SchemaObject>),
    Failure(Vec<String>),
}
