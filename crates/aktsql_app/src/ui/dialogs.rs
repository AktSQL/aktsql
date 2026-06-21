use super::*;
use iced::widget::{column, row};

pub(super) fn test_result_dialog(state: &Akt) -> Element<'static, Message> {
    let texts = i18n::texts(state.language());

    container(
        container(
            column![
                text(texts.confirm_test_connection)
                    .size(20)
                    .style(theme::primary_text),
                container(
                    text(state.status_message().to_owned())
                        .size(16)
                        .style(theme::on_surface_text),
                )
                .style(theme::panel_low)
                .width(Length::Fill)
                .padding([10, 12]),
                row![
                    Space::with_width(Length::Fill),
                    test_result_close_button(texts.ok, state.connection_testing()),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(16),
        )
        .style(theme::panel)
        .width(430)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn delete_confirmation_dialog(
    state: &Akt,
    profile_id: usize,
) -> Element<'static, Message> {
    let texts = i18n::texts(state.language());
    let label = state
        .connection_manager()
        .profile_label(profile_id)
        .unwrap_or_else(|| String::from("connection"));

    container(
        container(
            column![
                text(texts.delete_connection)
                    .size(20)
                    .style(theme::primary_text),
                text(format!("{} {}.", texts.delete_connection_body, label))
                    .size(13)
                    .style(theme::on_surface_text),
                row![
                    Space::with_width(Length::Fill),
                    button(button_label(texts.cancel, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::secondary_button)
                        .on_press(Message::CancelDeleteConnection),
                    button(button_label(texts.delete, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::danger_button)
                        .on_press(Message::ConfirmDeleteConnection),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(16),
        )
        .style(theme::panel)
        .width(430)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn schema_delete_confirmation_dialog(
    state: &Akt,
    kind: SchemaDeletionKind,
    name: &str,
) -> Element<'static, Message> {
    let texts = i18n::texts(state.language());
    let target = name.to_owned();
    let title = match kind {
        SchemaDeletionKind::Database => texts.drop_database,
        SchemaDeletionKind::Table => texts.delete_table,
    };
    let body = match kind {
        SchemaDeletionKind::Database => format!("DROP DATABASE {target}?"),
        SchemaDeletionKind::Table => format!("DROP TABLE {target}?"),
    };

    container(
        container(
            column![
                text(title).size(20).style(theme::primary_text),
                text(body).size(13).style(theme::on_surface_text),
                row![
                    Space::with_width(Length::Fill),
                    button(button_label(texts.back, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::secondary_button)
                        .on_press(Message::CancelSchemaDelete),
                    button(button_label(texts.ok, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::danger_button)
                        .on_press(Message::ConfirmSchemaDelete),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(16),
        )
        .style(theme::panel)
        .width(430)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

fn test_result_close_button(label: &'static str, testing: bool) -> Element<'static, Message> {
    button(button_label(label, 12))
        .width(112)
        .height(FORM_ACTION_HEIGHT)
        .padding([0, 12])
        .style(if testing {
            theme::secondary_button
        } else {
            theme::primary_button
        })
        .on_press(Message::CloseTestResult)
        .into()
}
