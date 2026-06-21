use super::*;
use crate::query::execute_sql;

#[test]
fn connection_success_opens_database_workspace_not_query_explorer() {
    let mut app = Akt::default();
    app.connection_manager
        .save_current()
        .expect("default connection draft should save");

    let report = ConnectionTestReport {
        driver: DatabaseDriver::MySql,
        target: String::from("127.0.0.1:3306"),
        elapsed_ms: 1,
    };

    let _ = app.update(Message::ConnectionConnectFinished(0, Ok(report)));

    assert_eq!(app.selected(), Section::Databases);
    assert!(app.database_workspace_active());
}

#[test]
fn mysql_database_charset_template_uses_selected_charset_and_collation() {
    let sql = alter_database_charset_statement(
        DatabaseDriver::MySql,
        "myapp_db",
        "utf8mb4",
        "utf8mb4_unicode_ci",
    )
    .expect("mysql charset statement should be generated");

    assert!(sql.contains("ALTER DATABASE `myapp_db`"));
    assert!(sql.contains("CHARACTER SET utf8mb4"));
    assert!(sql.contains("COLLATE utf8mb4_unicode_ci"));
}

#[test]
fn mysql_database_rename_uses_live_information_schema_plan() {
    let mut form = ConnectionForm::default();
    form.driver = DatabaseDriver::MySql;

    let sql = rename_database_statement(
        &mut form,
        "old_db",
        "new_db",
        &[],
        "utf8mb4",
        "utf8mb4_unicode_ci",
    )
    .expect("mysql database rename statement should be generated");

    assert!(sql.contains("information_schema.tables"));
    assert!(sql.contains("PREPARE aktsql_rename_database_stmt"));
    assert!(sql.contains("DROP DATABASE `old_db`"));
    assert!(!sql.contains("old_db`.`cached_table"));
}

#[test]
fn database_rename_has_sqlserver_and_oracle_strategy() {
    let mut sqlserver = ConnectionForm::default();
    sqlserver.driver = DatabaseDriver::SqlServer;
    let sql = rename_database_statement(
        &mut sqlserver,
        "old_db",
        "new_db",
        &[],
        "UTF8",
        "Latin1_General_100_CI_AS_SC_UTF8",
    )
    .expect("sql server database rename statement should be generated");

    assert_eq!(sqlserver.database, "master");
    assert_eq!(sql, "ALTER DATABASE [old_db] MODIFY NAME = [new_db];");

    let mut oracle = ConnectionForm::default();
    oracle.driver = DatabaseDriver::Oracle;
    let sql = rename_database_statement(&mut oracle, "OLDPDB", "NEWPDB", &[], "", "")
        .expect("oracle database rename statement should be generated");

    assert!(sql.contains("ALTER PLUGGABLE DATABASE \"OLDPDB\""));
    assert!(sql.contains("RENAME GLOBAL_NAME TO \"NEWPDB\""));
}

#[test]
fn table_row_browser_uses_limit_offset_without_editor_preview() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");

    let sql = select_rows_statement(&mut form, "users", 2, 100, &[])
        .expect("select rows statement should be generated");

    assert_eq!(
        sql,
        "SELECT * FROM `myapp_db`.`users` LIMIT 100 OFFSET 200;"
    );
}

#[test]
fn table_row_browser_orders_before_limit_offset() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");
    let order_by = vec![
        ResultSortKey::new(0, String::from("id")),
        ResultSortKey {
            column_index: 1,
            column_name: String::from("name"),
            direction: SortDirection::Asc,
        },
    ];

    let sql = select_rows_statement(&mut form, "users", 2, 100, &order_by)
        .expect("select rows statement should be generated");

    assert_eq!(
        sql,
        "SELECT * FROM `myapp_db`.`users` ORDER BY `id` DESC, `name` ASC LIMIT 100 OFFSET 200;"
    );
}

#[test]
fn ad_hoc_query_sort_wraps_original_select() {
    let order_by = vec![ResultSortKey::new(0, String::from("created_at"))];

    let sql = ordered_query_sql(
        DatabaseDriver::PostgreSql,
        "select id, created_at from public.users;",
        &order_by,
    )
    .expect("ordered query should be generated");

    assert_eq!(
            sql,
            "SELECT * FROM (select id, created_at from public.users) AS aktsql_result ORDER BY \"created_at\" DESC;"
        );
}

#[test]
fn mongodb_table_row_browser_uses_find_skip_limit() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MongoDb);
    let order_by = vec![ResultSortKey::new(0, String::from("created_at"))];
    let sql = select_rows_statement(&mut form, "users", 1, 100, &order_by)
        .expect("mongodb find command should be generated");

    assert!(sql.contains("\"find\":\"users\""));
    assert!(sql.contains("\"sort\":{\"created_at\":-1}"));
    assert!(sql.contains("\"limit\":100"));
    assert!(sql.contains("\"skip\":100"));
}

#[test]
fn alter_table_generates_real_column_and_index_operations() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::PostgreSql);
    let mut draft = AlterTableDraft::new(String::from("public.users"), String::new());
    draft.column_name = String::from("name");
    draft.new_column_name = String::from("display_name");

    let sql = alter_table_statement(&mut form, &draft)
        .expect("rename column statement should be generated");
    assert_eq!(
        sql,
        "ALTER TABLE \"public\".\"users\" RENAME COLUMN \"name\" TO \"display_name\";"
    );

    draft.operation = AlterTableOperation::AddIndex;
    draft.index_name = String::from("idx_users_display_name");
    draft.index_columns = String::from("display_name");
    let sql = alter_table_statement(&mut form, &draft)
        .expect("create index statement should be generated");
    assert_eq!(
            sql,
            "CREATE INDEX \"idx_users_display_name\" ON \"public\".\"users\" USING btree (display_name);"
        );
}

#[test]
fn alter_table_add_column_uses_structured_type_and_position() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.operation = AlterTableOperation::AddColumn;
    draft.column_name = String::from("status");
    draft.column_type = String::from("VARCHAR(32)");
    draft.column_definition = String::from("NOT NULL DEFAULT 'active'");
    draft.column_position = String::from("AFTER");
    draft.after_column = String::from("name");

    let sql = alter_table_statement(&mut form, &draft)
        .expect("structured add column statement should be generated");

    assert_eq!(
            sql,
            "ALTER TABLE `myapp_db`.`users` ADD COLUMN `status` VARCHAR(32) NOT NULL DEFAULT 'active' AFTER `name`;"
        );
}

#[test]
fn alter_table_rejects_duplicate_column_names_before_sql_execution() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.original_column_names = vec![String::from("id"), String::from("name")];
    draft.operation = AlterTableOperation::AddColumn;
    draft.column_name = String::from("name");
    draft.column_type = String::from("VARCHAR(255)");

    let error = alter_table_statement(&mut form, &draft)
        .expect_err("existing column should be rejected before MySQL returns 1060");
    assert!(error.contains("Column already exists"));

    draft.operation = AlterTableOperation::RenameColumn;
    draft.column_name = String::from("id");
    draft.new_column_name = String::from("name");
    let error = alter_table_statement(&mut form, &draft)
        .expect_err("rename target should be rejected before MySQL returns 1060");
    assert!(error.contains("Column already exists"));

    draft.operation = AlterTableOperation::MoveColumn;
    draft.reordered_columns = vec![
        TableColumnDetail {
            name: String::from("id"),
            data_type: String::from("BIGINT"),
            nullable: String::from("NO"),
            default_value: String::new(),
            extra: String::new(),
        },
        TableColumnDetail {
            name: String::from("id"),
            data_type: String::from("VARCHAR(255)"),
            nullable: String::from("YES"),
            default_value: String::new(),
            extra: String::new(),
        },
    ];
    let error = alter_table_statement(&mut form, &draft)
        .expect_err("duplicate rebuild columns should be rejected before MySQL returns 1060");
    assert!(error.contains("Duplicate column name"));
}

#[test]
fn create_table_opens_form_without_sql_preview_navigation() {
    let mut app = Akt::default();
    app.connection_manager
        .save_current()
        .expect("default connection draft should save");
    app.connection_manager
        .set_field(ConnectionField::Database, String::from("myapp_db"));

    let original_sql = app.query_workspace.sql().to_owned();
    let _ = app.update(Message::RequestCreateTable);

    assert_eq!(app.selected(), Section::Tables);
    assert_eq!(app.query_workspace.sql(), original_sql);
    assert!(app.create_table_draft().is_some());
}

#[test]
fn create_table_statement_is_driver_specific_direct_command() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");
    let mut draft = CreateTableDraft::for_driver(DatabaseDriver::MySql);
    draft.comment = String::from("created from structured designer");

    let sql = create_table_statement(&mut form, &draft)
        .expect("mysql create table statement should be generated");

    assert!(sql.starts_with("CREATE TABLE `myapp_db`.`new_table`"));
    assert!(sql.contains("`id` BIGINT NOT NULL AUTO_INCREMENT"));
    assert!(sql.contains("`name` VARCHAR(255) NOT NULL"));
    assert!(sql.contains("CONSTRAINT `pk_new_table` PRIMARY KEY (id)"));
    assert!(sql.contains(
        "CREATE INDEX `idx_new_table_name` USING BTREE ON `myapp_db`.`new_table` (`name`);"
    ));
    assert!(sql.contains("ENGINE=InnoDB"));
    assert!(sql.contains("DEFAULT CHARSET=utf8mb4"));
    assert!(sql.contains("COLLATE=utf8mb4_0900_ai_ci"));
    assert!(sql.contains("COMMENT='created from structured designer'"));
}

#[test]
fn create_table_columns_move_with_arrow_actions() {
    let mut app = Akt::default();
    app.create_table_draft = Some(CreateTableDraft::for_driver(DatabaseDriver::MySql));

    let _ = app.update(Message::MoveCreateTableColumn(2, -1));
    let columns = app
        .create_table_draft()
        .expect("create table draft should stay open")
        .columns();

    assert_eq!(columns[0].name(), "id");
    assert_eq!(columns[1].name(), "created_at");
    assert_eq!(columns[2].name(), "name");
}

#[test]
fn create_table_columns_insert_after_row_and_index_selected_column() {
    let mut app = Akt::default();
    app.create_table_draft = Some(CreateTableDraft::for_driver(DatabaseDriver::MySql));

    let _ = app.update(Message::InsertCreateTableColumnAfter(0));
    let _ = app.update(Message::CreateTableColumnFieldChanged(
        1,
        CreateTableColumnField::Name,
        String::from("tenant_id"),
    ));
    let _ = app.update(Message::AddCreateTableIndexForColumn(1));

    let draft = app
        .create_table_draft()
        .expect("create table draft should stay open");
    assert_eq!(draft.columns()[1].name(), "tenant_id");
    assert_eq!(
        draft.indexes().last().map(CreateTableIndexDraft::columns),
        Some("tenant_id")
    );
    assert_eq!(app.create_table_tab(), CreateTableTab::Indexes);
}

#[test]
fn alter_table_column_arrow_prepares_move_operation() {
    let mut app = Akt::default();
    app.connection_manager.set_driver(DatabaseDriver::MySql);
    app.connection_manager
        .set_field(ConnectionField::Database, String::from("myapp_db"));
    app.alter_table_draft = Some(AlterTableDraft::new(String::from("users"), String::new()));
    app.table_details = Some(TableDetails {
        table: String::from("users"),
        driver: DatabaseDriver::MySql,
        sections: Vec::new(),
        columns: vec![
            TableColumnDetail {
                name: String::from("id"),
                data_type: String::from("BIGINT"),
                nullable: String::from("NO"),
                default_value: String::new(),
                extra: String::from("AUTO_INCREMENT"),
            },
            TableColumnDetail {
                name: String::from("name"),
                data_type: String::from("VARCHAR(255)"),
                nullable: String::from("NO"),
                default_value: String::new(),
                extra: String::new(),
            },
            TableColumnDetail {
                name: String::from("created_at"),
                data_type: String::from("TIMESTAMP"),
                nullable: String::from("NO"),
                default_value: String::from("CURRENT_TIMESTAMP"),
                extra: String::new(),
            },
        ],
        indexes: Vec::new(),
        create_statement: String::new(),
    });

    let _ = app.update(Message::SelectAlterTableColumn(2));
    let _ = app.update(Message::MoveSelectedAlterTableColumn(-1));
    let draft = app
        .alter_table_draft()
        .expect("alter table draft should stay open");

    assert_eq!(draft.operation(), AlterTableOperation::MoveColumn);
    assert_eq!(draft.column_name(), "created_at");
    assert_eq!(draft.column_position(), "AFTER");
    assert_eq!(draft.after_column(), "id");
    assert_eq!(draft.column_type(), "TIMESTAMP");
    assert!(draft.column_definition().contains("NOT NULL"));
    assert_eq!(draft.reordered_columns()[0].name, "id");
    assert_eq!(draft.reordered_columns()[1].name, "created_at");
    assert_eq!(draft.reordered_columns()[2].name, "name");
}

#[test]
fn alter_table_add_column_appends_editable_grid_row() {
    let mut app = Akt::default();
    app.connection_manager.set_driver(DatabaseDriver::MySql);
    app.connection_manager
        .set_field(ConnectionField::Database, String::from("myapp_db"));
    app.alter_table_draft = Some(AlterTableDraft::new(String::from("users"), String::new()));
    app.table_details = Some(TableDetails {
        table: String::from("users"),
        driver: DatabaseDriver::MySql,
        sections: Vec::new(),
        columns: vec![TableColumnDetail {
            name: String::from("id"),
            data_type: String::from("BIGINT"),
            nullable: String::from("NO"),
            default_value: String::new(),
            extra: String::new(),
        }],
        indexes: Vec::new(),
        create_statement: String::new(),
    });

    let _ = app.update(Message::InsertAlterTableColumnAfterSelection);
    let _ = app.update(Message::AlterTableColumnFieldChanged(
        1,
        CreateTableColumnField::Name,
        String::from("status"),
    ));
    let _ = app.update(Message::AlterTableColumnFieldChanged(
        1,
        CreateTableColumnField::DataType,
        String::from("VARCHAR(32)"),
    ));
    let _ = app.update(Message::AlterTableColumnFieldChanged(
        1,
        CreateTableColumnField::DefaultValue,
        String::from("active"),
    ));

    let draft = app
        .alter_table_draft()
        .expect("alter table draft should stay open");
    assert_eq!(draft.operation(), AlterTableOperation::AddColumn);
    assert_eq!(draft.column_name(), "status");
    assert_eq!(draft.column_type(), "VARCHAR(32)");
    assert!(draft.column_definition().contains("DEFAULT 'active'"));
    assert_eq!(draft.reordered_columns().len(), 2);

    let mut form = app.connection_manager().form().clone();
    let sql = alter_table_statement(&mut form, draft)
        .expect("add column statement should be generated from grid row");
    assert_eq!(
        sql,
        "ALTER TABLE `myapp_db`.`users` ADD COLUMN `status` VARCHAR(32) DEFAULT 'active';"
    );
}

#[test]
fn alter_table_insert_column_after_selected_row_sets_position() {
    let mut app = Akt::default();
    app.connection_manager.set_driver(DatabaseDriver::MySql);
    app.connection_manager
        .set_field(ConnectionField::Database, String::from("myapp_db"));
    app.alter_table_draft = Some(AlterTableDraft::new(String::from("users"), String::new()));
    app.table_details = Some(TableDetails {
        table: String::from("users"),
        driver: DatabaseDriver::MySql,
        sections: Vec::new(),
        columns: vec![
            TableColumnDetail {
                name: String::from("id"),
                data_type: String::from("BIGINT"),
                nullable: String::from("NO"),
                default_value: String::new(),
                extra: String::new(),
            },
            TableColumnDetail {
                name: String::from("name"),
                data_type: String::from("VARCHAR(255)"),
                nullable: String::from("YES"),
                default_value: String::new(),
                extra: String::new(),
            },
        ],
        indexes: Vec::new(),
        create_statement: String::new(),
    });

    let _ = app.update(Message::SelectAlterTableColumn(0));
    let _ = app.update(Message::InsertAlterTableColumnAfterSelection);
    let _ = app.update(Message::AlterTableColumnFieldChanged(
        1,
        CreateTableColumnField::Name,
        String::from("tenant_id"),
    ));

    let draft = app
        .alter_table_draft()
        .expect("alter table draft should stay open");
    assert_eq!(draft.operation(), AlterTableOperation::AddColumn);
    assert_eq!(draft.column_name(), "tenant_id");
    assert_eq!(draft.column_position(), "AFTER");
    assert_eq!(draft.after_column(), "id");
    assert_eq!(draft.reordered_columns()[1].name, "tenant_id");

    let mut form = app.connection_manager().form().clone();
    let sql = alter_table_statement(&mut form, draft)
        .expect("positioned add column statement should be generated");
    assert!(sql.contains("ADD COLUMN `tenant_id` VARCHAR(255)"));
    assert!(sql.contains("AFTER `id`"));
}

#[test]
fn alter_table_index_for_selected_column_prefills_index_form() {
    let mut app = Akt::default();
    app.connection_manager
        .set_driver(DatabaseDriver::PostgreSql);
    app.alter_table_draft = Some(AlterTableDraft::new(
        String::from("public.users"),
        String::new(),
    ));
    app.table_details = Some(TableDetails {
        table: String::from("public.users"),
        driver: DatabaseDriver::PostgreSql,
        sections: Vec::new(),
        columns: vec![TableColumnDetail {
            name: String::from("email"),
            data_type: String::from("TEXT"),
            nullable: String::from("NO"),
            default_value: String::new(),
            extra: String::new(),
        }],
        indexes: Vec::new(),
        create_statement: String::new(),
    });

    let _ = app.update(Message::AddAlterTableIndexForColumn(0));

    let draft = app
        .alter_table_draft()
        .expect("alter table draft should stay open");
    assert_eq!(draft.operation(), AlterTableOperation::AddIndex);
    assert_eq!(draft.index_columns(), "email");
    assert_eq!(app.alter_table_tab(), AlterTableTab::Indexes);
}

#[test]
fn index_type_is_driver_specific_in_alter_table_sql() {
    let mut mysql = ConnectionForm::for_driver(DatabaseDriver::MySql);
    mysql.database = String::from("myapp_db");
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.operation = AlterTableOperation::AddIndex;
    draft.index_name = String::from("idx_users_name");
    draft.index_columns = String::from("name");
    draft.index_type = String::from("FULLTEXT");
    let sql = alter_table_statement(&mut mysql, &draft)
        .expect("mysql fulltext index should be generated");
    assert_eq!(
        sql,
        "CREATE FULLTEXT INDEX `idx_users_name` ON `myapp_db`.`users` (`name`);"
    );

    let mut postgres = ConnectionForm::for_driver(DatabaseDriver::PostgreSql);
    draft.table = String::from("public.users");
    draft.index_type = String::from("GIN");
    let sql = alter_table_statement(&mut postgres, &draft)
        .expect("postgres gin index should be generated");
    assert_eq!(
        sql,
        "CREATE INDEX \"idx_users_name\" ON \"public\".\"users\" USING gin (name);"
    );

    let mut sqlserver = ConnectionForm::for_driver(DatabaseDriver::SqlServer);
    draft.table = String::from("dbo.users");
    draft.index_type = String::from("UNIQUE NONCLUSTERED");
    let sql = alter_table_statement(&mut sqlserver, &draft)
        .expect("sql server nonclustered index should be generated");
    assert_eq!(
        sql,
        "CREATE UNIQUE NONCLUSTERED INDEX [idx_users_name] ON [dbo].[users] (name);"
    );
}

#[test]
fn mysql_text_indexes_use_prefix_length_to_avoid_1170() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.operation = AlterTableOperation::AddIndex;
    draft.index_name = String::from("idx_users_bio");
    draft.index_columns = String::from("bio");
    draft.index_type = String::from("BTREE");
    draft.reordered_columns = vec![TableColumnDetail {
        name: String::from("bio"),
        data_type: String::from("TEXT"),
        nullable: String::from("YES"),
        default_value: String::new(),
        extra: String::new(),
    }];

    let sql = alter_table_statement(&mut form, &draft)
        .expect("mysql text index should use a prefix length");
    assert_eq!(
        sql,
        "CREATE INDEX `idx_users_bio` USING BTREE ON `myapp_db`.`users` (`bio`(191));"
    );

    draft.index_type = String::from("FULLTEXT");
    let sql = alter_table_statement(&mut form, &draft)
        .expect("mysql fulltext text index should not use a prefix length");
    assert_eq!(
        sql,
        "CREATE FULLTEXT INDEX `idx_users_bio` ON `myapp_db`.`users` (`bio`);"
    );
}

#[test]
fn mysql_create_table_text_indexes_use_prefix_length_to_avoid_1170() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.database = String::from("myapp_db");
    let mut draft = CreateTableDraft::for_driver(DatabaseDriver::MySql);
    draft
        .columns
        .push(CreateTableColumnDraft::new("bio", "TEXT", "YES", "", ""));
    draft.indexes.push(CreateTableIndexDraft::with_type(
        "idx_new_table_bio",
        "bio",
        "BTREE",
    ));

    let sql = create_table_statement(&mut form, &draft)
        .expect("mysql create table text index should use a prefix length");
    assert!(sql.contains(
        "CREATE INDEX `idx_new_table_bio` USING BTREE ON `myapp_db`.`new_table` (`bio`(191));"
    ));
}

#[test]
fn constraint_type_is_driver_specific_in_alter_table_sql() {
    let mut mysql = ConnectionForm::for_driver(DatabaseDriver::MySql);
    mysql.database = String::from("myapp_db");
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.operation = AlterTableOperation::AddConstraint;
    draft.constraint_name = String::from("ck_users_status");
    draft.constraint_kind = String::from("CHECK");
    draft.constraint_expression = String::from("status in ('active', 'disabled')");

    let sql =
        alter_table_statement(&mut mysql, &draft).expect("mysql check constraint should generate");
    assert_eq!(
        sql,
        "ALTER TABLE `myapp_db`.`users` ADD CONSTRAINT `ck_users_status` CHECK (status in ('active', 'disabled'));"
    );

    let mut postgres = ConnectionForm::for_driver(DatabaseDriver::PostgreSql);
    draft.table = String::from("public.users");
    draft.constraint_kind = String::from("UNIQUE");
    draft.constraint_expression = String::from("email");
    let sql = alter_table_statement(&mut postgres, &draft)
        .expect("postgres unique constraint should generate");
    assert_eq!(
        sql,
        "ALTER TABLE \"public\".\"users\" ADD CONSTRAINT \"ck_users_status\" UNIQUE (email);"
    );
}

#[test]
fn sqlite_and_mongodb_reject_direct_alter_table_constraints() {
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.operation = AlterTableOperation::AddConstraint;
    draft.constraint_name = String::from("ck_users_status");
    draft.constraint_kind = String::from("CHECK");
    draft.constraint_expression = String::from("status is not null");

    let mut sqlite = ConnectionForm::for_driver(DatabaseDriver::Sqlite);
    let error = alter_table_statement(&mut sqlite, &draft)
        .expect_err("sqlite should require table rebuild for constraints");
    assert!(error.contains("SQLite cannot add table constraints"));

    let mut mongodb = ConnectionForm::for_driver(DatabaseDriver::MongoDb);
    let error = alter_table_statement(&mut mongodb, &draft)
        .expect_err("mongodb should reject SQL constraints");
    assert!(error.contains("MongoDB constraints"));
}

#[test]
fn alter_table_insert_column_between_postgres_fields_rebuilds_table() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::PostgreSql);
    let mut draft = AlterTableDraft::new(String::from("public.users"), String::new());
    draft.operation = AlterTableOperation::AddColumn;
    draft.column_name = String::from("tenant_id");
    draft.column_type = String::from("TEXT");
    draft.column_position = String::from("AFTER");
    draft.after_column = String::from("id");
    draft.original_column_names = vec![String::from("id"), String::from("name")];
    draft.reordered_columns = vec![
        TableColumnDetail {
            name: String::from("id"),
            data_type: String::from("BIGINT"),
            nullable: String::from("NO"),
            default_value: String::new(),
            extra: String::new(),
        },
        TableColumnDetail {
            name: String::from("tenant_id"),
            data_type: String::from("TEXT"),
            nullable: String::from("YES"),
            default_value: String::new(),
            extra: String::new(),
        },
        TableColumnDetail {
            name: String::from("name"),
            data_type: String::from("TEXT"),
            nullable: String::from("YES"),
            default_value: String::new(),
            extra: String::new(),
        },
    ];

    let sql = alter_table_statement(&mut form, &draft)
        .expect("postgres positioned add column should rebuild table");

    assert!(sql.contains("CREATE TABLE \"public\".\"users__aktsql_reorder\""));
    assert!(sql.contains("INSERT INTO \"public\".\"users__aktsql_reorder\""));
    assert!(sql.contains("SELECT \"id\", NULL, \"name\" FROM \"public\".\"users\""));
}

#[test]
fn alter_table_pending_add_column_row_can_be_removed() {
    let mut app = Akt::default();
    app.connection_manager
        .set_driver(DatabaseDriver::PostgreSql);
    app.alter_table_draft = Some(AlterTableDraft::new(
        String::from("public.users"),
        String::new(),
    ));
    app.table_details = Some(TableDetails {
        table: String::from("public.users"),
        driver: DatabaseDriver::PostgreSql,
        sections: Vec::new(),
        columns: vec![TableColumnDetail {
            name: String::from("id"),
            data_type: String::from("BIGINT"),
            nullable: String::from("NO"),
            default_value: String::new(),
            extra: String::new(),
        }],
        indexes: Vec::new(),
        create_statement: String::new(),
    });

    let _ = app.update(Message::InsertAlterTableColumnAfterSelection);
    let _ = app.update(Message::RemoveAlterTableColumn(1));

    let draft = app
        .alter_table_draft()
        .expect("alter table draft should stay open");
    assert_eq!(draft.operation(), AlterTableOperation::RenameColumn);
    assert_eq!(draft.reordered_columns().len(), 1);
}

#[test]
fn alter_table_column_arrows_move_again_from_current_draft_order() {
    let mut app = Akt::default();
    app.connection_manager.set_driver(DatabaseDriver::MySql);
    app.connection_manager
        .set_field(ConnectionField::Database, String::from("myapp_db"));
    app.alter_table_draft = Some(AlterTableDraft::new(String::from("users"), String::new()));
    app.table_details = Some(TableDetails {
        table: String::from("users"),
        driver: DatabaseDriver::MySql,
        sections: Vec::new(),
        columns: vec![
            TableColumnDetail {
                name: String::from("id"),
                data_type: String::from("BIGINT"),
                nullable: String::from("NO"),
                default_value: String::new(),
                extra: String::new(),
            },
            TableColumnDetail {
                name: String::from("name"),
                data_type: String::from("VARCHAR(255)"),
                nullable: String::from("YES"),
                default_value: String::new(),
                extra: String::new(),
            },
            TableColumnDetail {
                name: String::from("created_at"),
                data_type: String::from("TIMESTAMP"),
                nullable: String::from("NO"),
                default_value: String::from("CURRENT_TIMESTAMP"),
                extra: String::new(),
            },
        ],
        indexes: Vec::new(),
        create_statement: String::new(),
    });

    let _ = app.update(Message::SelectAlterTableColumn(2));
    let _ = app.update(Message::MoveSelectedAlterTableColumn(-1));
    let _ = app.update(Message::MoveSelectedAlterTableColumn(-1));

    let draft = app
        .alter_table_draft()
        .expect("alter table draft should stay open");
    assert_eq!(draft.reordered_columns()[0].name, "created_at");
    assert_eq!(draft.column_position(), "FIRST");
}

#[test]
fn move_column_statement_is_generated_for_all_supported_drivers() {
    let mut draft = AlterTableDraft::new(String::from("users"), String::new());
    draft.operation = AlterTableOperation::MoveColumn;
    draft.column_name = String::from("created_at");
    draft.column_type = String::from("TIMESTAMP");
    draft.column_position = String::from("AFTER");
    draft.after_column = String::from("id");
    draft.reordered_columns = vec![
        TableColumnDetail {
            name: String::from("id"),
            data_type: String::from("BIGINT"),
            nullable: String::from("NO"),
            default_value: String::new(),
            extra: String::new(),
        },
        TableColumnDetail {
            name: String::from("created_at"),
            data_type: String::from("TIMESTAMP"),
            nullable: String::from("NO"),
            default_value: String::from("CURRENT_TIMESTAMP"),
            extra: String::new(),
        },
        TableColumnDetail {
            name: String::from("name"),
            data_type: String::from("VARCHAR(255)"),
            nullable: String::from("YES"),
            default_value: String::new(),
            extra: String::new(),
        },
    ];

    let mut mysql = ConnectionForm::for_driver(DatabaseDriver::MySql);
    mysql.database = String::from("myapp_db");
    let sql = move_column_statement(&mut mysql, "users", &draft)
        .expect("mysql move column should be generated");
    assert!(sql.contains("CREATE TABLE `myapp_db`.`users__aktsql_reorder`"));
    assert!(sql.contains("RENAME TABLE"));
    assert!(sql.contains("DROP TABLE `myapp_db`.`users__aktsql_backup`"));

    for driver in [
        DatabaseDriver::PostgreSql,
        DatabaseDriver::CockroachDb,
        DatabaseDriver::Sqlite,
        DatabaseDriver::SqlServer,
        DatabaseDriver::Oracle,
    ] {
        let mut form = ConnectionForm::for_driver(driver);
        let sql = move_column_statement(&mut form, "users", &draft)
            .expect("rebuild move column should be generated");
        assert!(sql.contains("CREATE TABLE"));
        assert!(sql.contains("INSERT INTO"));
        assert!(sql.contains("DROP TABLE"));
    }

    let mut mongo = ConnectionForm::for_driver(DatabaseDriver::MongoDb);
    let command = move_column_statement(&mut mongo, "users", &draft)
        .expect("mongodb reorder command should be generated");
    assert!(command.contains("\"update\":\"users\""));
    assert!(command.contains("\"$replaceRoot\""));
}

#[test]
fn table_column_definition_tail_omits_generated_default_metadata() {
    let column = TableColumnDetail {
        name: String::from("updated_at"),
        data_type: String::from("timestamp"),
        nullable: String::from("NO"),
        default_value: String::from("CURRENT_TIMESTAMP"),
        extra: String::from("DEFAULT_GENERATED on update CURRENT_TIMESTAMP"),
    };

    let definition = table_column_definition_tail(&column);

    assert!(definition.contains("NOT NULL"));
    assert!(definition.contains("DEFAULT CURRENT_TIMESTAMP"));
    assert!(definition.contains("on update CURRENT_TIMESTAMP"));
    assert!(!definition.contains("DEFAULT_GENERATED"));
}

#[test]
#[ignore = "requires local Docker database services"]
fn live_mysql_move_column_rebuild_succeeds() {
    let mut form = live_mysql_form();
    let cleanup = "DROP TABLE IF EXISTS `aktsql_reorder_probe__aktsql_backup`; DROP TABLE IF EXISTS `aktsql_reorder_probe__aktsql_reorder`; DROP TABLE IF EXISTS `aktsql_reorder_probe`;";
    assert_query_success(execute_sql(form.clone(), String::from(cleanup)));
    assert_query_success(execute_sql(
            form.clone(),
            String::from(
                "CREATE TABLE `aktsql_reorder_probe` (`id` BIGINT NOT NULL AUTO_INCREMENT PRIMARY KEY, `name` VARCHAR(64) NOT NULL DEFAULT 'n', `updated_at` TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP); INSERT INTO `aktsql_reorder_probe` (`name`) VALUES ('alpha');",
            ),
        ));

    let details =
        crate::query::load_table_details(form.clone(), String::from("aktsql_reorder_probe"))
            .expect("mysql table details should load");
    let id = details.columns[0].clone();
    let name = details.columns[1].clone();
    let updated_at = details.columns[2].clone();
    let mut draft = AlterTableDraft::new(
        String::from("aktsql_reorder_probe"),
        details.create_statement.clone(),
    );
    draft.operation = AlterTableOperation::MoveColumn;
    draft.reordered_columns = vec![id, updated_at, name];
    draft.original_column_names = vec![
        String::from("id"),
        String::from("updated_at"),
        String::from("name"),
    ];

    let sql = move_column_statement(&mut form, "aktsql_reorder_probe", &draft)
        .expect("mysql reorder statement should be generated");
    assert!(!sql.contains("DEFAULT_GENERATED"));
    assert_query_success(execute_sql(form.clone(), sql));

    let reordered =
        crate::query::load_table_details(form.clone(), String::from("aktsql_reorder_probe"))
            .expect("mysql reordered table details should load");
    let column_names = reordered
        .columns
        .iter()
        .map(|column| column.name.as_str())
        .collect::<Vec<_>>();
    assert_eq!(column_names, ["id", "updated_at", "name"]);

    assert_query_success(execute_sql(form, String::from(cleanup)));
}

fn live_mysql_form() -> ConnectionForm {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MySql);
    form.location = String::from("127.0.0.1");
    form.port = String::from("3306");
    form.username = String::from("root");
    form.password = String::from("root123");
    form.database = String::from("myapp_db");
    form
}

fn assert_query_success(outcome: QueryExecutionOutcome) {
    if let QueryExecutionOutcome::Failure(errors) = outcome {
        panic!("query should succeed: {errors:?}");
    }
}

#[test]
fn create_table_statement_supports_multi_column_unique_index_and_constraints() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::PostgreSql);
    form.database = String::from("myapp_db");
    let mut draft = CreateTableDraft::for_driver(DatabaseDriver::PostgreSql);
    draft.name = String::from("public.users");
    draft
        .columns
        .push(CreateTableColumnDraft::new("email", "TEXT", "NO", "", ""));
    draft.indexes = vec![CreateTableIndexDraft::new(
        "idx_users_name_email",
        "name, email",
        "YES",
    )];
    draft.constraints = vec![
        CreateTableConstraintDraft::new("pk_users", "PRIMARY KEY", "id"),
        CreateTableConstraintDraft::new("ck_users_email", "CHECK", "email <> ''"),
    ];

    let sql = create_table_statement(&mut form, &draft)
        .expect("postgres create table statement should be generated");

    assert!(sql.contains("\"email\" TEXT NOT NULL"));
    assert!(sql.contains("CONSTRAINT \"pk_users\" PRIMARY KEY (id)"));
    assert!(sql.contains("CONSTRAINT \"ck_users_email\" CHECK (email <> '')"));
    assert!(sql.contains(
            "CREATE UNIQUE INDEX \"idx_users_name_email\" ON \"public\".\"users\" USING btree (name, email);"
        ));
}

#[test]
fn mongodb_create_collection_statement_includes_index_commands() {
    let mut form = ConnectionForm::for_driver(DatabaseDriver::MongoDb);
    form.database = String::from("myapp_db");
    let mut draft = CreateTableDraft::for_driver(DatabaseDriver::MongoDb);
    draft.name = String::from("events");
    draft.indexes = vec![
        CreateTableIndexDraft::new("idx_events_account", "account_id", "NO"),
        CreateTableIndexDraft::new("idx_events_account_created", "account_id, created_at", "NO"),
    ];

    let command = create_table_statement(&mut form, &draft)
        .expect("mongodb create collection command should be generated");

    assert!(command.contains("\"create\":\"events\""));
    assert!(command.contains("\"createIndexes\":\"events\""));
    assert!(command.contains("\"name\":\"idx_events_account\""));
    assert!(command.contains("\"account_id\":1"));
    assert!(command.contains("\"created_at\":1"));
}

#[test]
fn metadata_inspector_contexts_are_mutually_exclusive() {
    let mut app = Akt::default();
    app.connection_manager
        .save_current()
        .expect("default connection draft should save");

    let _ = app.refresh_database_details(String::from("myapp_db"));
    assert_eq!(app.database_detail_target(), Some("myapp_db"));
    assert_eq!(app.table_detail_target(), None);
    assert!(app.alter_table_draft().is_none());

    let _ = app.refresh_table_details(String::from("users"));
    assert_eq!(app.database_detail_target(), None);
    assert_eq!(app.table_detail_target(), Some("users"));

    let _ = app.refresh_database_details(String::from("myapp_db"));
    assert_eq!(app.database_detail_target(), Some("myapp_db"));
    assert_eq!(app.table_detail_target(), None);
    assert!(app.alter_table_draft().is_none());
}

#[test]
fn alter_table_tabs_keep_operations_self_consistent() {
    let mut app = Akt::default();
    app.alter_table_draft = Some(AlterTableDraft::new(String::from("users"), String::new()));

    let _ = app.update(Message::AlterTableTabSelected(AlterTableTab::Indexes));
    assert_eq!(app.alter_table_tab(), AlterTableTab::Indexes);
    assert_eq!(
        app.alter_table_draft().map(AlterTableDraft::operation),
        Some(AlterTableOperation::AddIndex)
    );

    let _ = app.update(Message::AlterTableTabSelected(AlterTableTab::Columns));
    assert_eq!(app.alter_table_tab(), AlterTableTab::Columns);
    assert_eq!(
        app.alter_table_draft().map(AlterTableDraft::operation),
        Some(AlterTableOperation::RenameColumn)
    );
}

#[test]
fn index_column_append_keeps_comma_list_unique() {
    let mut columns = String::from("name, created_at");

    crate::schema::append_index_column(&mut columns, "NAME");
    assert_eq!(columns, "name, created_at");

    crate::schema::append_index_column(&mut columns, "email");
    assert_eq!(columns, "name, created_at, email");
}

#[test]
fn index_column_chip_actions_update_create_and_alter_drafts() {
    let mut app = Akt::default();
    app.create_table_draft = Some(CreateTableDraft::for_driver(DatabaseDriver::MySql));
    app.create_table_draft
        .as_mut()
        .expect("create table draft should exist")
        .indexes
        .push(CreateTableIndexDraft::with_type(
            "idx_new_table",
            "name",
            "BTREE",
        ));

    let _ = app.update(Message::AppendCreateTableIndexColumn(
        0,
        String::from("created_at"),
    ));

    assert_eq!(
        app.create_table_draft()
            .and_then(|draft| draft.indexes().first())
            .map(CreateTableIndexDraft::columns),
        Some("name, created_at")
    );

    app.alter_table_draft = Some(AlterTableDraft::new(
        String::from("users"),
        String::from("CREATE TABLE users (id INT);"),
    ));
    let _ = app.update(Message::ToggleAlterTableIndexColumn(String::from("email")));

    assert_eq!(
        app.alter_table_draft().map(AlterTableDraft::index_columns),
        Some("email")
    );
    assert_eq!(
        app.alter_table_draft().map(AlterTableDraft::operation),
        Some(AlterTableOperation::AddIndex)
    );

    let _ = app.update(Message::ToggleAlterTableIndexColumn(String::from("email")));

    assert_eq!(
        app.alter_table_draft().map(AlterTableDraft::index_columns),
        Some("")
    );
}

#[test]
fn default_language_is_simplified_chinese() {
    let app = Akt::default();

    assert_eq!(app.language(), Language::ZhCn);
}

#[test]
fn create_database_submit_runs_directly_without_sql_preview_navigation() {
    let mut app = Akt::default();
    app.connection_manager
        .save_current()
        .expect("default connection draft should save");

    let original_sql = app.query_workspace.sql().to_owned();

    let _ = app.update(Message::RunDatabaseAction(DatabaseAction::CreateDatabase));
    let _ = app.update(Message::CreateDatabaseFieldChanged(
        CreateDatabaseField::Name,
        String::from("aktsql_direct_create_test"),
    ));
    let _ = app.update(Message::SubmitCreateDatabase);

    assert_eq!(app.selected(), Section::Databases);
    assert_eq!(app.query_workspace.sql(), original_sql);
    assert!(app.create_database_draft().is_some());
}
