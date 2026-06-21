use super::*;
use iced::widget::column;

pub(in crate::ui) fn query_workspace_view(state: &Akt) -> Element<'_, Message> {
    let body = if state.result_focus() {
        column![result_panel(state)].height(Length::Fill)
    } else {
        column![
            query_editor_panel(state),
            horizontal_separator(),
            result_panel(state)
        ]
        .height(Length::Fill)
    };

    container(
        column![
            query_context_bar(state),
            horizontal_separator(),
            body,
            query_workspace_footer(state),
        ]
        .height(Length::Fill),
    )
    .style(theme::query_workbench)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn query_context_bar(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let form = state.connection_manager().form();
    let has_active_connection = state.connection_manager().selected_profile_id().is_some();
    let schema = if form.database.trim().is_empty() {
        "--"
    } else {
        form.database.as_str()
    };
    let target = if has_active_connection {
        driver_context_label(form)
    } else {
        String::from("NO ACTIVE CONNECTION")
    };
    let driver = if has_active_connection {
        form.driver.to_string()
    } else {
        String::from("--")
    };

    container(
        row![
            text("<>").size(14).style(theme::primary_text),
            text(texts.query_file_name)
                .size(14)
                .style(theme::on_surface_text),
            chip(target, false),
            Space::with_width(Length::Fill),
            text(format!("{}: {}", texts.database_short, driver).to_uppercase())
                .size(11)
                .style(theme::secondary_text),
            text(format!("{}: {}", texts.schema, schema).to_uppercase())
                .size(11)
                .style(theme::secondary_text),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .height(QUERY_CONTEXT_HEIGHT)
    .padding([8, 20])
    .style(theme::query_header)
    .width(Length::Fill)
    .into()
}

fn driver_context_label(form: &ConnectionForm) -> String {
    let target = if form.driver.requires_port() {
        format!("{}:{}", form.location.trim(), form.port.trim())
    } else {
        form.location.trim().to_owned()
    };

    target.to_uppercase()
}

pub(in crate::ui) fn schema_refresh_button(
    state: &Akt,
    label: &'static str,
) -> Element<'static, Message> {
    let mut action = button(
        text(if state.schema_loading() {
            i18n::texts(state.language()).loading.to_uppercase()
        } else {
            label.to_uppercase()
        })
        .size(10),
    )
    .width(78)
    .padding([6, 8])
    .style(theme::secondary_button);

    if !state.schema_loading() {
        action = action.on_press(Message::RefreshQuerySchema);
    }

    action.into()
}

fn query_editor_panel(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());

    container(
        column![
            container(
                row![
                    text("SQL").size(11).style(theme::primary_text),
                    chip(texts.ready.to_uppercase(), false),
                    Space::with_width(Length::Fill),
                    editor_tool_button(texts.new_query, Message::NewQueryDraft),
                    editor_tool_button(texts.example_query, Message::LoadQueryExample),
                    editor_tool_button(texts.format_query, Message::FormatQuery),
                    editor_tool_button(texts.clear_query, Message::ClearQuery),
                    editor_tool_button(texts.refresh_schema, Message::RefreshQuerySchema),
                    editor_execute_button(state),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 14])
            .width(Length::Fill)
            .style(theme::query_header),
            container(
                row![
                    query_line_numbers(state.query_editor().line_count()),
                    text_editor(state.query_editor())
                        .placeholder(query_editor_placeholder(
                            state.connection_manager().form().driver
                        ))
                        .on_action(Message::QueryEditorAction)
                        .height(Length::Fill)
                        .padding(14)
                        .size(14)
                        .style(theme::text_editor),
                ]
                .height(Length::Fill)
                .spacing(0),
            )
            .style(theme::query_canvas)
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .height(Length::Fill),
    )
    .style(theme::query_canvas)
    .width(Length::Fill)
    .height(QUERY_EDITOR_HEIGHT)
    .into()
}

fn query_editor_placeholder(driver: DatabaseDriver) -> &'static str {
    match driver {
        DatabaseDriver::MongoDb => "{ \"listCollections\": 1 }",
        DatabaseDriver::Sqlite => "select * from sqlite_master limit 100;",
        _ => "select 1 as ok;",
    }
}

fn editor_tool_button(label: &'static str, message: Message) -> Element<'static, Message> {
    button(text(label.to_uppercase()).size(10))
        .height(24)
        .padding([5, 8])
        .style(theme::secondary_button)
        .on_press(message)
        .into()
}

fn editor_execute_button(state: &Akt) -> Element<'static, Message> {
    let has_connection = state.connection_manager().selected_profile_id().is_some();
    let has_sql = !state.query_workspace().sql().trim().is_empty();
    let enabled = has_connection && has_sql && !state.query_running();
    let label = if !has_connection {
        i18n::texts(state.language()).connect
    } else if state.query_running() {
        i18n::texts(state.language()).running
    } else {
        i18n::texts(state.language()).execute
    };

    let mut action = button(text(label.to_uppercase()).size(10))
        .width(78)
        .height(24)
        .padding([5, 8])
        .style(if enabled {
            theme::primary_button
        } else {
            theme::secondary_button
        });

    if enabled {
        action = action.on_press(Message::ExecuteQuery);
    }

    action.into()
}

fn query_line_numbers(line_count: usize) -> Element<'static, Message> {
    let content = (1..=line_count.max(4))
        .take(99)
        .fold(column![].spacing(7), |column, line| {
            column.push(text(line.to_string()).size(12).style(theme::secondary_text))
        });

    container(content)
        .style(theme::query_gutter)
        .width(QUERY_LINE_GUTTER_WIDTH)
        .height(Length::Fill)
        .padding([14, 12])
        .align_x(Horizontal::Right)
        .into()
}

fn result_panel(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let result = state.query_workspace().last_result();
    let result_body = match result {
        Some(result) => result_grid(result, state.language(), state.result_order_by()),
        None => empty_result_body(state),
    };

    container(
        column![
            container(
                row![
                    text(result_title(texts.results, result, state.language()))
                        .size(11)
                        .style(theme::secondary_text),
                    if result.is_some() {
                        chip(texts.query_success.to_uppercase(), true)
                    } else {
                        chip(texts.ready.to_uppercase(), false)
                    },
                    chip(
                        result_latency_chip(result, state.query_running(), state.language()),
                        false
                    ),
                    Space::with_width(Length::Fill),
                    result_tool_button(texts.search, Message::RequestResultSearch),
                    result_tool_button(texts.csv, Message::ExportResultCsv),
                    result_tool_button(
                        if state.result_focus() {
                            texts.exit
                        } else {
                            texts.fullscreen
                        },
                        Message::ToggleResultFocus,
                    ),
                ]
                .spacing(8)
                .align_y(Alignment::Center),
            )
            .padding([8, 14])
            .width(Length::Fill)
            .style(theme::query_header),
            result_body,
            table_result_pagination(state),
        ]
        .height(Length::Fill),
    )
    .style(theme::query_canvas)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn table_result_pagination(state: &Akt) -> Element<'static, Message> {
    let Some(page) = state.table_rows_page() else {
        return Space::with_height(0).into();
    };
    let has_previous = page.page() > 0;
    let has_next = state
        .query_workspace()
        .last_result()
        .map(|result| result.rows.len() == page.page_size())
        .unwrap_or(false);

    let mut previous = button(button_label("<", 12))
        .width(42)
        .height(26)
        .padding(0)
        .style(theme::secondary_button);
    if has_previous && !state.query_running() {
        previous = previous.on_press(Message::LoadTableRowsPage(-1));
    }

    let mut next = button(button_label(">", 12))
        .width(42)
        .height(26)
        .padding(0)
        .style(theme::secondary_button);
    if has_next && !state.query_running() {
        next = next.on_press(Message::LoadTableRowsPage(1));
    }

    container(
        row![
            text(format!(
                "{}  page {}  rows/page {}",
                page.table(),
                page.page() + 1,
                page.page_size()
            ))
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
            Space::with_width(Length::Fill),
            previous,
            next,
        ]
        .spacing(8)
        .align_y(Alignment::Center),
    )
    .height(34)
    .padding([4, 14])
    .style(theme::query_header)
    .width(Length::Fill)
    .into()
}

fn query_workspace_footer(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let latency = state
        .result_latency_ms()
        .map(|latency| format!("{latency}ms"))
        .unwrap_or_else(|| String::from("--"));
    let status = if state.query_running() {
        texts.running
    } else if state.query_workspace().last_result().is_some() {
        texts.query_success
    } else {
        texts.ready
    };

    container(
        row![
            text(state.status_message())
                .size(10)
                .style(theme::secondary_text),
            Space::with_width(Length::Fill),
            text(format!("{}: {}", texts.rows, state.result_row_count()))
                .size(10)
                .style(theme::secondary_text),
            vertical_separator(),
            text(format!("{}: {latency}", texts.latency))
                .size(10)
                .style(theme::secondary_text),
            vertical_separator(),
            text(status.to_uppercase())
                .size(10)
                .style(theme::primary_text),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .height(26)
    .padding([4, 14])
    .style(theme::query_header)
    .width(Length::Fill)
    .into()
}

fn empty_result_body(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let content = state.query_workspace().messages().iter().fold(
        column![
            text(texts.no_result).size(13).style(theme::secondary_text),
            text(texts.messages.to_uppercase())
                .size(11)
                .style(theme::primary_text),
        ]
        .spacing(8)
        .align_x(Alignment::Center),
        |column, message| column.push(text(message).size(12).style(theme::on_surface_text)),
    );

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn result_title(label: &'static str, result: Option<&QueryResult>, language: Language) -> String {
    let texts = i18n::texts(language);

    match result {
        Some(result) if result.columns.is_empty() => {
            format!(
                "{} ({} {})",
                label.to_uppercase(),
                result.row_count(),
                texts.affected.to_uppercase()
            )
        }
        Some(result) if result.truncated => {
            format!(
                "{} ({} {})",
                label.to_uppercase(),
                result.rows.len(),
                texts.rows_shown_capped.to_uppercase()
            )
        }
        Some(result) => format!(
            "{} ({} {})",
            label.to_uppercase(),
            result.rows.len(),
            texts.rows.to_uppercase()
        ),
        None => label.to_uppercase(),
    }
}

fn result_latency_chip(result: Option<&QueryResult>, running: bool, language: Language) -> String {
    if running {
        i18n::texts(language).running.to_uppercase()
    } else {
        result
            .map(|result| format!("{}ms", result.elapsed_ms))
            .unwrap_or_else(|| String::from("--"))
    }
}

pub(in crate::ui) fn chip(label: String, primary: bool) -> Element<'static, Message> {
    container(text(label).size(10).style(if primary {
        theme::on_primary_text
    } else {
        theme::on_surface_text
    }))
    .style(if primary {
        theme::active_marker
    } else {
        theme::status_chip
    })
    .padding([5, 9])
    .into()
}

fn result_tool_button(label: &'static str, message: Message) -> Element<'static, Message> {
    button(text(label.to_uppercase()).size(10))
        .padding([5, 8])
        .style(theme::secondary_button)
        .on_press(message)
        .into()
}
