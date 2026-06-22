use super::*;

impl Akt {
    pub(super) fn alter_table_column_move_target(&self, index: usize, delta: i32) -> Option<usize> {
        let columns = self
            .alter_table_draft
            .as_ref()
            .filter(|draft| !draft.reordered_columns.is_empty())
            .map(|draft| draft.reordered_columns.as_slice())
            .or_else(|| {
                self.table_details
                    .as_ref()
                    .map(|details| details.columns.as_slice())
            })?;

        if columns.is_empty() || index >= columns.len() {
            return None;
        }

        if delta.is_negative() {
            index.checked_sub(1)
        } else {
            index
                .checked_add(1)
                .filter(|next_index| *next_index < columns.len())
        }
    }

    pub(super) fn prepare_alter_table_column_move(&mut self, index: usize, delta: i32) {
        if !self.ensure_alter_table_column_draft() {
            return;
        }

        let current_columns = self
            .alter_table_draft
            .as_ref()
            .map(|draft| draft.reordered_columns.clone())
            .unwrap_or_default();
        let original_column_count = self
            .alter_table_draft
            .as_ref()
            .map(|draft| draft.original_column_names.len())
            .unwrap_or_default();

        if current_columns.len() > original_column_count {
            self.status_message =
                String::from("Apply or remove the pending ADD COLUMN row before moving columns.");
            return;
        }

        if index >= current_columns.len() {
            self.status_message = String::from("Column row was not found.");
            return;
        }

        let target_position = if delta.is_negative() {
            if index == 0 {
                self.status_message = String::from("Column is already first.");
                return;
            }
            index - 1
        } else {
            if index + 1 >= current_columns.len() {
                self.status_message = String::from("Column is already last.");
                return;
            }
            index + 1
        };

        let mut reordered_columns = current_columns;
        move_vec_item(&mut reordered_columns, index, delta);
        let column = reordered_columns
            .get(target_position)
            .expect("target column should exist after reorder")
            .clone();
        let position = if delta.is_negative() && target_position == 0 {
            String::from("FIRST")
        } else {
            String::from("AFTER")
        };
        let after_column = if position == "AFTER" {
            reordered_columns[target_position - 1].name.clone()
        } else {
            String::new()
        };

        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };
        let mut original_column_names = draft.original_column_names.clone();
        move_vec_item(&mut original_column_names, index, delta);

        draft.operation = AlterTableOperation::MoveColumn;
        draft.column_name = column.name.clone();
        draft.column_type = column.data_type.clone();
        draft.column_definition = table_column_definition_tail(&column);
        draft.column_position = position;
        draft.after_column = after_column;
        draft.reordered_columns = reordered_columns;
        draft.original_column_names = original_column_names;
        self.alter_table_tab = AlterTableTab::Columns;
        self.status_message = format!(
            "MOVE COLUMN prepared for {}. Apply to execute.",
            column.name
        );
    }

    pub(super) fn add_alter_table_column(&mut self) {
        self.insert_alter_table_column_at(usize::MAX);
    }

    pub(super) fn insert_alter_table_column_after(&mut self, index: usize) {
        self.insert_alter_table_column_at(index.saturating_add(1));
    }

    fn insert_alter_table_column_at(&mut self, insert_at: usize) {
        if !self.ensure_alter_table_column_draft() {
            return;
        }

        let driver = self.connection_manager.form().driver;
        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };

        if draft.reordered_columns.len() > draft.original_column_names.len() {
            let index = pending_alter_table_column_index(draft)
                .unwrap_or_else(|| draft.reordered_columns.len() - 1);
            sync_alter_table_add_column(draft, index);
            self.alter_table_tab = AlterTableTab::Columns;
            self.status_message =
                String::from("Finish the pending ADD COLUMN row before applying.");
            return;
        }

        let insert_at = insert_at.min(draft.reordered_columns.len());
        let name = next_alter_table_column_name(&draft.reordered_columns);
        draft.reordered_columns.insert(
            insert_at,
            TableColumnDetail {
                name,
                data_type: default_alter_table_column_type(driver),
                nullable: String::from("YES"),
                default_value: String::new(),
                extra: String::new(),
            },
        );
        sync_alter_table_add_column(draft, insert_at);
        self.alter_table_tab = AlterTableTab::Columns;
        self.status_message = String::from("ADD COLUMN row added. Apply to execute.");
    }

    pub(super) fn add_alter_table_index_for_column(&mut self, index: usize) {
        if !self.ensure_alter_table_column_draft() {
            return;
        }

        let driver = self.connection_manager.form().driver;
        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };
        if draft.reordered_columns.len() > draft.original_column_names.len() {
            self.status_message =
                String::from("Apply or remove the pending ADD COLUMN row before adding an index.");
            return;
        }
        let Some(column) = draft.reordered_columns.get(index) else {
            self.status_message = String::from("Column row was not found.");
            return;
        };
        let column_name = column.name.clone();
        draft.operation = AlterTableOperation::AddIndex;
        draft.index_name = index_name_for_column(draft.table(), &column_name);
        draft.index_columns = column_name.clone();
        draft.index_type = String::from(engine::schema::default_index_type(driver));
        self.alter_table_tab = AlterTableTab::Indexes;
        self.status_message = format!("INDEX prepared for {column_name}.");
    }

    pub(super) fn remove_alter_table_column(&mut self, index: usize) {
        if !self.ensure_alter_table_column_draft() {
            return;
        }

        let driver = self.connection_manager.form().driver;
        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };

        if !is_pending_alter_table_column(draft, index) || index >= draft.reordered_columns.len() {
            self.status_message = String::from("Only pending ADD COLUMN rows can be removed.");
            return;
        }

        draft.reordered_columns.remove(index);
        draft.operation = AlterTableOperation::RenameColumn;
        draft.column_name.clear();
        draft.column_type = default_alter_table_column_type(driver);
        draft.column_definition.clear();
        draft.column_position = String::from("LAST");
        draft.after_column.clear();
        self.status_message = String::from("Pending ADD COLUMN row removed.");
    }

    pub(super) fn revert_selected_alter_table_change(&mut self) {
        let Some(draft) = self.alter_table_draft.as_ref() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };

        match self.alter_table_tab {
            AlterTableTab::Indexes if draft.operation == AlterTableOperation::AddIndex => {
                self.clear_alter_table_index_draft();
                self.status_message = String::from("Selected ADD INDEX change reverted.");
                return;
            }
            AlterTableTab::Constraints if draft.operation == AlterTableOperation::AddConstraint => {
                self.clear_alter_table_constraint_draft();
                self.status_message = String::from("Selected ADD CONSTRAINT change reverted.");
                return;
            }
            _ => {}
        }

        let Some(index) = self.selected_alter_table_column else {
            self.status_message = String::from("Select a column change before reverting.");
            return;
        };

        if !self.ensure_alter_table_column_draft() {
            return;
        }

        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };

        if index >= draft.reordered_columns.len() {
            self.status_message = String::from("Column row was not found.");
            return;
        }

        if is_pending_alter_table_column(draft, index) {
            draft.reordered_columns.remove(index);
            reset_alter_table_column_operation(draft, self.connection_manager.form().driver);
            self.selected_alter_table_column = None;
            self.status_message = String::from("Selected ADD COLUMN change reverted.");
            return;
        }

        let Some(details) = self.table_details.as_ref() else {
            self.status_message = String::from("Table metadata is still loading.");
            return;
        };
        let original_name = draft.original_column_names.get(index).cloned().or_else(|| {
            draft
                .reordered_columns
                .get(index)
                .map(|column| column.name.clone())
        });
        let Some(original_name) = original_name else {
            self.status_message = String::from("Original column was not found.");
            return;
        };
        let Some(original_column) = details
            .columns
            .iter()
            .find(|column| column.name.eq_ignore_ascii_case(&original_name))
            .cloned()
        else {
            self.status_message = String::from("Original column metadata was not found.");
            return;
        };

        if index < draft.reordered_columns.len() {
            draft.reordered_columns[index] = original_column;
        }
        draft.original_column_names = details
            .columns
            .iter()
            .map(|column| column.name.clone())
            .collect();
        if draft.operation == AlterTableOperation::MoveColumn {
            draft.reordered_columns = details.columns.clone();
        }
        reset_alter_table_column_operation(draft, self.connection_manager.form().driver);
        self.status_message = String::from("Selected column change reverted.");
    }

    pub(super) fn revert_all_alter_table_changes(&mut self) {
        let driver = self.connection_manager.form().driver;
        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };

        if let Some(details) = self.table_details.as_ref() {
            draft.original_column_names = details
                .columns
                .iter()
                .map(|column| column.name.clone())
                .collect();
            draft.reordered_columns = details.columns.clone();
            draft.create_statement = details.create_statement.clone();
        } else {
            draft.original_column_names.clear();
            draft.reordered_columns.clear();
        }

        reset_alter_table_column_operation(draft, driver);
        draft.index_name.clear();
        draft.index_columns.clear();
        draft.index_type = String::from(engine::schema::default_index_type(driver));
        draft.constraint_name.clear();
        draft.constraint_kind = String::from(engine::schema::default_constraint_type(driver));
        draft.constraint_expression.clear();
        self.selected_alter_table_column = None;
        self.alter_table_tab = AlterTableTab::Columns;
        self.status_message = String::from("All structure changes reverted.");
    }

    pub(super) fn update_alter_table_column(
        &mut self,
        index: usize,
        field: CreateTableColumnField,
        value: String,
    ) {
        if !self.ensure_alter_table_column_draft() {
            return;
        }

        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return;
        };
        if draft.reordered_columns.len() > draft.original_column_names.len()
            && !is_pending_alter_table_column(draft, index)
        {
            self.status_message = String::from(
                "Apply or remove the pending ADD COLUMN row before editing existing columns.",
            );
            return;
        }
        let original_name = draft.original_column_names.get(index).cloned();
        let Some(column) = draft.reordered_columns.get_mut(index) else {
            self.status_message = String::from("Column row was not found.");
            return;
        };

        match field {
            CreateTableColumnField::Name => column.name = value,
            CreateTableColumnField::DataType => column.data_type = value,
            CreateTableColumnField::Nullable => column.nullable = value,
            CreateTableColumnField::DefaultValue => column.default_value = value,
            CreateTableColumnField::Extra => column.extra = value,
        }
        let edited_name = column.name.clone();

        if is_pending_alter_table_column(draft, index) {
            sync_alter_table_add_column(draft, index);
        } else if matches!(field, CreateTableColumnField::Name) {
            if let Some(original_name) = original_name {
                draft.operation = AlterTableOperation::RenameColumn;
                draft.column_name = original_name;
                draft.new_column_name = edited_name;
            }
        } else {
            self.status_message = String::from(
                "This existing column attribute is read-only; rename and reorder are supported.",
            );
            return;
        }
        self.alter_table_tab = AlterTableTab::Columns;
        self.status_message = String::from("Table structure draft updated. Apply to execute.");
    }

    fn ensure_alter_table_column_draft(&mut self) -> bool {
        let Some(details) = self.table_details.as_ref() else {
            self.status_message = String::from("Table columns are still loading.");
            return false;
        };
        let Some(draft) = self.alter_table_draft.as_mut() else {
            self.status_message = String::from("Table designer is not open.");
            return false;
        };

        if draft.reordered_columns.is_empty() {
            draft.original_column_names = details
                .columns
                .iter()
                .map(|column| column.name.clone())
                .collect();
            draft.reordered_columns = details.columns.clone();
        }

        true
    }
}

fn reset_alter_table_column_operation(draft: &mut AlterTableDraft, driver: DatabaseDriver) {
    draft.operation = AlterTableOperation::RenameColumn;
    draft.column_name.clear();
    draft.new_column_name.clear();
    draft.column_type = default_alter_table_column_type(driver);
    draft.column_definition.clear();
    draft.column_position = String::from("LAST");
    draft.after_column.clear();
}

fn clear_alter_table_index_draft_impl(draft: &mut AlterTableDraft, driver: DatabaseDriver) {
    draft.operation = AlterTableOperation::RenameColumn;
    draft.index_name.clear();
    draft.index_columns.clear();
    draft.index_type = String::from(engine::schema::default_index_type(driver));
}

fn clear_alter_table_constraint_draft_impl(draft: &mut AlterTableDraft, driver: DatabaseDriver) {
    draft.operation = AlterTableOperation::RenameColumn;
    draft.constraint_name.clear();
    draft.constraint_kind = String::from(engine::schema::default_constraint_type(driver));
    draft.constraint_expression.clear();
}

impl Akt {
    fn clear_alter_table_index_draft(&mut self) {
        let driver = self.connection_manager.form().driver;
        if let Some(draft) = self.alter_table_draft.as_mut() {
            clear_alter_table_index_draft_impl(draft, driver);
        }
    }

    fn clear_alter_table_constraint_draft(&mut self) {
        let driver = self.connection_manager.form().driver;
        if let Some(draft) = self.alter_table_draft.as_mut() {
            clear_alter_table_constraint_draft_impl(draft, driver);
        }
    }
}

pub(super) fn move_vec_item<T>(items: &mut [T], index: usize, delta: i32) {
    if items.is_empty() || index >= items.len() {
        return;
    }

    let target = if delta.is_negative() {
        index.saturating_sub(delta.unsigned_abs() as usize)
    } else {
        index.saturating_add(delta as usize).min(items.len() - 1)
    };

    if target != index {
        items.swap(index, target);
    }
}

pub(super) fn insert_create_table_column(
    draft: &mut CreateTableDraft,
    index: usize,
    driver: DatabaseDriver,
) {
    let existing = draft
        .columns
        .iter()
        .map(|column| TableColumnDetail {
            name: column.name.clone(),
            data_type: column.data_type.clone(),
            nullable: column.nullable.clone(),
            default_value: column.default_value.clone(),
            extra: column.extra.clone(),
        })
        .collect::<Vec<_>>();
    let insert_at = index.min(draft.columns.len());
    draft.columns.insert(
        insert_at,
        CreateTableColumnDraft::new(
            &next_alter_table_column_name(&existing),
            &default_alter_table_column_type(driver),
            "YES",
            "",
            "",
        ),
    );
}

pub(super) fn index_name_for_column(table: &str, column: &str) -> String {
    let table = last_table_part(table).trim_matches('"').trim_matches('`');
    let table = sanitize_identifier_part(table);
    let column = sanitize_identifier_part(column);
    format!("idx_{table}_{column}")
}

fn sync_alter_table_add_column(draft: &mut AlterTableDraft, index: usize) {
    let Some(column) = draft.reordered_columns.get(index).cloned() else {
        return;
    };

    draft.operation = AlterTableOperation::AddColumn;
    draft.column_name = column.name;
    draft.column_type = column.data_type;
    draft.column_definition = table_column_definition_tail(&TableColumnDetail {
        name: draft.column_name.clone(),
        data_type: draft.column_type.clone(),
        nullable: column.nullable,
        default_value: column.default_value,
        extra: column.extra,
    });
    if index == 0 {
        draft.column_position = String::from("FIRST");
        draft.after_column.clear();
    } else if index + 1 == draft.reordered_columns.len() {
        draft.column_position = String::from("LAST");
        draft.after_column.clear();
    } else {
        draft.column_position = String::from("AFTER");
        draft.after_column = draft.reordered_columns[index - 1].name.clone();
    }
}

fn pending_alter_table_column_index(draft: &AlterTableDraft) -> Option<usize> {
    draft
        .reordered_columns()
        .iter()
        .enumerate()
        .find(|(index, _)| is_pending_alter_table_column(draft, *index))
        .map(|(index, _)| index)
}

fn is_pending_alter_table_column(draft: &AlterTableDraft, index: usize) -> bool {
    let Some(column) = draft.reordered_columns().get(index) else {
        return false;
    };
    !draft
        .original_column_names()
        .iter()
        .any(|name| name.eq_ignore_ascii_case(&column.name))
}

fn next_alter_table_column_name(columns: &[TableColumnDetail]) -> String {
    let used = columns
        .iter()
        .map(|column| column.name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let base = "new_column";

    if !used.contains(base) {
        return base.to_owned();
    }

    (2..)
        .map(|suffix| format!("{base}_{suffix}"))
        .find(|name| !used.contains(&name.to_ascii_lowercase()))
        .unwrap_or_else(|| base.to_owned())
}

fn default_alter_table_column_type(driver: DatabaseDriver) -> String {
    match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            String::from("VARCHAR(255)")
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => String::from("TEXT"),
        DatabaseDriver::Sqlite => String::from("TEXT"),
        DatabaseDriver::SqlServer => String::from("NVARCHAR(255)"),
        DatabaseDriver::Oracle => String::from("VARCHAR2(255)"),
        DatabaseDriver::MongoDb => String::from("String"),
    }
}

pub(super) fn sanitize_identifier_part(value: &str) -> String {
    let sanitized = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect::<String>()
        .trim_matches('_')
        .to_owned();

    if sanitized.is_empty() {
        String::from("column")
    } else {
        sanitized
    }
}
