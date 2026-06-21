use super::{chip, section_label};
use crate::app::{Akt, Message, QuickAction};
use crate::i18n;
use crate::theme;
use iced::widget::{button, column, container, horizontal_rule, row, text, Space};
use iced::{Alignment, Element, Length};

pub(super) fn generic_workspace(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let title = section_label(state.selected(), state.language());

    let cards = row![
        capability_panel(
            texts.connection_manager_placeholder,
            "Profiles, drivers, credentials, validation"
        ),
        capability_panel(
            texts.query_console_placeholder,
            "SQL editor, execution results, schema tree"
        ),
        capability_panel(
            texts.metadata_diagrams_placeholder,
            "Objects, relations, migrations, diagrams"
        ),
    ]
    .spacing(14)
    .height(148);

    container(
        column![
            row![
                column![
                    text(title).size(30).style(theme::primary_text),
                    text(texts.shell_ready)
                        .size(13)
                        .style(theme::secondary_text),
                ]
                .spacing(6)
                .width(Length::Fill),
                chip(texts.ready.to_uppercase(), true),
            ]
            .align_y(Alignment::Center),
            quick_actions(),
            cards,
            container(
                column![
                    text(texts.next_capability_placeholders.to_uppercase())
                        .size(11)
                        .style(theme::secondary_text),
                    horizontal_rule(1),
                    placeholder_row("01", texts.table_designer_placeholder, "DDL draft surface"),
                    placeholder_row(
                        "02",
                        texts.system_logs,
                        "Connection events and query history"
                    ),
                    placeholder_row("03", texts.messages, "Driver diagnostics and notices"),
                ]
                .spacing(12),
            )
            .style(theme::panel)
            .width(Length::Fill)
            .padding(18),
        ]
        .spacing(16)
        .padding([28, 32]),
    )
    .style(theme::workspace)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn capability_panel(title: &'static str, detail: &'static str) -> Element<'static, Message> {
    container(
        column![
            text(title.trim_start_matches("- "))
                .size(17)
                .style(theme::primary_text),
            text(detail).size(12).style(theme::secondary_text),
            Space::with_height(Length::Fill),
            container(Space::with_height(3))
                .style(theme::active_marker)
                .height(3)
                .width(Length::Fill),
        ]
        .spacing(9),
    )
    .style(theme::panel)
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .padding(16)
    .into()
}

fn placeholder_row(
    index: &'static str,
    title: &'static str,
    detail: &'static str,
) -> Element<'static, Message> {
    row![
        text(index).size(12).style(theme::primary_text),
        column![
            text(title.trim_start_matches("- "))
                .size(14)
                .style(theme::on_surface_text),
            text(detail).size(11).style(theme::secondary_text),
        ]
        .spacing(3)
        .width(Length::Fill),
        text("READY").size(10).style(theme::secondary_text),
    ]
    .spacing(14)
    .align_y(Alignment::Center)
    .into()
}

fn quick_actions() -> Element<'static, Message> {
    let actions = QuickAction::PRIMARY.into_iter().fold(
        row![].spacing(8).align_y(Alignment::Center),
        |row, action| row.push(quick_action_button(action)),
    );

    container(actions).width(Length::Fill).into()
}

fn quick_action_button(action: QuickAction) -> Element<'static, Message> {
    button(text(action.label()).size(14))
        .padding([7, 10])
        .style(if action == QuickAction::NewConnection {
            theme::primary_button
        } else {
            theme::secondary_button
        })
        .on_press(Message::RunQuickAction(action))
        .into()
}
