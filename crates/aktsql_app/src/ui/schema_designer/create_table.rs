use super::*;
use iced::widget::column;

pub(in crate::ui) fn create_table_designer_panel<'a>(
    state: &'a Akt,
    draft: &'a CreateTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let driver = state.connection_manager().form().driver;
    let title = if driver == DatabaseDriver::MongoDb {
        texts.create_collection
    } else {
        texts.create_table
    };

    container(
        column![
            row![
                column![
                    text(title)
                        .size(14)
                        .wrapping(Wrapping::None)
                        .style(theme::primary_text),
                    text(draft.name().to_owned())
                        .size(11)
                        .wrapping(Wrapping::None)
                        .style(theme::secondary_text),
                ]
                .spacing(3)
                .width(Length::Fill),
                button(button_label(texts.cancel, 11))
                    .height(28)
                    .padding([0, 12])
                    .style(theme::secondary_button)
                    .on_press(Message::CancelCreateTable),
                button(button_label(title, 11))
                    .height(28)
                    .padding([0, 14])
                    .style(theme::primary_button)
                    .on_press(Message::SubmitCreateTable),
            ]
            .spacing(10)
            .align_y(Alignment::Center),
            create_table_options_panel(state.language(), driver, draft),
            create_table_tab_bar(state.create_table_tab(), state.language()),
            create_table_tab_content(state, state.create_table_tab(), draft),
        ]
        .spacing(12),
    )
    .style(theme::panel_low)
    .padding([12, 14])
    .width(Length::Fill)
    .into()
}

fn create_table_options_panel<'a>(
    language: Language,
    driver: DatabaseDriver,
    draft: &'a CreateTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(language);
    let table_label = if driver == DatabaseDriver::MongoDb {
        texts.collection_node
    } else {
        texts.table_node
    };

    container(
        column![
            detail_section_label(texts.table_options),
            row![
                fixed_width_field(
                    create_table_text_input(
                        table_label,
                        "new_table",
                        draft.name(),
                        CreateTableField::Name,
                    ),
                    220.0,
                ),
                fixed_width_field(
                    create_table_text_input(
                        "ENGINE",
                        "InnoDB",
                        draft.engine(),
                        CreateTableField::Engine,
                    ),
                    160.0,
                ),
                form_slot(
                    create_table_text_input(
                        "CHARSET",
                        "utf8mb4",
                        draft.charset(),
                        CreateTableField::Charset,
                    ),
                    1,
                ),
                form_slot(
                    create_table_text_input(
                        "COLLATION",
                        "utf8mb4_unicode_ci",
                        draft.collation(),
                        CreateTableField::Collation,
                    ),
                    1,
                ),
            ]
            .spacing(10),
            create_table_text_input(
                "COMMENT",
                "business meaning / maintenance note",
                draft.comment(),
                CreateTableField::Comment,
            ),
        ]
        .spacing(8),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn create_table_tab_bar(active: CreateTableTab, language: Language) -> Element<'static, Message> {
    CreateTableTab::ALL
        .into_iter()
        .fold(row![].spacing(6).align_y(Alignment::Center), |row, tab| {
            row.push(create_table_tab_button(tab, active == tab, language))
        })
        .into()
}

fn create_table_tab_button(
    tab: CreateTableTab,
    active: bool,
    language: Language,
) -> Element<'static, Message> {
    let texts = i18n::texts(language);
    let label = match tab {
        CreateTableTab::Columns => texts.columns,
        CreateTableTab::Indexes => texts.indexes,
        CreateTableTab::Constraints => texts.constraints,
        CreateTableTab::Sql => texts.create_table_statement,
    };

    button(
        text(label.to_uppercase())
            .size(10)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .height(26)
    .padding([0, 12])
    .style(if active {
        theme::primary_button
    } else {
        theme::secondary_button
    })
    .on_press(Message::CreateTableTabSelected(tab))
    .into()
}

fn create_table_tab_content<'a>(
    state: &'a Akt,
    tab: CreateTableTab,
    draft: &'a CreateTableDraft,
) -> Element<'a, Message> {
    let language = state.language();
    let driver = state.connection_manager().form().driver;
    match tab {
        CreateTableTab::Columns => create_table_columns_panel(language, driver, draft),
        CreateTableTab::Indexes => create_table_indexes_panel(language, driver, draft),
        CreateTableTab::Constraints => create_table_constraints_panel(language, driver, draft),
        CreateTableTab::Sql => sql_preview_block(
            language,
            i18n::texts(language).create_table_statement,
            state.create_table_sql_preview(draft),
            Message::CopyCreateTableSql,
            220,
        ),
    }
}

fn create_table_columns_panel<'a>(
    language: Language,
    driver: DatabaseDriver,
    draft: &'a CreateTableDraft,
) -> Element<'a, Message> {
    if driver == DatabaseDriver::MongoDb {
        return Space::with_height(0).into();
    }

    let rows = draft.columns().iter().enumerate().fold(
        column![create_table_columns_header(language)].spacing(6),
        |column, (index, column_draft)| {
            column.push(create_table_column_row(driver, index, column_draft))
        },
    );

    container(
        column![
            row![
                detail_section_label(i18n::texts(language).columns),
                Space::with_width(Length::Fill),
                small_command_button(
                    i18n::texts(language).add_column,
                    Message::AddCreateTableColumn
                ),
            ]
            .align_y(Alignment::Center),
            rows,
        ]
        .spacing(8),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn create_table_columns_header(language: Language) -> Element<'static, Message> {
    let texts = i18n::texts(language);
    row![
        grid_header(texts.column, 2),
        grid_header(texts.data_type, 2),
        grid_header(texts.nullable, 1),
        grid_header(texts.default_value, 2),
        grid_header(texts.extra_clause, 2),
        Space::with_width(132),
    ]
    .spacing(8)
    .into()
}

fn create_table_column_row<'a>(
    driver: DatabaseDriver,
    index: usize,
    column_draft: &'a CreateTableColumnDraft,
) -> Element<'a, Message> {
    row![
        form_slot(
            create_table_column_input(
                "id",
                column_draft.name(),
                index,
                CreateTableColumnField::Name,
            ),
            2,
        ),
        form_slot(
            create_table_column_type_pick_list(driver, column_draft.data_type(), index),
            2,
        ),
        form_slot(
            create_table_column_input(
                "NO",
                column_draft.nullable(),
                index,
                CreateTableColumnField::Nullable,
            ),
            1,
        ),
        form_slot(
            create_table_column_input(
                "CURRENT_TIMESTAMP",
                column_draft.default_value(),
                index,
                CreateTableColumnField::DefaultValue,
            ),
            2,
        ),
        form_slot(
            create_table_column_input(
                "AUTO_INCREMENT",
                column_draft.extra(),
                index,
                CreateTableColumnField::Extra,
            ),
            2,
        ),
        row![
            icon_command_button("+", Message::InsertCreateTableColumnAfter(index)),
            icon_command_button("#", Message::AddCreateTableIndexForColumn(index)),
            icon_command_button("↑", Message::MoveCreateTableColumn(index, -1)),
            icon_command_button("↓", Message::MoveCreateTableColumn(index, 1)),
            small_danger_button("-", Message::RemoveCreateTableColumn(index)),
        ]
        .spacing(4)
        .align_y(Alignment::Center),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn create_table_indexes_panel<'a>(
    language: Language,
    driver: DatabaseDriver,
    draft: &'a CreateTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(language);
    let rows = draft.indexes().iter().enumerate().fold(
        column![row![
            grid_header(texts.index, 2),
            grid_header(texts.index_type, 2),
            grid_header(texts.index_columns, 3),
            Space::with_width(46),
        ]
        .spacing(8)]
        .spacing(6),
        |column, (index, index_draft)| {
            column.push(create_table_index_row(driver, draft, index, index_draft))
        },
    );
    let title = if driver == DatabaseDriver::MongoDb {
        texts.indexes
    } else {
        texts.indexes
    };

    container(
        column![
            row![
                detail_section_label(title),
                Space::with_width(Length::Fill),
                small_command_button(texts.add_index, Message::AddCreateTableIndex),
            ]
            .align_y(Alignment::Center),
            rows,
        ]
        .spacing(8),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn create_table_index_row<'a>(
    driver: DatabaseDriver,
    draft: &'a CreateTableDraft,
    index: usize,
    index_draft: &'a CreateTableIndexDraft,
) -> Element<'a, Message> {
    container(
        column![
            row![
                form_slot(
                    create_table_index_input(
                        "idx_name",
                        index_draft.name(),
                        index,
                        CreateTableIndexField::Name,
                    ),
                    2,
                ),
                form_slot(
                    create_table_index_type_pick_list(driver, index_draft.index_type(), index),
                    2,
                ),
                form_slot(
                    create_table_index_input(
                        "name, created_at",
                        index_draft.columns(),
                        index,
                        CreateTableIndexField::Columns,
                    ),
                    3,
                ),
                small_danger_button("-", Message::RemoveCreateTableIndex(index)),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            create_table_index_column_picker(draft, index),
        ]
        .spacing(6),
    )
    .style(theme::panel_flat)
    .padding(8)
    .width(Length::Fill)
    .into()
}

fn create_table_index_column_picker<'a>(
    draft: &'a CreateTableDraft,
    index: usize,
) -> Element<'a, Message> {
    let chips = draft
        .columns()
        .iter()
        .filter(|column| !column.name().trim().is_empty())
        .fold(
            row![].spacing(5).align_y(Alignment::Center),
            |row, column| {
                row.push(index_column_chip(
                    column.name().to_owned(),
                    Message::AppendCreateTableIndexColumn(index, column.name().to_owned()),
                ))
            },
        );

    scrollable(chips)
        .direction(ScrollDirection::Horizontal(Scrollbar::new()))
        .height(32)
        .style(theme::scrollable)
        .into()
}

fn create_table_constraints_panel<'a>(
    language: Language,
    driver: DatabaseDriver,
    draft: &'a CreateTableDraft,
) -> Element<'a, Message> {
    if driver == DatabaseDriver::MongoDb {
        return Space::with_height(0).into();
    }

    let rows = draft.constraints().iter().enumerate().fold(
        column![row![
            grid_header("CONSTRAINT", 2),
            grid_header("KIND", 2),
            grid_header("EXPRESSION", 4),
            Space::with_width(46),
        ]
        .spacing(8)]
        .spacing(6),
        |column, (index, constraint)| column.push(create_table_constraint_row(index, constraint)),
    );

    container(
        column![
            row![
                detail_section_label(i18n::texts(language).constraints),
                Space::with_width(Length::Fill),
                small_command_button("+ CONSTRAINT", Message::AddCreateTableConstraint),
            ]
            .align_y(Alignment::Center),
            rows,
        ]
        .spacing(8),
    )
    .style(theme::panel_low)
    .padding(10)
    .width(Length::Fill)
    .into()
}

fn create_table_constraint_row<'a>(
    index: usize,
    constraint: &'a CreateTableConstraintDraft,
) -> Element<'a, Message> {
    row![
        form_slot(
            create_table_constraint_input(
                "pk_table",
                constraint.name(),
                index,
                CreateTableConstraintField::Name,
            ),
            2,
        ),
        form_slot(
            create_table_constraint_input(
                "PRIMARY KEY",
                constraint.kind(),
                index,
                CreateTableConstraintField::Kind,
            ),
            2,
        ),
        form_slot(
            create_table_constraint_input(
                "id / status in ('active')",
                constraint.expression(),
                index,
                CreateTableConstraintField::Expression,
            ),
            4,
        ),
        small_danger_button("-", Message::RemoveCreateTableConstraint(index)),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}
