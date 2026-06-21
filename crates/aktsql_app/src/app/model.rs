use super::*;

#[derive(Debug, Clone)]
pub struct RenameDatabaseDraft {
    pub(super) database: String,
    pub(super) new_name: String,
}

impl RenameDatabaseDraft {
    pub(super) fn new(database: String) -> Self {
        Self {
            new_name: database.clone(),
            database,
        }
    }

    pub fn database(&self) -> &str {
        &self.database
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}

#[derive(Debug, Clone)]
pub struct AlterDatabaseCharsetDraft {
    pub(super) database: String,
    pub(super) charset: String,
    pub(super) collation: String,
}

impl AlterDatabaseCharsetDraft {
    pub(super) fn for_form(form: &ConnectionForm, database: String) -> Self {
        Self {
            database,
            charset: form.charset.clone(),
            collation: form.collation.clone(),
        }
    }

    pub fn database(&self) -> &str {
        &self.database
    }

    pub fn charset(&self) -> &str {
        &self.charset
    }

    pub fn collation(&self) -> &str {
        &self.collation
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateDatabaseField {
    Name,
    Charset,
    Collation,
    Owner,
    Template,
    Tablespace,
    InitialCollection,
}

#[derive(Debug, Clone)]
pub struct CreateDatabaseDraft {
    pub(super) name: String,
    pub(super) charset: String,
    pub(super) collation: String,
    pub(super) owner: String,
    pub(super) template: String,
    pub(super) tablespace: String,
    pub(super) initial_collection: String,
}

impl CreateDatabaseDraft {
    pub(super) fn for_driver(driver: DatabaseDriver) -> Self {
        Self {
            name: String::from("new_database"),
            charset: String::from(driver.default_charset()),
            collation: String::from(driver.default_collation()),
            owner: String::new(),
            template: String::new(),
            tablespace: String::new(),
            initial_collection: String::from("_aktsql_manager_init"),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn charset(&self) -> &str {
        &self.charset
    }

    pub fn collation(&self) -> &str {
        &self.collation
    }

    pub fn owner(&self) -> &str {
        &self.owner
    }

    pub fn template(&self) -> &str {
        &self.template
    }

    pub fn tablespace(&self) -> &str {
        &self.tablespace
    }

    pub fn initial_collection(&self) -> &str {
        &self.initial_collection
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateTableField {
    Name,
    Engine,
    Charset,
    Collation,
    Comment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateTableColumnField {
    Name,
    DataType,
    Nullable,
    DefaultValue,
    Extra,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateTableIndexField {
    Name,
    Columns,
    IndexType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateTableConstraintField {
    Name,
    Kind,
    Expression,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreateTableTab {
    Columns,
    Indexes,
    Constraints,
    Sql,
}

impl CreateTableTab {
    pub const ALL: [Self; 4] = [Self::Columns, Self::Indexes, Self::Constraints, Self::Sql];
}

#[derive(Debug, Clone)]
pub struct CreateTableDraft {
    pub(super) name: String,
    pub(super) columns: Vec<CreateTableColumnDraft>,
    pub(super) indexes: Vec<CreateTableIndexDraft>,
    pub(super) constraints: Vec<CreateTableConstraintDraft>,
    pub(super) engine: String,
    pub(super) charset: String,
    pub(super) collation: String,
    pub(super) comment: String,
}

impl CreateTableDraft {
    pub(super) fn for_driver(driver: DatabaseDriver) -> Self {
        let columns = match driver {
            DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
                vec![
                    CreateTableColumnDraft::new("id", "BIGINT", "NO", "", "AUTO_INCREMENT"),
                    CreateTableColumnDraft::new("name", "VARCHAR(255)", "NO", "", ""),
                    CreateTableColumnDraft::new(
                        "created_at",
                        "TIMESTAMP",
                        "NO",
                        "CURRENT_TIMESTAMP",
                        "",
                    ),
                ]
            }
            DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
                vec![
                    CreateTableColumnDraft::new("id", "BIGSERIAL", "NO", "", ""),
                    CreateTableColumnDraft::new("name", "TEXT", "NO", "", ""),
                    CreateTableColumnDraft::new("created_at", "TIMESTAMPTZ", "NO", "now()", ""),
                ]
            }
            DatabaseDriver::MongoDb => Vec::new(),
            DatabaseDriver::Sqlite => {
                vec![
                    CreateTableColumnDraft::new("id", "INTEGER", "NO", "", "AUTOINCREMENT"),
                    CreateTableColumnDraft::new("name", "TEXT", "NO", "", ""),
                    CreateTableColumnDraft::new(
                        "created_at",
                        "TEXT",
                        "NO",
                        "CURRENT_TIMESTAMP",
                        "",
                    ),
                ]
            }
            DatabaseDriver::SqlServer => {
                vec![
                    CreateTableColumnDraft::new("id", "BIGINT IDENTITY(1,1)", "NO", "", ""),
                    CreateTableColumnDraft::new("name", "NVARCHAR(255)", "NO", "", ""),
                    CreateTableColumnDraft::new(
                        "created_at",
                        "DATETIME2",
                        "NO",
                        "SYSUTCDATETIME()",
                        "",
                    ),
                ]
            }
            DatabaseDriver::Oracle => {
                vec![
                    CreateTableColumnDraft::new(
                        "id",
                        "NUMBER GENERATED BY DEFAULT AS IDENTITY",
                        "NO",
                        "",
                        "",
                    ),
                    CreateTableColumnDraft::new("name", "VARCHAR2(255)", "NO", "", ""),
                    CreateTableColumnDraft::new(
                        "created_at",
                        "TIMESTAMP",
                        "NO",
                        "CURRENT_TIMESTAMP",
                        "",
                    ),
                ]
            }
        };

        Self {
            name: match driver {
                DatabaseDriver::MongoDb => String::from("new_collection"),
                _ => String::from("new_table"),
            },
            columns,
            indexes: vec![CreateTableIndexDraft::new(
                "idx_new_table_name",
                "name",
                "NO",
            )],
            constraints: if driver == DatabaseDriver::MongoDb {
                Vec::new()
            } else {
                vec![CreateTableConstraintDraft::new(
                    "pk_new_table",
                    "PRIMARY KEY",
                    "id",
                )]
            },
            engine: match driver {
                DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
                    String::from("InnoDB")
                }
                _ => String::new(),
            },
            charset: String::from(driver.default_charset()),
            collation: String::from(driver.default_collation()),
            comment: String::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn columns(&self) -> &[CreateTableColumnDraft] {
        &self.columns
    }

    pub fn indexes(&self) -> &[CreateTableIndexDraft] {
        &self.indexes
    }

    pub fn constraints(&self) -> &[CreateTableConstraintDraft] {
        &self.constraints
    }

    pub fn engine(&self) -> &str {
        &self.engine
    }

    pub fn charset(&self) -> &str {
        &self.charset
    }

    pub fn collation(&self) -> &str {
        &self.collation
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

#[derive(Debug, Clone)]
pub struct CreateTableColumnDraft {
    pub(super) name: String,
    pub(super) data_type: String,
    pub(super) nullable: String,
    pub(super) default_value: String,
    pub(super) extra: String,
}

impl CreateTableColumnDraft {
    pub(super) fn new(
        name: &str,
        data_type: &str,
        nullable: &str,
        default_value: &str,
        extra: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            data_type: data_type.to_owned(),
            nullable: nullable.to_owned(),
            default_value: default_value.to_owned(),
            extra: extra.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn data_type(&self) -> &str {
        &self.data_type
    }

    pub fn nullable(&self) -> &str {
        &self.nullable
    }

    pub fn default_value(&self) -> &str {
        &self.default_value
    }

    pub fn extra(&self) -> &str {
        &self.extra
    }
}

#[derive(Debug, Clone)]
pub struct CreateTableIndexDraft {
    pub(super) name: String,
    pub(super) columns: String,
    pub(super) index_type: String,
}

impl CreateTableIndexDraft {
    pub(super) fn new(name: &str, columns: &str, unique: &str) -> Self {
        Self {
            name: name.to_owned(),
            columns: columns.to_owned(),
            index_type: String::from(if unique.eq_ignore_ascii_case("YES") {
                "UNIQUE BTREE"
            } else {
                "INDEX"
            }),
        }
    }

    pub(super) fn with_type(name: &str, columns: &str, index_type: &str) -> Self {
        Self {
            name: name.to_owned(),
            columns: columns.to_owned(),
            index_type: index_type.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn columns(&self) -> &str {
        &self.columns
    }

    pub fn index_type(&self) -> &str {
        &self.index_type
    }
}

#[derive(Debug, Clone)]
pub struct CreateTableConstraintDraft {
    pub(super) name: String,
    pub(super) kind: String,
    pub(super) expression: String,
}

impl CreateTableConstraintDraft {
    pub(super) fn new(name: &str, kind: &str, expression: &str) -> Self {
        Self {
            name: name.to_owned(),
            kind: kind.to_owned(),
            expression: expression.to_owned(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn expression(&self) -> &str {
        &self.expression
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterTableOperation {
    RenameColumn,
    AddColumn,
    AddIndex,
    AddConstraint,
    MoveColumn,
}

impl AlterTableOperation {
    pub fn label(self) -> &'static str {
        match self {
            Self::RenameColumn => "RENAME COLUMN",
            Self::AddColumn => "ADD COLUMN",
            Self::AddIndex => "ADD INDEX",
            Self::AddConstraint => "ADD CONSTRAINT",
            Self::MoveColumn => "MOVE COLUMN",
        }
    }
}

impl std::fmt::Display for AlterTableOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.label())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterTableTab {
    Columns,
    Indexes,
    Constraints,
    Ddl,
}

impl AlterTableTab {
    pub const ALL: [Self; 4] = [Self::Columns, Self::Indexes, Self::Constraints, Self::Ddl];
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlterTableField {
    ColumnName,
    NewColumnName,
    ColumnType,
    ColumnDefinition,
    IndexName,
    IndexColumns,
    IndexType,
    ConstraintName,
    ConstraintKind,
    ConstraintExpression,
    ColumnPosition,
    AfterColumn,
}

#[derive(Debug, Clone)]
pub struct AlterTableDraft {
    pub(super) table: String,
    pub(super) operation: AlterTableOperation,
    pub(super) column_name: String,
    pub(super) new_column_name: String,
    pub(super) column_type: String,
    pub(super) column_definition: String,
    pub(super) index_name: String,
    pub(super) index_columns: String,
    pub(super) index_type: String,
    pub(super) constraint_name: String,
    pub(super) constraint_kind: String,
    pub(super) constraint_expression: String,
    pub(super) column_position: String,
    pub(super) after_column: String,
    pub(super) reordered_columns: Vec<TableColumnDetail>,
    pub(super) original_column_names: Vec<String>,
    pub(super) create_statement: String,
}

impl AlterTableDraft {
    pub(super) fn new(table: String, create_statement: String) -> Self {
        Self {
            table,
            operation: AlterTableOperation::RenameColumn,
            column_name: String::new(),
            new_column_name: String::new(),
            column_type: String::from("TEXT"),
            column_definition: String::new(),
            index_name: String::new(),
            index_columns: String::new(),
            index_type: String::from("INDEX"),
            constraint_name: String::new(),
            constraint_kind: String::from("CHECK"),
            constraint_expression: String::new(),
            column_position: String::from("LAST"),
            after_column: String::new(),
            reordered_columns: Vec::new(),
            original_column_names: Vec::new(),
            create_statement,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn operation(&self) -> AlterTableOperation {
        self.operation
    }

    pub fn column_name(&self) -> &str {
        &self.column_name
    }

    pub fn new_column_name(&self) -> &str {
        &self.new_column_name
    }

    pub fn column_type(&self) -> &str {
        &self.column_type
    }

    pub fn column_definition(&self) -> &str {
        &self.column_definition
    }

    pub fn index_name(&self) -> &str {
        &self.index_name
    }

    pub fn index_columns(&self) -> &str {
        &self.index_columns
    }

    pub fn index_type(&self) -> &str {
        &self.index_type
    }

    pub fn constraint_name(&self) -> &str {
        &self.constraint_name
    }

    pub fn constraint_kind(&self) -> &str {
        &self.constraint_kind
    }

    pub fn constraint_expression(&self) -> &str {
        &self.constraint_expression
    }

    pub fn column_position(&self) -> &str {
        &self.column_position
    }

    pub fn after_column(&self) -> &str {
        &self.after_column
    }

    pub fn reordered_columns(&self) -> &[TableColumnDetail] {
        &self.reordered_columns
    }

    pub fn original_column_names(&self) -> &[String] {
        &self.original_column_names
    }

    pub fn create_statement(&self) -> &str {
        &self.create_statement
    }
}

#[derive(Debug, Clone)]
pub struct TableRowsPage {
    pub(super) table: String,
    pub(super) page: usize,
    pub(super) page_size: usize,
    pub(super) order_by: Vec<ResultSortKey>,
}

impl TableRowsPage {
    pub(super) fn new(table: String) -> Self {
        Self {
            table,
            page: 0,
            page_size: 100,
            order_by: Vec::new(),
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn page(&self) -> usize {
        self.page
    }

    pub fn page_size(&self) -> usize {
        self.page_size
    }

    pub fn order_by(&self) -> &[ResultSortKey] {
        &self.order_by
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub(super) fn next(self) -> Option<Self> {
        match self {
            Self::Desc => Some(Self::Asc),
            Self::Asc => None,
        }
    }

    pub(super) fn sql(self) -> &'static str {
        match self {
            Self::Asc => "ASC",
            Self::Desc => "DESC",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultSortKey {
    pub(super) column_index: usize,
    pub(super) column_name: String,
    pub(super) direction: SortDirection,
}

impl ResultSortKey {
    pub(super) fn new(column_index: usize, column_name: String) -> Self {
        Self {
            column_index,
            column_name,
            direction: SortDirection::Desc,
        }
    }

    pub fn column_index(&self) -> usize {
        self.column_index
    }

    pub fn direction(&self) -> SortDirection {
        self.direction
    }
}

#[derive(Debug, Clone)]
pub struct RenameTableDraft {
    pub(super) table: String,
    pub(super) new_name: String,
}

impl RenameTableDraft {
    pub(super) fn new(table: String) -> Self {
        Self {
            new_name: table.clone(),
            table,
        }
    }

    pub fn table(&self) -> &str {
        &self.table
    }

    pub fn new_name(&self) -> &str {
        &self.new_name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SchemaDeletionKind {
    Database,
    Table,
}

#[derive(Debug, Clone)]
pub struct PendingSchemaDeletion {
    pub(super) kind: SchemaDeletionKind,
    pub(super) name: String,
    pub(super) form: ConnectionForm,
    pub(super) sql: String,
}

impl PendingSchemaDeletion {
    pub fn kind(&self) -> SchemaDeletionKind {
        self.kind
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
