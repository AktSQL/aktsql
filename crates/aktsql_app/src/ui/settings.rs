use crate::app::{Akt, Message};
use crate::i18n::{self, Language};
use crate::theme;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{button, column, container, horizontal_rule, row, scrollable, text, Space};
use iced::{Alignment, Element, Length};

pub(super) fn appearance_settings_view(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let content = column![
        column![
            text(texts.appearance_settings)
                .size(32)
                .style(theme::primary_text),
            text(texts.appearance_settings_body)
                .size(14)
                .style(theme::secondary_text),
        ]
        .spacing(8),
        row![
            theme_settings_panel(state),
            density_settings_panel(state.language()),
        ]
        .spacing(14)
        .height(260),
        row![
            typography_settings_panel(state.language()),
            keybinding_settings_panel(state.language()),
        ]
        .spacing(14)
        .height(300),
    ]
    .spacing(14)
    .padding([28, 32]);

    container(
        column![
            scrollable(content)
                .height(Length::Fill)
                .style(theme::scrollable),
            container(
                row![
                    Space::with_width(Length::Fill),
                    button(text(texts.discard_changes.to_uppercase()).size(12))
                        .padding([10, 16])
                        .style(theme::secondary_button),
                    button(text(texts.apply_configuration.to_uppercase()).size(12))
                        .padding([10, 20])
                        .style(theme::primary_button)
                        .on_press(Message::ToggleTheme),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .style(theme::panel_low)
            .width(Length::Fill)
            .padding([14, 32]),
        ]
        .height(Length::Fill),
    )
    .style(theme::workspace)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn theme_settings_panel(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());

    container(
        column![
            settings_section_title(texts.palette, texts.interface_theme),
            row![
                theme_option("Obsidian", "Dark", state.theme().label() == "Dark", true),
                theme_option("Dawn", "Light", state.theme().label() == "Light", false),
                theme_option("Amaterasu", "High Contrast", false, true),
            ]
            .spacing(14)
            .height(Length::Fill),
        ]
        .spacing(18),
    )
    .style(theme::panel)
    .width(Length::FillPortion(7))
    .height(Length::Fill)
    .padding(22)
    .into()
}

fn theme_option(
    name: &'static str,
    detail: &'static str,
    active: bool,
    dark_preview: bool,
) -> Element<'static, Message> {
    let preview_style = if dark_preview {
        theme::workspace
    } else {
        theme::panel_low
    };

    let mut action = button(
        column![
            container(
                column![
                    container(Space::new(96, 8))
                        .style(theme::active_marker)
                        .width(96)
                        .height(8),
                    container(Space::new(130, 5))
                        .style(theme::panel)
                        .width(130)
                        .height(5),
                    container(Space::new(82, 5))
                        .style(theme::panel)
                        .width(82)
                        .height(5),
                ]
                .spacing(7),
            )
            .style(preview_style)
            .width(Length::Fill)
            .height(78)
            .padding(12),
            row![
                column![
                    text(name.to_uppercase())
                        .size(11)
                        .style(theme::on_surface_text),
                    text(detail.to_uppercase())
                        .size(10)
                        .style(theme::secondary_text),
                ]
                .spacing(3),
                Space::with_width(Length::Fill),
                if active {
                    text("OK").size(11).style(theme::primary_text)
                } else {
                    text("").size(11).style(theme::secondary_text)
                },
            ]
            .align_y(Alignment::Center),
        ]
        .spacing(10),
    )
    .width(Length::FillPortion(1))
    .padding(12)
    .style(if active {
        theme::primary_button
    } else {
        theme::secondary_button
    });

    if !active {
        action = action.on_press(Message::ToggleTheme);
    }

    action.into()
}

fn density_settings_panel(language: Language) -> Element<'static, Message> {
    let texts = i18n::texts(language);

    container(
        column![
            settings_section_title(texts.layout, texts.layout_density),
            density_row(
                texts.compact_default,
                texts.compact_detail,
                "4px Grid",
                true
            ),
            density_row(texts.normal, texts.normal_detail, "8px Grid", false),
            horizontal_rule(1),
            row![
                text(texts.sidebar_width.to_uppercase())
                    .size(11)
                    .style(theme::secondary_text),
                Space::with_width(Length::Fill),
                text("240PX").size(15).style(theme::primary_text),
            ],
            container(Space::new(Length::Fill, 4))
                .style(theme::active_marker)
                .height(4)
                .width(Length::Fill),
        ]
        .spacing(16),
    )
    .style(theme::panel)
    .width(Length::FillPortion(5))
    .height(Length::Fill)
    .padding(22)
    .into()
}

fn density_row(
    title: &'static str,
    detail: &'static str,
    value: &'static str,
    active: bool,
) -> Element<'static, Message> {
    row![
        radio_indicator(active),
        column![
            text(title).size(14).style(theme::on_surface_text),
            text(detail).size(11).style(theme::secondary_text),
        ]
        .spacing(3)
        .width(Length::Fill),
        text(value).size(16).style(if active {
            theme::primary_text
        } else {
            theme::secondary_text
        }),
    ]
    .spacing(12)
    .align_y(Alignment::Center)
    .into()
}

fn radio_indicator(active: bool) -> Element<'static, Message> {
    let dot: Element<'static, Message> = if active {
        container(Space::new(8, 8))
            .width(8)
            .height(8)
            .style(theme::radio_dot)
            .into()
    } else {
        Space::new(8, 8).into()
    };

    container(dot)
        .width(18)
        .height(18)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .style(theme::radio_outer(active))
        .into()
}

fn typography_settings_panel(language: Language) -> Element<'static, Message> {
    let texts = i18n::texts(language);

    container(
        column![
            settings_section_title(texts.editor, texts.typography),
            field_preview(texts.editor_font_family, "Noto Sans CJK SC"),
            row![
                field_preview(texts.font_size, "13 PX"),
                field_preview(texts.line_height, "1.5 EM"),
            ]
            .spacing(18),
            container(
                column![
                    text("1   SELECT * FROM users")
                        .size(13)
                        .style(theme::on_surface_text),
                    text("2   WHERE status = 'ACTIVE';")
                        .size(13)
                        .style(theme::primary_text),
                ]
                .spacing(6),
            )
            .style(theme::panel_low)
            .width(Length::Fill)
            .padding(14),
        ]
        .spacing(16),
    )
    .style(theme::panel)
    .width(Length::FillPortion(6))
    .height(Length::Fill)
    .padding(22)
    .into()
}

fn keybinding_settings_panel(language: Language) -> Element<'static, Message> {
    let texts = i18n::texts(language);

    container(
        column![
            settings_section_title(texts.control, texts.keybindings),
            keybinding_row(texts.execute_current_query, "CTRL", "ENTER"),
            keybinding_row(texts.toggle_theme, "F6", ""),
            keybinding_row(texts.refresh_schema, "F5", ""),
            keybinding_row(texts.commit_transactions, "F9", ""),
            horizontal_rule(1),
            row![
                button(text(texts.standard.to_uppercase()).size(12))
                    .padding([9, 24])
                    .style(theme::primary_button),
                button(text("VIM").size(12))
                    .padding([9, 24])
                    .style(theme::secondary_button),
                button(text("EMACS").size(12))
                    .padding([9, 24])
                    .style(theme::secondary_button),
            ]
            .spacing(10),
        ]
        .spacing(16),
    )
    .style(theme::panel)
    .width(Length::FillPortion(6))
    .height(Length::Fill)
    .padding(22)
    .into()
}

fn settings_section_title(kind: &'static str, title: &'static str) -> Element<'static, Message> {
    column![
        text(kind).size(10).style(theme::secondary_text),
        text(title).size(24).style(theme::on_surface_text),
    ]
    .spacing(3)
    .into()
}

fn field_preview(label: &'static str, value: &'static str) -> Element<'static, Message> {
    column![
        text(label).size(11).style(theme::secondary_text),
        container(text(value).size(14).style(theme::on_surface_text))
            .style(theme::panel_low)
            .width(Length::Fill)
            .padding([10, 12]),
    ]
    .spacing(7)
    .width(Length::Fill)
    .into()
}

fn keybinding_row(
    label: &'static str,
    key_a: &'static str,
    key_b: &'static str,
) -> Element<'static, Message> {
    row![
        text(label).size(14).style(theme::on_surface_text),
        Space::with_width(Length::Fill),
        keycap(key_a),
        if key_b.is_empty() {
            Space::with_width(0).into()
        } else {
            keycap(key_b)
        },
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn keycap(label: &'static str) -> Element<'static, Message> {
    container(text(label).size(10).style(theme::on_surface_text))
        .style(theme::panel_low)
        .padding([5, 8])
        .into()
}
