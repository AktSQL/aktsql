use super::{
    button_label, vertical_separator, TITLE_BAR_PADDING, TITLE_LANGUAGE_DROPDOWN_RIGHT_OFFSET,
    TITLE_LANGUAGE_WIDTH, TOP_BAR_HEIGHT, TOP_COMMAND_HEIGHT, TOP_PRIMARY_WIDTH, TOP_REFRESH_WIDTH,
    WINDOW_CONTROL_SIZE,
};
use crate::app::{Akt, Message, QuickAction, Section};
use crate::i18n::{self, Language};
use crate::theme;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::text::Wrapping;
use iced::widget::{button, column, container, mouse_area, row, svg, text, Space};
use iced::{Alignment, Element, Length};
use std::sync::LazyLock;

static LOGO_HANDLE: LazyLock<iced::widget::svg::Handle> = LazyLock::new(|| {
    iced::widget::svg::Handle::from_memory(
        include_bytes!("../../assets/aktsql_logo.svg").as_slice(),
    )
});

pub(super) fn top_bar(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let database_workspace =
        state.selected() == Section::Databases && state.database_workspace_active();

    let nav = row![
        nav_label(
            texts.nav_editor,
            matches!(
                state.selected(),
                Section::Databases | Section::QueryExplorer
            )
        ),
        nav_label(texts.nav_data, false),
        nav_label(
            texts.nav_schema,
            matches!(state.selected(), Section::Tables)
        ),
        nav_label(
            texts.nav_console,
            matches!(state.selected(), Section::Settings | Section::History)
        ),
    ]
    .spacing(18)
    .align_y(Alignment::Center);
    let action_bar = if database_workspace {
        row![
            top_icon_button(
                if state.schema_loading() { "..." } else { "↻" },
                if state.schema_loading() {
                    None
                } else {
                    Some(Message::RefreshQuerySchema)
                },
            ),
            top_icon_button("↓", Some(Message::RunQuickAction(QuickAction::RefreshData))),
            top_icon_button("▽", Some(Message::RunQuickAction(QuickAction::RefreshData))),
            top_action_button(texts.commit, Some(Message::CommitTransaction), false),
            top_action_button(
                if state.query_running() {
                    texts.running
                } else {
                    texts.execute
                },
                if state.query_running() {
                    None
                } else {
                    Some(Message::ExecuteQuery)
                },
                true,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    } else {
        row![
            top_icon_button(
                if state.schema_loading()
                    && matches!(
                        state.selected(),
                        Section::Databases | Section::QueryExplorer
                    )
                {
                    "..."
                } else {
                    texts.refresh
                },
                if state.selected() == Section::Databases {
                    Some(Message::ReloadConnectionProfiles)
                } else if state.schema_loading() && state.selected() == Section::QueryExplorer {
                    None
                } else {
                    Some(Message::RunQuickAction(QuickAction::RefreshData))
                },
            ),
            top_action_button(
                if state.selected() == Section::Databases {
                    if state.connection_connecting() {
                        texts.running
                    } else if state.connection_manager().is_new_profile() {
                        texts.save
                    } else {
                        texts.connect
                    }
                } else if state.query_running() {
                    texts.running
                } else {
                    texts.execute
                },
                if state.selected() == Section::Databases && state.connection_connecting() {
                    None
                } else if state.selected() == Section::Databases
                    && state.connection_manager().is_new_profile()
                {
                    Some(Message::SaveConnectionProfile)
                } else if state.selected() == Section::Databases {
                    Some(Message::ConnectConnectionProfile)
                } else if state.query_running() {
                    None
                } else {
                    Some(Message::ExecuteQuery)
                },
                true,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    };

    container(
        row![
            mouse_area(
                row![brand_group(texts.app_title), vertical_separator(), nav,]
                    .spacing(18)
                    .align_y(Alignment::Center)
                    .height(Length::Fill),
            )
            .on_press(Message::DragWindow),
            Space::with_width(Length::Fill),
            action_bar,
            language_select(state.language()),
            window_control_button("-", Message::MinimizeWindow, false),
            window_control_button("x", Message::CloseWindow, true),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
        .padding(TITLE_BAR_PADDING),
    )
    .style(theme::top_bar)
    .height(TOP_BAR_HEIGHT)
    .width(Length::Fill)
    .into()
}

pub(super) fn language_dropdown(active_language: Language) -> Element<'static, Message> {
    container(column![
        Space::with_height(TOP_BAR_HEIGHT),
        row![
            Space::with_width(Length::Fill),
            container(
                column(Language::ALL.into_iter().map(|language| {
                    language_option_button(language, language == active_language)
                }))
                .spacing(4)
                .padding([8, 8]),
            )
            .width(TITLE_LANGUAGE_WIDTH)
            .style(theme::panel),
            Space::with_width(TITLE_LANGUAGE_DROPDOWN_RIGHT_OFFSET),
        ]
        .align_y(Alignment::Start),
        Space::with_height(Length::Fill),
    ])
    .width(Length::Fill)
    .height(Length::Fill)
    .style(theme::transparent)
    .into()
}

fn brand_group(title: &'static str) -> Element<'static, Message> {
    container(
        row![
            logo_mark(),
            Space::with_width(12),
            text(title)
                .size(24)
                .wrapping(Wrapping::None)
                .style(theme::primary_text),
        ]
        .spacing(0)
        .align_y(Alignment::Center),
    )
    .height(Length::Fill)
    .align_y(Vertical::Center)
    .width(214)
    .into()
}

fn language_select(language: Language) -> Element<'static, Message> {
    button(button_label(language_button_label(language), 11))
        .width(TITLE_LANGUAGE_WIDTH)
        .height(TOP_COMMAND_HEIGHT)
        .padding([0, 10])
        .style(theme::secondary_button)
        .on_press(Message::ToggleLanguageMenu)
        .into()
}

fn language_button_label(language: Language) -> String {
    let texts = i18n::texts(language);
    format!("{}：{}", texts.language_prefix, language.local_label())
}

fn language_option_button(language: Language, active: bool) -> Element<'static, Message> {
    button(button_label(language.label().to_uppercase(), 11))
        .width(Length::Fill)
        .height(26)
        .padding([0, 10])
        .style(if active {
            theme::primary_button
        } else {
            theme::secondary_button
        })
        .on_press(Message::LanguageSelected(language))
        .into()
}

fn top_action_button(
    label: &'static str,
    message: Option<Message>,
    primary: bool,
) -> Element<'static, Message> {
    let mut action = button(button_label(label.to_uppercase(), 12))
        .width(if primary {
            TOP_PRIMARY_WIDTH
        } else {
            TOP_REFRESH_WIDTH
        })
        .height(TOP_COMMAND_HEIGHT)
        .padding([0, 10])
        .style(if primary {
            theme::primary_button
        } else {
            theme::secondary_button
        });

    if let Some(message) = message {
        action = action.on_press(message);
    }

    action.into()
}

fn top_icon_button(label: &'static str, message: Option<Message>) -> Element<'static, Message> {
    let compact = label.chars().count() <= 2;
    let mut action = button(button_label(label, 12))
        .width(if compact { 34.0 } else { TOP_REFRESH_WIDTH })
        .height(TOP_COMMAND_HEIGHT)
        .padding([0, 10])
        .style(theme::secondary_button);

    if let Some(message) = message {
        action = action.on_press(message);
    }

    action.into()
}

fn window_control_button(
    label: &'static str,
    message: Message,
    close: bool,
) -> Element<'static, Message> {
    button(button_label(label, if close { 18 } else { 13 }))
        .width(WINDOW_CONTROL_SIZE)
        .height(WINDOW_CONTROL_SIZE)
        .padding(0)
        .style(theme::title_control_button(close))
        .on_press(message)
        .into()
}

fn logo_mark() -> Element<'static, Message> {
    container(svg((*LOGO_HANDLE).clone()).width(26).height(26))
        .width(26)
        .height(26)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .clip(true)
        .into()
}

fn nav_label(label: &'static str, active: bool) -> Element<'static, Message> {
    let style = if active {
        theme::primary_text
    } else {
        theme::secondary_text
    };

    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(style),
        container(Space::new(Length::Fill, 2))
            .height(2)
            .width(Length::Fill)
            .style(if active {
                theme::active_marker
            } else {
                theme::transparent
            }),
    ]
    .spacing(4)
    .align_x(Alignment::Center)
    .width(Length::Shrink)
    .into()
}
