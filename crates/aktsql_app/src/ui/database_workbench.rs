use super::*;
use iced::widget::column;

#[derive(Clone, Copy)]
struct TableActionSpec {
    label: &'static str,
    action: TableAction,
    danger: bool,
}

pub(in crate::ui) fn query_sidebar_schema(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let form = state.connection_manager().form();
    let has_database_objects = state
        .query_workspace()
        .schema_objects()
        .iter()
        .any(|object| object.kind == SchemaObjectKind::Database);
    let tree_title = if form.database.trim().is_empty() || has_database_objects {
        texts.section_databases
    } else {
        texts.section_tables
    };
    let tree_detail = form
        .database
        .trim()
        .is_empty()
        .then(|| texts.select_database_prompt.to_owned());
    let schema_items = if state.query_workspace().schema_objects().is_empty() {
        column![text(state.query_workspace().schema_message())
            .size(11)
            .wrapping(Wrapping::WordOrGlyph)
            .style(theme::secondary_text),]
        .spacing(7)
    } else {
        state
            .query_workspace()
            .schema_objects()
            .iter()
            .enumerate()
            .fold(
                (column![].spacing(3), false),
                |(column, parent_expanded), (index, object)| match object.kind {
                    SchemaObjectKind::Database => (
                        column.push(query_tree_object_with_menu(
                            state,
                            index,
                            object,
                            form.database.trim(),
                        )),
                        false,
                    ),
                    SchemaObjectKind::Table
                    | SchemaObjectKind::View
                    | SchemaObjectKind::Index
                    | SchemaObjectKind::Collection => {
                        let expanded = state.schema_object_expanded(index);
                        (
                            column.push(query_tree_object_with_menu(
                                state,
                                index,
                                object,
                                form.database.trim(),
                            )),
                            expanded,
                        )
                    }
                    SchemaObjectKind::Column => {
                        if parent_expanded {
                            (
                                column.push(query_tree_object_with_menu(
                                    state,
                                    index,
                                    object,
                                    form.database.trim(),
                                )),
                                parent_expanded,
                            )
                        } else {
                            (column, parent_expanded)
                        }
                    }
                },
            )
            .0
    };

    let mut tree = column![row![
        text(tree_title.to_uppercase())
            .size(10)
            .wrapping(Wrapping::None)
            .style(theme::primary_text),
        Space::with_width(Length::Fill),
        schema_refresh_button(state, texts.refresh),
    ]
    .align_y(Alignment::Center),]
    .spacing(8)
    .height(Length::Fill);

    if let Some(detail) = tree_detail {
        tree = tree.push(
            text(detail)
                .size(10)
                .wrapping(Wrapping::WordOrGlyph)
                .style(theme::secondary_text),
        );
    }

    tree = tree.push(
        scrollable(container(schema_items).padding([2, 0]).width(Length::Fill))
            .height(Length::Fill)
            .style(theme::scrollable),
    );

    container(tree)
        .style(theme::panel_low)
        .padding([10, 10])
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

pub(in crate::ui) fn database_workbench_view(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let form = state.connection_manager().form();
    let has_database_context =
        form.driver == DatabaseDriver::Sqlite || !form.database.trim().is_empty();
    let inspector_active = state.create_table_draft().is_some()
        || state.alter_table_draft().is_some()
        || state.table_detail_target().is_some()
        || state.database_detail_target().is_some();
    let database_label = if form.database.trim().is_empty() {
        texts.select_database_prompt.to_owned()
    } else {
        form.database.clone()
    };
    let schema_objects = state.query_workspace().schema_objects();
    let table_count = schema_objects
        .iter()
        .filter(|object| {
            matches!(
                object.kind,
                SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
            )
        })
        .count();
    let database_count = schema_objects
        .iter()
        .filter(|object| object.kind == SchemaObjectKind::Database)
        .count();
    let column_count = schema_objects
        .iter()
        .filter(|object| object.kind == SchemaObjectKind::Column)
        .count();

    let mut content = column![database_workbench_header(state), horizontal_separator(),]
        .spacing(14)
        .height(Length::Fill)
        .padding(18);

    if !inspector_active {
        content = content.push(
            row![
                database_metric_card(
                    texts.current_database,
                    database_label,
                    texts.database.to_uppercase(),
                ),
                database_metric_card(
                    texts.schema_objects,
                    format!("{}", schema_objects.len()),
                    format!(
                        "{} {} / {} {} / {} {}",
                        database_count,
                        texts.section_databases,
                        table_count,
                        texts.section_tables,
                        column_count,
                        texts.schema
                    ),
                ),
            ]
            .spacing(14)
            .width(Length::Fill),
        );
    }

    content = content.push(database_workbench_inspector(state));

    content = if inspector_active {
        content
    } else if has_database_context {
        content.push(database_table_list(state))
    } else {
        content.push(database_context_empty_state(state))
    };

    container(content)
        .style(theme::workspace)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn database_context_empty_state(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());

    container(
        column![
            text(texts.section_databases.to_uppercase())
                .size(11)
                .wrapping(Wrapping::None)
                .style(theme::primary_text),
            text(texts.select_database_prompt)
                .size(13)
                .wrapping(Wrapping::WordOrGlyph)
                .style(theme::secondary_text),
        ]
        .spacing(10),
    )
    .style(theme::query_workbench)
    .padding([14, 16])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn database_workbench_inspector(state: &Akt) -> Element<'_, Message> {
    if let Some(draft) = state.create_table_draft() {
        return container(
            scrollable(schema_designer::create_table_designer_panel(state, draft))
                .height(Length::Fill)
                .style(theme::scrollable),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into();
    }

    if let Some(draft) = state.alter_table_draft() {
        return container(
            scrollable(schema_designer::alter_table_designer_panel(state, draft))
                .height(Length::Fill)
                .style(theme::scrollable),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into();
    }

    if let Some(target) = state.table_detail_target() {
        return container(
            scrollable(table_detail_panel(state, target))
                .height(Length::Fill)
                .style(theme::scrollable),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into();
    }

    if let Some(target) = state.database_detail_target() {
        return container(
            scrollable(database_detail_panel(state, target))
                .height(Length::Fill)
                .style(theme::scrollable),
        )
        .height(Length::Fill)
        .width(Length::Fill)
        .into();
    }

    Space::with_height(0).into()
}

fn database_detail_panel<'a>(state: &'a Akt, target: &'a str) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let title = if state.database_detail_loading() {
        texts.database_details_loading.to_owned()
    } else {
        texts.database_details_title.to_owned()
    };
    let details = state
        .database_details()
        .filter(|details| details.database == target || details.driver == DatabaseDriver::Sqlite);

    container(
        column![
            text(title)
                .size(12)
                .wrapping(Wrapping::None)
                .style(theme::primary_text),
            details
                .map(|details| live_database_detail_sections(state, details))
                .unwrap_or_else(|| pending_database_detail_sections(state, target)),
        ]
        .spacing(10),
    )
    .style(theme::panel_low)
    .padding([12, 14])
    .width(Length::Fill)
    .into()
}

fn live_database_detail_sections<'a>(
    state: &'a Akt,
    details: &'a DatabaseDetails,
) -> Element<'a, Message> {
    details
        .sections
        .iter()
        .enumerate()
        .fold(column![].spacing(10), |column, (index, section)| {
            column.push(database_detail_section(
                index,
                database_detail_section_title(state, section.kind),
                section.fields.iter().map(|field| {
                    (
                        std::borrow::Cow::Borrowed(field.label.as_str()),
                        std::borrow::Cow::Borrowed(field.value.as_str()),
                    )
                }),
            ))
        })
        .into()
}

fn pending_database_detail_sections(state: &Akt, target: &str) -> Element<'static, Message> {
    let texts = i18n::texts(state.language());
    let form = state.connection_manager().form();

    column![database_detail_section(
        0,
        texts.database_detail_core,
        vec![
            (
                std::borrow::Cow::Borrowed(texts.database),
                std::borrow::Cow::Owned(target.to_owned())
            ),
            (
                std::borrow::Cow::Borrowed(texts.driver),
                std::borrow::Cow::Owned(form.driver.to_string())
            ),
        ],
    )]
    .spacing(10)
    .into()
}

fn database_detail_section_title(state: &Akt, kind: DatabaseDetailSectionKind) -> &'static str {
    let texts = i18n::texts(state.language());
    match kind {
        DatabaseDetailSectionKind::Core => texts.database_detail_core,
        DatabaseDetailSectionKind::Storage => texts.database_detail_storage,
        DatabaseDetailSectionKind::Objects => texts.database_detail_objects,
        DatabaseDetailSectionKind::Runtime => texts.database_detail_runtime,
    }
}

fn database_detail_section<'a, I>(
    index: usize,
    title: &'static str,
    fields: I,
) -> Element<'a, Message>
where
    I: IntoIterator<Item = (std::borrow::Cow<'a, str>, std::borrow::Cow<'a, str>)>,
{
    let detail_rows = fields.into_iter().fold(
        row![].spacing(10).width(Length::Fill),
        |row, (label, value)| row.push(readonly_database_detail(label, value)),
    );

    row![
        container(Space::new(3, Length::Fill))
            .style(theme::detail_category_marker(index))
            .width(3)
            .height(Length::Fill),
        column![
            text(title)
                .size(10)
                .wrapping(Wrapping::None)
                .style(theme::primary_text),
            detail_rows,
        ]
        .spacing(7)
        .width(Length::Fill),
    ]
    .spacing(10)
    .height(54)
    .into()
}

fn readonly_database_detail<'a>(
    label: std::borrow::Cow<'a, str>,
    value: std::borrow::Cow<'a, str>,
) -> Element<'a, Message> {
    container(
        column![
            text(label.to_uppercase())
                .size(9)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
            text(value)
                .size(12)
                .wrapping(Wrapping::None)
                .style(theme::on_surface_text),
        ]
        .spacing(4),
    )
    .width(Length::FillPortion(1))
    .into()
}

fn table_detail_panel<'a>(state: &'a Akt, target: &'a str) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let title = if state.table_detail_loading() {
        texts.table_details_loading.to_owned()
    } else {
        texts.table_details_title.to_owned()
    };
    let details = state
        .table_details()
        .filter(|details| details.table == target || details.table.ends_with(target));

    container(
        column![
            row![
                text(title)
                    .size(12)
                    .wrapping(Wrapping::None)
                    .style(theme::primary_text),
                Space::with_width(Length::Fill),
                text(
                    details
                        .map(|details| details.driver.to_string())
                        .unwrap_or_else(|| state.connection_manager().form().driver.to_string())
                        .to_uppercase()
                )
                .size(10)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
            ]
            .align_y(Alignment::Center),
            details
                .map(|details| live_table_detail_sections(state, details))
                .unwrap_or_else(|| pending_table_detail_sections(state, target)),
        ]
        .spacing(10),
    )
    .style(theme::panel_low)
    .padding([12, 14])
    .width(Length::Fill)
    .into()
}

fn live_table_detail_sections<'a>(
    state: &'a Akt,
    details: &'a TableDetails,
) -> Element<'a, Message> {
    let mut content = details.sections.iter().enumerate().fold(
        column![].spacing(10),
        |column, (index, section)| {
            column.push(database_detail_section(
                index,
                database_detail_section_title(state, section.kind),
                section.fields.iter().map(|field| {
                    (
                        std::borrow::Cow::Borrowed(field.label.as_str()),
                        std::borrow::Cow::Borrowed(field.value.as_str()),
                    )
                }),
            ))
        },
    );

    content = content.push(table_columns_panel(state.language(), details));
    content = content.push(table_indexes_panel(state.language(), details));
    content = content.push(table_create_statement_panel(state.language(), details));
    content.into()
}

fn pending_table_detail_sections(state: &Akt, target: &str) -> Element<'static, Message> {
    let texts = i18n::texts(state.language());
    column![database_detail_section(
        0,
        texts.database_detail_core,
        vec![
            (
                std::borrow::Cow::Borrowed(texts.section_tables),
                std::borrow::Cow::Owned(target.to_owned())
            ),
            (
                std::borrow::Cow::Borrowed(texts.driver),
                std::borrow::Cow::Owned(state.connection_manager().form().driver.to_string())
            ),
            (
                std::borrow::Cow::Borrowed(texts.database),
                std::borrow::Cow::Owned(state.connection_manager().form().database.clone())
            ),
        ],
    )]
    .spacing(10)
    .into()
}

fn table_columns_panel<'a>(language: Language, details: &'a TableDetails) -> Element<'a, Message> {
    let texts = i18n::texts(language);
    let header = row![
        result_header_cell(String::from(texts.column)),
        result_header_cell(String::from(texts.data_type)),
        result_header_cell(String::from(texts.nullable)),
        result_header_cell(String::from(texts.default_value)),
        result_header_cell(String::from(texts.extra_clause)),
    ]
    .spacing(0);
    let rows = details.columns.iter().enumerate().fold(
        column![].spacing(0),
        |column, (row_index, column_detail)| {
            column.push(
                row![
                    result_data_cell(&column_detail.name, row_index),
                    result_data_cell(&column_detail.data_type, row_index),
                    result_data_cell(&column_detail.nullable, row_index),
                    result_data_cell(&column_detail.default_value, row_index),
                    result_data_cell(&column_detail.extra, row_index),
                ]
                .spacing(0),
            )
        },
    );

    let grid = column![header, rows].width(RESULT_CELL_WIDTH * 5.0);

    container(column![
        detail_section_label(texts.columns),
        scrollable(grid).height(120).style(theme::scrollable)
    ])
    .style(theme::query_canvas)
    .padding(8)
    .width(Length::Fill)
    .into()
}

fn table_indexes_panel<'a>(language: Language, details: &'a TableDetails) -> Element<'a, Message> {
    let texts = i18n::texts(language);
    let header = row![
        result_header_cell(String::from(texts.index)),
        result_header_cell(String::from(texts.index_columns)),
        result_header_cell(String::from(texts.unique_index)),
    ]
    .spacing(0);
    let rows = details.indexes.iter().enumerate().fold(
        column![].spacing(0),
        |column, (row_index, index)| {
            column.push(
                row![
                    result_data_cell(&index.name, row_index),
                    result_data_cell(&index.columns, row_index),
                    result_data_cell(&index.unique, row_index),
                ]
                .spacing(0),
            )
        },
    );

    let grid = column![header, rows].width(RESULT_CELL_WIDTH * 3.0);

    container(column![
        detail_section_label(texts.indexes),
        scrollable(grid).height(96).style(theme::scrollable)
    ])
    .style(theme::query_canvas)
    .padding(8)
    .width(Length::Fill)
    .into()
}

fn table_create_statement_panel(
    language: Language,
    details: &TableDetails,
) -> Element<'static, Message> {
    let texts = i18n::texts(language);
    container(
        column![
            detail_section_label(texts.create_table_statement),
            scrollable(
                text(details.create_statement.clone())
                    .size(11)
                    .wrapping(Wrapping::WordOrGlyph)
                    .style(theme::on_surface_text)
            )
            .height(110)
            .style(theme::scrollable),
        ]
        .spacing(6),
    )
    .style(theme::query_canvas)
    .padding(10)
    .width(Length::Fill)
    .into()
}

pub(in crate::ui) fn detail_section_label(label: &'static str) -> Element<'static, Message> {
    text(label)
        .size(10)
        .wrapping(Wrapping::None)
        .style(theme::primary_text)
        .into()
}

fn database_workbench_header(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let form = state.connection_manager().form();
    let has_database = form.driver == DatabaseDriver::Sqlite || !form.database.trim().is_empty();

    row![
        column![
            text(texts.database_workbench)
                .size(28)
                .wrapping(Wrapping::None)
                .style(theme::on_surface_text),
            text(format!(
                "{} / {}",
                form.driver,
                state.connection_manager().active_label()
            ))
            .size(12)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        ]
        .spacing(4)
        .width(Length::Fill),
        column![
            text(texts.database_actions.to_uppercase())
                .size(10)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
            row![
                database_action_button(texts.create_database, DatabaseAction::CreateDatabase, true),
                database_action_button(
                    texts.edit_database_charset,
                    DatabaseAction::AlterDatabaseCharset,
                    has_database,
                ),
                database_action_button(
                    texts.drop_database,
                    DatabaseAction::DropDatabase,
                    has_database
                ),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
        ]
        .spacing(6),
        schema_refresh_button(state, texts.refresh_schema),
    ]
    .align_y(Alignment::Center)
    .spacing(16)
    .into()
}

fn database_action_button(
    label: &'static str,
    action: DatabaseAction,
    enabled: bool,
) -> Element<'static, Message> {
    let mut button = button(
        text(label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .height(28)
    .padding([6, 9])
    .style(if matches!(action, DatabaseAction::DropDatabase) {
        theme::danger_button
    } else if enabled {
        theme::primary_button
    } else {
        theme::secondary_button
    });

    if enabled {
        button = button.on_press(Message::RunDatabaseAction(action));
    }

    button.into()
}

fn database_metric_card(
    title: &'static str,
    value: String,
    detail: String,
) -> Element<'static, Message> {
    container(
        column![
            text(title.to_uppercase())
                .size(10)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
            text(value)
                .size(16)
                .wrapping(Wrapping::WordOrGlyph)
                .style(theme::on_surface_text),
            text(detail)
                .size(11)
                .wrapping(Wrapping::WordOrGlyph)
                .style(theme::secondary_text),
        ]
        .spacing(7),
    )
    .style(theme::panel_low)
    .width(Length::FillPortion(1))
    .padding([14, 16])
    .into()
}

fn database_table_list(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let form = state.connection_manager().form();
    let tables = state
        .query_workspace()
        .schema_objects()
        .iter()
        .enumerate()
        .filter(|(_, object)| {
            matches!(
                object.kind,
                SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
            )
        })
        .fold(column![].spacing(8), |column, (index, object)| {
            column.push(database_table_row(index, object, state.language()))
        });
    let content: Element<'_, Message> = if form.database.trim().is_empty() {
        container(
            text(texts.select_database_prompt)
                .size(13)
                .wrapping(Wrapping::WordOrGlyph)
                .style(theme::secondary_text),
        )
        .style(theme::panel_low)
        .padding(16)
        .width(Length::Fill)
        .into()
    } else if state.query_workspace().schema_objects().is_empty() {
        container(
            column![
                text(state.query_workspace().schema_message())
                    .size(13)
                    .wrapping(Wrapping::WordOrGlyph)
                    .style(theme::secondary_text),
                schema_refresh_button(state, texts.refresh_schema),
            ]
            .spacing(10),
        )
        .style(theme::panel_low)
        .padding(16)
        .width(Length::Fill)
        .into()
    } else {
        scrollable(container(tables).padding([2, 0]).width(Length::Fill))
            .height(Length::Fill)
            .style(theme::scrollable)
            .into()
    };

    container(
        column![
            row![
                text(texts.section_tables.to_uppercase())
                    .size(11)
                    .wrapping(Wrapping::None)
                    .style(theme::primary_text),
                Space::with_width(Length::Fill),
                create_table_button(
                    texts.create_table,
                    form.driver == DatabaseDriver::Sqlite || !form.database.trim().is_empty(),
                ),
                text(texts.table_actions.to_uppercase())
                    .size(11)
                    .wrapping(Wrapping::None)
                    .style(theme::secondary_text),
            ]
            .align_y(Alignment::Center),
            content,
        ]
        .spacing(10),
    )
    .style(theme::query_workbench)
    .padding([14, 16])
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn create_table_button(label: &'static str, enabled: bool) -> Element<'static, Message> {
    let mut action = button(
        text(label)
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .height(28)
    .padding([6, 10])
    .style(if enabled {
        theme::primary_button
    } else {
        theme::secondary_button
    });

    if enabled {
        action = action.on_press(Message::RequestCreateTable);
    }

    action.into()
}

fn database_table_row(
    index: usize,
    object: &SchemaObject,
    language: Language,
) -> Element<'static, Message> {
    let actions = table_action_specs(language)
        .into_iter()
        .fold(row![].spacing(8).align_y(Alignment::Center), |row, spec| {
            row.push(table_action_button(spec, index))
        });

    container(
        row![
            column![
                text(object.display_label())
                    .size(14)
                    .wrapping(Wrapping::None)
                    .style(theme::on_surface_text),
                text(object.sql_preview())
                    .size(11)
                    .wrapping(Wrapping::WordOrGlyph)
                    .style(theme::secondary_text),
            ]
            .spacing(4)
            .width(Length::Fill),
            actions,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .style(theme::panel_low)
    .padding([10, 12])
    .width(Length::Fill)
    .into()
}

fn table_action_button(spec: TableActionSpec, index: usize) -> Element<'static, Message> {
    button(
        text(spec.label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .height(26)
    .padding([5, 8])
    .style(if spec.danger {
        theme::danger_button
    } else {
        theme::secondary_button
    })
    .on_press(Message::RunTableAction(spec.action, index))
    .into()
}

fn query_tree_object_with_menu(
    state: &Akt,
    index: usize,
    object: &SchemaObject,
    selected_database: &str,
) -> Element<'static, Message> {
    let mut item = column![query_tree_object(state, index, object, selected_database)].spacing(4);

    if state.schema_object_menu_open(index) {
        item = item.push(query_tree_context_menu(state, index, object));
    }

    item.into()
}

fn query_tree_object(
    state: &Akt,
    index: usize,
    object: &SchemaObject,
    selected_database: &str,
) -> Element<'static, Message> {
    let depth = object.depth() as u16;
    let active = object.kind == SchemaObjectKind::Database
        && !selected_database.is_empty()
        && object.name == selected_database;
    let expanded = state.schema_object_expanded(index);
    let marker = query_tree_marker(object.kind, active, expanded);
    let kind_label = query_tree_kind_label(object.kind, state.language());
    let message = if matches!(
        object.kind,
        SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
    ) {
        Message::ToggleSchemaObject(index)
    } else {
        Message::UseQuerySchemaObject(index)
    };

    row![
        Space::with_width(2.0 + (depth as f32) * 16.0),
        mouse_area(
            button(
                text(format!("{}  {}  {}", marker, kind_label, object.name))
                    .size(if object.kind == SchemaObjectKind::Database {
                        12
                    } else {
                        11
                    })
                    .wrapping(Wrapping::None)
            )
            .width(Length::Fill)
            .padding([4, 8])
            .style(theme::sidebar_button(active))
            .on_press(message),
        )
        .on_right_press(Message::ToggleSchemaObjectMenu(index)),
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .into()
}

fn query_tree_kind_label(kind: SchemaObjectKind, language: Language) -> &'static str {
    let texts = i18n::texts(language);
    match kind {
        SchemaObjectKind::Database => texts.database_node,
        SchemaObjectKind::Table => texts.table_node,
        SchemaObjectKind::View => texts.view_node,
        SchemaObjectKind::Index => texts.index_node,
        SchemaObjectKind::Collection => texts.collection_node,
        SchemaObjectKind::Column => texts.field_node,
    }
}

fn query_tree_context_menu(
    state: &Akt,
    index: usize,
    object: &SchemaObject,
) -> Element<'static, Message> {
    let depth = object.depth() as f32;
    let actions = match object.kind {
        SchemaObjectKind::Database => database_tree_actions(index, state.language()),
        SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection => {
            table_tree_actions(index, state.language())
        }
        _ => column![].spacing(4),
    };

    row![
        Space::with_width(24.0 + depth * 16.0),
        container(actions)
            .style(theme::panel_low)
            .padding([7, 8])
            .width(220),
    ]
    .into()
}

fn database_tree_actions(
    index: usize,
    language: Language,
) -> iced::widget::Column<'static, Message> {
    let texts = i18n::texts(language);

    column![
        tree_action_button(
            texts.create_table,
            Message::RunDatabaseObjectAction(DatabaseAction::CreateTable, index),
            false,
        ),
        tree_action_button(
            texts.view_database,
            Message::RunDatabaseObjectAction(DatabaseAction::ShowDatabase, index),
            false,
        ),
        tree_action_button(
            texts.rename_database,
            Message::RunDatabaseObjectAction(DatabaseAction::AlterDatabaseName, index),
            false,
        ),
        tree_action_button(
            texts.edit_database_charset,
            Message::RunDatabaseObjectAction(DatabaseAction::AlterDatabaseCharset, index),
            false,
        ),
        tree_action_button(
            texts.drop_database,
            Message::RunDatabaseObjectAction(DatabaseAction::DropDatabase, index),
            true,
        ),
    ]
    .spacing(4)
}

fn table_tree_actions(index: usize, language: Language) -> iced::widget::Column<'static, Message> {
    table_action_specs(language)
        .into_iter()
        .fold(column![].spacing(4), |column, spec| {
            column.push(tree_action_button(
                spec.label,
                Message::RunTableAction(spec.action, index),
                spec.danger,
            ))
        })
}

fn table_action_specs(language: Language) -> Vec<TableActionSpec> {
    let texts = i18n::texts(language);

    vec![
        TableActionSpec {
            label: texts.browse_data,
            action: TableAction::SelectRows,
            danger: false,
        },
        TableActionSpec {
            label: texts.design_structure,
            action: TableAction::DescribeTable,
            danger: false,
        },
        TableActionSpec {
            label: texts.rename_table,
            action: TableAction::RenameTable,
            danger: false,
        },
        TableActionSpec {
            label: texts.modify_structure,
            action: TableAction::AlterTable,
            danger: false,
        },
        TableActionSpec {
            label: texts.truncate_table,
            action: TableAction::TruncateTable,
            danger: true,
        },
        TableActionSpec {
            label: texts.delete_table,
            action: TableAction::DropTable,
            danger: true,
        },
    ]
}

fn tree_action_button(
    label: &'static str,
    message: Message,
    danger: bool,
) -> Element<'static, Message> {
    button(
        text(label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .width(Length::Fill)
    .height(24)
    .padding([3, 8])
    .style(if danger {
        theme::danger_button
    } else {
        theme::secondary_button
    })
    .on_press(message)
    .into()
}

fn query_tree_marker(kind: SchemaObjectKind, active: bool, expanded: bool) -> &'static str {
    match kind {
        SchemaObjectKind::Database if active => "v",
        SchemaObjectKind::Table | SchemaObjectKind::View | SchemaObjectKind::Collection
            if expanded =>
        {
            "v"
        }
        SchemaObjectKind::Database
        | SchemaObjectKind::Table
        | SchemaObjectKind::View
        | SchemaObjectKind::Index
        | SchemaObjectKind::Collection => ">",
        SchemaObjectKind::Column => "-",
    }
}
