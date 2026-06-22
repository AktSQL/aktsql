use super::*;
use iced::widget::column;

pub(in crate::view) fn alter_table_designer_panel<'a>(
    state: &'a Akt,
    draft: &'a AlterTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let tab = state.alter_table_tab();
    let header = row![
        column![
            text(texts.modify_structure)
                .size(14)
                .wrapping(Wrapping::None)
                .style(theme::primary_text),
            text(draft.table().to_owned())
                .size(11)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
        ]
        .spacing(3)
        .width(Length::Fill),
        small_command_button(
            alter_table_revert_selected_label(state.language()),
            Message::RevertSelectedAlterTableChange,
        ),
        small_command_button(
            alter_table_revert_all_label(state.language()),
            Message::RevertAllAlterTableChanges,
        ),
        button(button_label(texts.cancel, 11))
            .height(28)
            .padding([0, 12])
            .style(theme::secondary_button)
            .on_press(Message::CancelTableEdit),
        alter_table_apply_button(true, state.language()),
    ]
    .spacing(10)
    .align_y(Alignment::Center);
    let mut content = column![header, alter_table_tab_bar(tab, state.language())].spacing(12);

    content = content.push(alter_table_tab_content(state, tab, draft));

    container(content)
        .style(theme::panel_low)
        .padding([12, 14])
        .width(Length::Fill)
        .into()
}

fn alter_table_revert_selected_label(language: Language) -> &'static str {
    match language {
        Language::ZhCn => "回退选中",
        _ => "Revert Selected",
    }
}

fn alter_table_revert_all_label(language: Language) -> &'static str {
    match language {
        Language::ZhCn => "回退全部",
        _ => "Revert All",
    }
}

fn alter_table_apply_button(enabled: bool, language: Language) -> Element<'static, Message> {
    let mut action = button(button_label(i18n::texts(language).apply, 11))
        .height(28)
        .padding([0, 14])
        .style(if enabled {
            theme::primary_button
        } else {
            theme::secondary_button
        });

    if enabled {
        action = action.on_press(Message::SubmitAlterTable);
    }

    action.into()
}

fn alter_table_tab_bar(active: AlterTableTab, language: Language) -> Element<'static, Message> {
    AlterTableTab::ALL
        .into_iter()
        .fold(row![].spacing(6).align_y(Alignment::Center), |row, tab| {
            row.push(alter_table_tab_button(tab, active == tab, language))
        })
        .into()
}

fn alter_table_tab_button(
    tab: AlterTableTab,
    active: bool,
    language: Language,
) -> Element<'static, Message> {
    let label = alter_table_tab_label(tab, language);
    button(
        text(label.to_uppercase())
            .size(10)
            .wrapping(Wrapping::None)
            .style(if active {
                theme::on_primary_text
            } else {
                theme::on_surface_text
            }),
    )
    .height(26)
    .padding([0, 12])
    .style(if active {
        theme::primary_button
    } else {
        theme::secondary_button
    })
    .on_press(Message::AlterTableTabSelected(tab))
    .into()
}

fn alter_table_tab_content<'a>(
    state: &'a Akt,
    tab: AlterTableTab,
    draft: &'a AlterTableDraft,
) -> Element<'a, Message> {
    match tab {
        AlterTableTab::Columns => alter_table_columns_tab(state, draft),
        AlterTableTab::Indexes => alter_table_indexes_tab(state, draft),
        AlterTableTab::Constraints => alter_table_constraints_tab(state, draft),
    }
}

fn alter_table_tab_label(tab: AlterTableTab, language: Language) -> &'static str {
    let texts = i18n::texts(language);
    match tab {
        AlterTableTab::Columns => texts.columns,
        AlterTableTab::Indexes => texts.indexes,
        AlterTableTab::Constraints => texts.constraints,
    }
}

fn alter_table_columns_tab<'a>(
    state: &'a Akt,
    _draft: &'a AlterTableDraft,
) -> Element<'a, Message> {
    alter_table_existing_columns_panel(state)
}

fn alter_table_existing_columns_panel<'a>(state: &'a Akt) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let Some(details) = state.table_details() else {
        return container(
            text(texts.loading)
                .size(11)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
        )
        .style(theme::panel_low)
        .padding(10)
        .width(Length::Fill)
        .into();
    };

    let columns = state
        .alter_table_draft()
        .filter(|draft| !draft.reordered_columns().is_empty())
        .map(AlterTableDraft::reordered_columns)
        .unwrap_or(details.columns.as_slice());

    let driver = state.connection_manager().form().driver;
    let language = state.language();
    let original_column_names = details
        .columns
        .iter()
        .map(|column| column.name.as_str())
        .collect::<Vec<_>>();
    let selected_column = state.selected_alter_table_column();
    let pending_add_index = columns.iter().position(|column| {
        !original_column_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(&column.name))
    });
    let has_pending_add = pending_add_index.is_some();
    let rows = columns.iter().enumerate().fold(
        column![alter_table_existing_columns_header(language)].spacing(6),
        |column, (index, column_detail)| {
            column.push(alter_table_existing_column_row(
                driver,
                index,
                column_detail,
                pending_add_index == Some(index),
                has_pending_add,
                selected_column == Some(index),
            ))
        },
    );

    container(
        column![
            row![
                detail_section_label(texts.columns),
                Space::with_width(Length::Fill),
                alter_table_column_toolbar(selected_column, columns.len(), pending_add_index),
            ]
            .align_y(Alignment::Center),
            rows,
        ]
        .spacing(8)
        .width(Length::Fill),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn alter_table_existing_columns_header(language: Language) -> Element<'static, Message> {
    let texts = i18n::texts(language);
    row![
        Space::with_width(34),
        grid_header(texts.column, 2),
        grid_header(texts.data_type, 2),
        grid_header(texts.nullable, 1),
        grid_header(texts.default_value, 2),
        grid_header(texts.extra_clause, 2),
        Space::with_width(34),
    ]
    .spacing(8)
    .into()
}

fn alter_table_existing_column_row<'a>(
    driver: DatabaseDriver,
    index: usize,
    column_detail: &'a engine::TableColumnDetail,
    pending_add: bool,
    has_pending_add: bool,
    selected: bool,
) -> Element<'a, Message> {
    let actions = if pending_add {
        row![icon_danger_button_enabled(
            "-",
            Some(Message::RemoveAlterTableColumn(index))
        )]
        .spacing(4)
        .align_y(Alignment::Center)
    } else {
        row![icon_command_button_enabled(
            "#",
            (!has_pending_add).then_some(Message::AddAlterTableIndexForColumn(index))
        )]
        .spacing(4)
        .align_y(Alignment::Center)
    };
    let data_type_cell = if pending_add {
        form_slot(
            alter_table_column_type_pick_list(driver, &column_detail.data_type, index),
            2,
        )
    } else {
        form_slot(
            readonly_index_cell(column_detail.data_type.clone(), index),
            2,
        )
    };
    let nullable_cell = if pending_add {
        form_slot(
            alter_table_column_input(
                "YES",
                &column_detail.nullable,
                index,
                CreateTableColumnField::Nullable,
            ),
            1,
        )
    } else {
        form_slot(
            readonly_index_cell(column_detail.nullable.clone(), index),
            1,
        )
    };
    let default_value_cell = if pending_add {
        form_slot(
            alter_table_column_input(
                "CURRENT_TIMESTAMP",
                &column_detail.default_value,
                index,
                CreateTableColumnField::DefaultValue,
            ),
            2,
        )
    } else {
        form_slot(
            readonly_index_cell(column_detail.default_value.clone(), index),
            2,
        )
    };
    let extra_cell = if pending_add {
        form_slot(
            alter_table_column_input(
                "AUTO_INCREMENT",
                &column_detail.extra,
                index,
                CreateTableColumnField::Extra,
            ),
            2,
        )
    } else {
        form_slot(readonly_index_cell(column_detail.extra.clone(), index), 2)
    };

    container(
        row![
            row_select_button(
                (index + 1).to_string(),
                selected,
                Message::SelectAlterTableColumn(index),
            ),
            form_slot(
                alter_table_column_input(
                    "id",
                    &column_detail.name,
                    index,
                    CreateTableColumnField::Name,
                ),
                2,
            ),
            data_type_cell,
            nullable_cell,
            default_value_cell,
            extra_cell,
            actions,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .padding([3, 0])
    .style(theme::schema_grid_row(index, selected))
    .into()
}

fn alter_table_column_toolbar(
    selected_column: Option<usize>,
    column_count: usize,
    pending_add_index: Option<usize>,
) -> Element<'static, Message> {
    let has_pending_add = pending_add_index.is_some();
    let can_add = !has_pending_add;
    let can_remove = selected_column.is_some_and(|index| pending_add_index == Some(index));
    let can_move_up = selected_column.is_some_and(|index| !has_pending_add && index > 0);
    let can_move_down =
        selected_column.is_some_and(|index| !has_pending_add && index + 1 < column_count);

    row![
        icon_command_button_enabled(
            "+",
            can_add.then_some(Message::InsertAlterTableColumnAfterSelection)
        ),
        icon_danger_button_enabled(
            "×",
            can_remove.then_some(Message::RemoveSelectedAlterTableColumn)
        ),
        icon_command_button_enabled(
            "▲",
            can_move_up.then_some(Message::MoveSelectedAlterTableColumn(-1))
        ),
        icon_command_button_enabled(
            "▼",
            can_move_down.then_some(Message::MoveSelectedAlterTableColumn(1))
        ),
    ]
    .spacing(5)
    .align_y(Alignment::Center)
    .into()
}

fn alter_table_indexes_tab<'a>(state: &'a Akt, draft: &'a AlterTableDraft) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());

    container(
        row![
            alter_table_index_toolbar(),
            column![
                detail_section_label(texts.indexes),
                alter_table_index_editor(state, draft),
                alter_table_existing_indexes_panel(state),
            ]
            .spacing(8)
            .width(Length::Fill),
        ]
        .spacing(10)
        .height(Length::Shrink),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn alter_table_index_toolbar() -> Element<'static, Message> {
    column![
        icon_command_button("+", Message::PrepareAlterTableIndex),
        icon_command_button("×", Message::ClearAlterTableIndex),
    ]
    .spacing(6)
    .width(34)
    .into()
}

fn alter_table_index_editor<'a>(
    state: &'a Akt,
    draft: &'a AlterTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let driver = state.connection_manager().form().driver;

    container(
        column![
            row![
                grid_header(texts.index, 2),
                grid_header(texts.index_type, 2),
                grid_header(texts.index_columns, 4),
            ]
            .spacing(8),
            row![
                form_slot(
                    alter_table_text_input(
                        texts.index,
                        "idx_users_email",
                        draft.index_name(),
                        AlterTableField::IndexName,
                    ),
                    2,
                ),
                form_slot(
                    alter_table_index_type_pick_list(driver, draft.index_type()),
                    2
                ),
                form_slot(
                    alter_table_text_input(
                        texts.index_columns,
                        "email, created_at",
                        draft.index_columns(),
                        AlterTableField::IndexColumns,
                    ),
                    4,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            row![
                fixed_width_field(
                    text(texts.selected_columns)
                        .size(10)
                        .wrapping(Wrapping::None)
                        .style(theme::secondary_text)
                        .into(),
                    116.0,
                ),
                text(draft.index_columns().to_owned())
                    .size(11)
                    .wrapping(Wrapping::None)
                    .style(theme::on_surface_text),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            text(texts.available_columns)
                .size(10)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
            alter_table_index_column_picker(state),
        ]
        .spacing(8),
    )
    .style(theme::panel_flat)
    .padding(8)
    .width(Length::Fill)
    .into()
}

fn alter_table_index_column_picker<'a>(state: &'a Akt) -> Element<'a, Message> {
    let columns = state
        .alter_table_draft()
        .filter(|draft| !draft.reordered_columns().is_empty())
        .map(AlterTableDraft::reordered_columns)
        .or_else(|| {
            state
                .table_details()
                .map(|details| details.columns.as_slice())
        })
        .unwrap_or(&[]);

    let selected_columns = state
        .alter_table_draft()
        .map(|draft| selected_index_columns(draft.index_columns()))
        .unwrap_or_default();
    let chips = columns
        .iter()
        .filter(|column| !column.name.trim().is_empty())
        .fold(
            row![].spacing(5).align_y(Alignment::Center),
            |row, column| {
                let selected = selected_columns
                    .iter()
                    .any(|selected| selected.eq_ignore_ascii_case(&column.name));
                row.push(index_column_toggle_chip(
                    column.name.clone(),
                    Message::ToggleAlterTableIndexColumn(column.name.clone()),
                    selected,
                ))
            },
        );

    scrollable(chips)
        .direction(ScrollDirection::Horizontal(Scrollbar::new()))
        .height(32)
        .style(theme::scrollable)
        .into()
}

fn selected_index_columns(columns: &str) -> Vec<String> {
    columns
        .split(',')
        .map(str::trim)
        .filter(|column| !column.is_empty())
        .map(str::to_owned)
        .collect()
}

fn alter_table_existing_indexes_panel<'a>(state: &'a Akt) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let Some(details) = state.table_details() else {
        return Space::with_height(0).into();
    };

    let rows = if details.indexes.is_empty() {
        column![text(texts.no_existing_indexes)
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text)]
        .spacing(5)
    } else {
        details.indexes.iter().enumerate().fold(
            column![alter_table_existing_indexes_header(state.language())].spacing(5),
            |column, (row_index, index)| {
                column.push(alter_table_existing_index_row(row_index, index))
            },
        )
    };

    container(
        column![detail_section_label(texts.existing_indexes), rows]
            .spacing(7)
            .width(Length::Fill),
    )
    .style(theme::panel_flat)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn alter_table_existing_index_row<'a>(
    row_index: usize,
    index: &'a engine::TableIndexDetail,
) -> Element<'a, Message> {
    let mut rows = column![row![
        form_slot(
            readonly_index_cell(format!("# {}", index.name), row_index),
            2
        ),
        form_slot(readonly_index_cell(index.unique.as_str(), row_index), 1),
        form_slot(readonly_index_cell(index.columns.as_str(), row_index), 3),
    ]
    .spacing(8)]
    .spacing(4);

    for column_name in selected_index_columns(&index.columns) {
        rows = rows.push(
            row![
                Space::with_width(24),
                form_slot(
                    readonly_index_cell(format!("-> {column_name}"), row_index),
                    2
                ),
                form_slot(readonly_index_cell("", row_index), 1),
                form_slot(readonly_index_cell("", row_index), 3),
            ]
            .spacing(8),
        );
    }

    rows.into()
}

fn alter_table_existing_indexes_header(language: Language) -> Element<'static, Message> {
    let texts = i18n::texts(language);
    row![
        grid_header(texts.index, 2),
        grid_header(texts.unique_index, 1),
        grid_header(texts.index_columns, 3),
    ]
    .spacing(8)
    .into()
}

fn readonly_index_cell(value: impl Into<String>, row_index: usize) -> Element<'static, Message> {
    container(
        text(value.into())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .height(28)
    .padding([5, 8])
    .style(theme::result_data_cell(row_index))
    .into()
}

fn alter_table_constraints_tab<'a>(
    state: &'a Akt,
    draft: &'a AlterTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let driver = state.connection_manager().form().driver;
    container(
        column![
            detail_section_label(texts.constraints),
            row![
                form_slot(
                    alter_table_text_input(
                        texts.constraint,
                        "ck_table_rule",
                        draft.constraint_name(),
                        AlterTableField::ConstraintName,
                    ),
                    2,
                ),
                form_slot(
                    column![
                        text(texts.constraint_type)
                            .size(11)
                            .wrapping(Wrapping::None)
                            .style(theme::secondary_text),
                        alter_table_constraint_type_pick_list(driver, draft.constraint_kind()),
                    ]
                    .spacing(4)
                    .into(),
                    2,
                ),
                form_slot(
                    alter_table_text_input(
                        texts.constraint_expression,
                        "status in ('active') / user_id references users(id)",
                        draft.constraint_expression(),
                        AlterTableField::ConstraintExpression,
                    ),
                    4,
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(8),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}
