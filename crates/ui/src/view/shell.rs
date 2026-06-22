use super::{
    button_label, vertical_separator, TITLE_BAR_PADDING, TOP_BAR_HEIGHT, TOP_COMMAND_HEIGHT,
    TOP_PRIMARY_WIDTH, TOP_REFRESH_WIDTH, WINDOW_CONTROL_SIZE,
};
use crate::i18n;
use crate::theme;
use crate::Akt;
use engine::{Message, Section};
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
    let nav = row![
        nav_label(
            texts.nav_editor,
            matches!(state.selected(), Section::Databases)
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
    let action_bar = if state.selected() == Section::Tables
        || (state.selected() == Section::Databases && state.database_workspace_active())
    {
        row![top_icon_button(
            if state.schema_loading() {
                "..."
            } else {
                texts.refresh
            },
            if state.schema_loading() {
                None
            } else {
                Some(Message::RefreshQuerySchema)
            },
        )]
        .spacing(8)
        .align_y(Alignment::Center)
    } else if state.selected() == Section::Databases {
        row![
            top_icon_button(texts.refresh, Some(Message::ReloadConnectionProfiles),),
            top_action_button(
                if state.connection_connecting() {
                    texts.running
                } else if state.connection_manager().is_new_profile() {
                    texts.save
                } else if state.query_running() {
                    texts.running
                } else {
                    texts.connect
                },
                if state.connection_connecting() {
                    None
                } else if state.connection_manager().is_new_profile() {
                    Some(Message::SaveConnectionProfile)
                } else {
                    Some(Message::ConnectConnectionProfile)
                },
                true,
            ),
        ]
        .spacing(8)
        .align_y(Alignment::Center)
    } else {
        row![].spacing(8).align_y(Alignment::Center)
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
