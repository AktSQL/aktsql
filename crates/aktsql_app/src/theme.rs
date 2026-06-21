use iced::widget::overlay::menu as menu_widget;
use iced::widget::{
    button, container, pick_list as pick_list_widget, scrollable as scrollable_widget,
    text as text_widget, text_editor as text_editor_widget, text_input as text_input_widget,
};
use iced::{Background, Border, Color, Shadow, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

impl ThemeMode {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }

    pub fn iced_theme(self) -> Theme {
        match self {
            Self::Dark => Theme::Dark,
            Self::Light => Theme::Light,
        }
    }
}

pub const PRIMARY: Color = Color {
    r: 1.0,
    g: 0.706,
    b: 0.659,
    a: 1.0,
};

pub const PRIMARY_CONTAINER: Color = Color {
    r: 0.902,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

pub const ON_PRIMARY_CONTAINER: Color = Color {
    r: 1.0,
    g: 0.969,
    b: 0.961,
    a: 1.0,
};

pub const BACKGROUND: Color = Color {
    r: 0.071,
    g: 0.078,
    b: 0.078,
    a: 1.0,
};

pub const SURFACE: Color = Color {
    r: 0.071,
    g: 0.078,
    b: 0.078,
    a: 1.0,
};

pub const SURFACE_LOW: Color = Color {
    r: 0.106,
    g: 0.11,
    b: 0.11,
    a: 1.0,
};

pub const SURFACE_CONTAINER: Color = Color {
    r: 0.122,
    g: 0.125,
    b: 0.125,
    a: 1.0,
};

pub const SURFACE_HIGH: Color = Color {
    r: 0.161,
    g: 0.165,
    b: 0.165,
    a: 1.0,
};

pub const ON_SURFACE: Color = Color {
    r: 0.89,
    g: 0.886,
    b: 0.886,
    a: 1.0,
};

pub const SECONDARY: Color = Color {
    r: 0.788,
    g: 0.776,
    b: 0.773,
    a: 1.0,
};

pub const OUTLINE_VARIANT: Color = Color {
    r: 0.31,
    g: 0.255,
    b: 0.245,
    a: 0.68,
};

pub const ERROR_CONTAINER: Color = Color {
    r: 0.576,
    g: 0.0,
    b: 0.039,
    a: 1.0,
};

#[derive(Debug, Clone, Copy)]
struct Palette {
    primary: Color,
    primary_container: Color,
    on_primary_container: Color,
    background: Color,
    surface: Color,
    surface_low: Color,
    surface_container: Color,
    surface_high: Color,
    on_surface: Color,
    secondary: Color,
    outline_variant: Color,
    error_container: Color,
}

impl Palette {
    fn for_theme(theme: &Theme) -> Self {
        match theme {
            Theme::Light => Self {
                primary: Color::from_rgb8(192, 0, 0),
                primary_container: Color::from_rgb8(230, 0, 0),
                on_primary_container: Color::WHITE,
                background: Color::from_rgb8(250, 250, 250),
                surface: Color::from_rgb8(255, 255, 255),
                surface_low: Color::from_rgb8(245, 245, 245),
                surface_container: Color::from_rgb8(238, 238, 238),
                surface_high: Color::from_rgb8(226, 226, 226),
                on_surface: Color::from_rgb8(28, 27, 27),
                secondary: Color::from_rgb8(71, 70, 70),
                outline_variant: Color::from_rgb8(184, 177, 176),
                error_container: Color::from_rgb8(147, 0, 10),
            },
            _ => Self {
                primary: PRIMARY,
                primary_container: PRIMARY_CONTAINER,
                on_primary_container: ON_PRIMARY_CONTAINER,
                background: BACKGROUND,
                surface: SURFACE,
                surface_low: SURFACE_LOW,
                surface_container: SURFACE_CONTAINER,
                surface_high: SURFACE_HIGH,
                on_surface: ON_SURFACE,
                secondary: SECONDARY,
                outline_variant: OUTLINE_VARIANT,
                error_container: ERROR_CONTAINER,
            },
        }
    }
}

pub fn primary_text(theme: &Theme) -> text_widget::Style {
    text_widget::Style {
        color: Some(Palette::for_theme(theme).primary),
    }
}

pub fn on_surface_text(theme: &Theme) -> text_widget::Style {
    text_widget::Style {
        color: Some(Palette::for_theme(theme).on_surface),
    }
}

pub fn secondary_text(theme: &Theme) -> text_widget::Style {
    text_widget::Style {
        color: Some(Palette::for_theme(theme).secondary),
    }
}

pub fn on_primary_text(theme: &Theme) -> text_widget::Style {
    text_widget::Style {
        color: Some(Palette::for_theme(theme).on_primary_container),
    }
}

pub fn app_background(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);
    let border = match theme {
        Theme::Light => with_alpha(palette.primary_container, 0.34),
        _ => with_alpha(palette.primary_container, 0.46),
    };

    container::Style::default()
        .background(palette.background)
        .color(palette.on_surface)
        .border(Border::default().width(1).color(border))
}

pub fn top_bar(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);
    panel_with_border(
        palette.surface_low,
        palette.outline_variant,
        palette.on_surface,
    )
}

pub fn sidebar(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);
    panel_with_border(palette.surface, palette.outline_variant, palette.on_surface)
}

pub fn workspace(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.background)
        .color(palette.on_surface)
}

pub fn panel(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);
    panel_with_border(
        palette.surface_container,
        palette.outline_variant,
        palette.on_surface,
    )
}

pub fn panel_low(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);
    panel_with_border(
        palette.surface_low,
        palette.outline_variant,
        palette.on_surface,
    )
}

pub fn panel_flat(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.surface_container)
        .color(palette.on_surface)
}

pub fn query_workbench(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.surface)
        .color(palette.on_surface)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.74)),
        )
}

pub fn query_header(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.surface_low)
        .color(palette.on_surface)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.22)),
        )
}

pub fn query_canvas(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.surface)
        .color(palette.on_surface)
}

pub fn query_gutter(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.surface)
        .color(palette.secondary)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.42)),
        )
}

pub fn result_header_cell(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.surface_high)
        .color(palette.primary)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.44)),
        )
}

pub fn result_header_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let palette = Palette::for_theme(theme);
        let background = if active {
            palette.surface_container
        } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            palette.surface_high
        } else {
            palette.surface_high
        };
        let border_color = if active {
            with_alpha(palette.primary, 0.72)
        } else {
            with_alpha(palette.outline_variant, 0.44)
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: palette.primary,
            border: Border::default().width(1).color(border_color),
            shadow: Shadow::default(),
        }
    }
}

pub fn result_data_cell(row_index: usize) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        let palette = Palette::for_theme(theme);
        let background = if row_index % 2 == 0 {
            palette.surface
        } else {
            with_alpha(palette.surface_low, 0.58)
        };

        container::Style::default()
            .background(background)
            .color(palette.on_surface)
            .border(
                Border::default()
                    .width(1)
                    .color(with_alpha(palette.outline_variant, 0.20)),
            )
    }
}

pub fn schema_grid_row(row_index: usize, selected: bool) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        let palette = Palette::for_theme(theme);
        let background = if selected {
            with_alpha(palette.primary_container, 0.18)
        } else if row_index % 2 == 0 {
            palette.surface
        } else {
            with_alpha(palette.surface_low, 0.58)
        };
        let border_color = if selected {
            with_alpha(palette.primary, 0.72)
        } else {
            with_alpha(palette.outline_variant, 0.20)
        };

        container::Style::default()
            .background(background)
            .color(palette.on_surface)
            .border(Border::default().width(1).color(border_color))
    }
}

pub fn status_chip(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(with_alpha(palette.outline_variant, 0.20))
        .color(palette.primary)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.48)),
        )
}

pub fn form_section(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(with_alpha(palette.surface_low, 0.38))
        .color(palette.on_surface)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.16)),
        )
}

pub fn form_action_bar(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(with_alpha(palette.surface, 0.0))
        .color(palette.on_surface)
        .border(
            Border::default()
                .width(1)
                .color(with_alpha(palette.outline_variant, 0.24)),
        )
}

pub fn form_header_icon(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(with_alpha(palette.primary_container, 0.84))
        .color(palette.on_primary_container)
}

pub fn divider(theme: &Theme) -> container::Style {
    container::Style::default()
        .background(with_alpha(Palette::for_theme(theme).outline_variant, 0.42))
}

pub fn input_underline(theme: &Theme) -> container::Style {
    container::Style::default()
        .background(with_alpha(Palette::for_theme(theme).outline_variant, 0.58))
}

pub fn validation_error(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);
    panel_with_border(
        palette.error_container,
        palette.primary,
        palette.on_primary_container,
    )
}

pub fn active_marker(theme: &Theme) -> container::Style {
    container::Style::default().background(Palette::for_theme(theme).primary)
}

pub fn danger_marker(theme: &Theme) -> container::Style {
    container::Style::default().background(Palette::for_theme(theme).primary_container)
}

pub fn detail_category_marker(index: usize) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        let palette = Palette::for_theme(theme);
        let color = match index % 5 {
            0 => palette.primary_container,
            1 => Color::from_rgb8(211, 92, 0),
            2 => Color::from_rgb8(144, 88, 214),
            3 => Color::from_rgb8(0, 132, 120),
            _ => palette.outline_variant,
        };

        container::Style::default().background(color)
    }
}

pub fn success_dot(_theme: &Theme) -> container::Style {
    container::Style::default()
        .background(Color::from_rgb8(34, 197, 94))
        .border(Border::default().rounded(8))
}

pub fn radio_outer(active: bool) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        let palette = Palette::for_theme(theme);
        let border_color = if active {
            palette.primary
        } else {
            with_alpha(palette.secondary, 0.66)
        };

        container::Style::default().border(
            Border::default()
                .width(if active { 1.6 } else { 1.2 })
                .rounded(9)
                .color(border_color),
        )
    }
}

pub fn radio_dot(theme: &Theme) -> container::Style {
    container::Style::default()
        .background(Palette::for_theme(theme).primary)
        .border(Border::default().rounded(4))
}

pub fn transparent(_theme: &Theme) -> container::Style {
    container::Style::default()
}

pub fn modal_scrim(_theme: &Theme) -> container::Style {
    container::Style::default().background(Color {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.58,
    })
}

pub fn scrollable(theme: &Theme, status: scrollable_widget::Status) -> scrollable_widget::Style {
    let palette = Palette::for_theme(theme);
    let scroller_color = match status {
        scrollable_widget::Status::Dragged { .. } => with_alpha(palette.primary, 0.78),
        scrollable_widget::Status::Hovered { .. } => with_alpha(palette.primary, 0.58),
        scrollable_widget::Status::Active => with_alpha(palette.secondary, 0.42),
    };

    let rail = scrollable_widget::Rail {
        background: Some(Background::Color(with_alpha(palette.surface_high, 0.18))),
        border: Border::default()
            .width(0)
            .rounded(4)
            .color(with_alpha(palette.outline_variant, 0.0)),
        scroller: scrollable_widget::Scroller {
            color: scroller_color,
            border: Border::default()
                .width(0)
                .rounded(4)
                .color(with_alpha(palette.outline_variant, 0.0)),
        },
    };

    scrollable_widget::Style {
        container: container::Style::default(),
        vertical_rail: rail,
        horizontal_rail: rail,
        gap: Some(Background::Color(palette.background)),
    }
}

pub fn primary_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = Palette::for_theme(theme);
    let background = match status {
        button::Status::Hovered | button::Status::Pressed => palette.primary,
        _ => palette.primary_container,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: palette.on_primary_container,
        border: Border::default()
            .width(1)
            .color(with_alpha(palette.primary_container, 0.62)),
        shadow: Shadow::default(),
    }
}

pub fn secondary_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = Palette::for_theme(theme);
    let border_color = match status {
        button::Status::Hovered | button::Status::Pressed => with_alpha(palette.primary, 0.85),
        _ => with_alpha(palette.outline_variant, 0.36),
    };

    button::Style {
        background: Some(Background::Color(palette.surface_container)),
        text_color: palette.on_surface,
        border: Border::default().width(1).color(border_color),
        shadow: Shadow::default(),
    }
}

pub fn danger_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = Palette::for_theme(theme);
    let background = match status {
        button::Status::Hovered | button::Status::Pressed => palette.error_container,
        _ => palette.surface_high,
    };

    button::Style {
        background: Some(Background::Color(background)),
        text_color: palette.on_surface,
        border: Border::default()
            .width(1)
            .color(with_alpha(palette.outline_variant, 0.34)),
        shadow: Shadow::default(),
    }
}

pub fn title_control_button(close: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let palette = Palette::for_theme(theme);
        let is_active = matches!(status, button::Status::Hovered | button::Status::Pressed);
        let background = if close && is_active {
            palette.primary_container
        } else if is_active {
            palette.surface_high
        } else {
            palette.surface_container
        };
        let text_color = if close && is_active {
            palette.on_primary_container
        } else {
            palette.secondary
        };
        let border_color = if is_active {
            with_alpha(palette.primary, 0.74)
        } else {
            with_alpha(palette.outline_variant, 0.30)
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color,
            border: Border::default().width(1).rounded(2).color(border_color),
            shadow: Shadow::default(),
        }
    }
}

pub fn sidebar_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let palette = Palette::for_theme(theme);
        let background = if active {
            palette.surface_high
        } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            palette.surface_container
        } else {
            palette.surface
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: if active {
                palette.primary
            } else {
                palette.secondary
            },
            border: Border::default()
                .width(0)
                .rounded(0)
                .color(with_alpha(palette.surface, 0.0)),
            shadow: Shadow::default(),
        }
    }
}

pub fn sidebar_item(active: bool) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        let palette = Palette::for_theme(theme);
        let background = if active {
            palette.surface_high
        } else {
            palette.surface
        };

        container::Style::default()
            .background(background)
            .color(if active {
                palette.primary
            } else {
                palette.secondary
            })
    }
}

pub fn sidebar_primary_action(theme: &Theme) -> container::Style {
    let palette = Palette::for_theme(theme);

    container::Style::default()
        .background(palette.primary_container)
        .color(palette.on_primary_container)
}

pub fn connection_card_button(active: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let palette = Palette::for_theme(theme);
        let background = if active {
            palette.surface_high
        } else if matches!(status, button::Status::Hovered | button::Status::Pressed) {
            palette.surface_container
        } else {
            palette.surface_low
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: palette.on_surface,
            border: Border::default().width(1).color(if active {
                with_alpha(palette.primary, 0.38)
            } else {
                with_alpha(palette.outline_variant, 0.10)
            }),
            shadow: Shadow::default(),
        }
    }
}

pub fn form_text_input(
    theme: &Theme,
    status: text_input_widget::Status,
) -> text_input_widget::Style {
    let palette = Palette::for_theme(theme);
    let border_color = match status {
        text_input_widget::Status::Focused => palette.primary,
        text_input_widget::Status::Hovered => with_alpha(palette.secondary, 0.82),
        text_input_widget::Status::Disabled | text_input_widget::Status::Active => {
            with_alpha(palette.outline_variant, 0.62)
        }
    };

    text_input_widget::Style {
        background: Background::Color(palette.surface),
        border: Border::default().width(0).color(border_color),
        icon: palette.secondary,
        placeholder: palette.outline_variant,
        value: palette.on_surface,
        selection: palette.primary,
    }
}

pub fn text_editor(theme: &Theme, status: text_editor_widget::Status) -> text_editor_widget::Style {
    let palette = Palette::for_theme(theme);
    let border_color = match status {
        text_editor_widget::Status::Focused => palette.primary,
        text_editor_widget::Status::Hovered => with_alpha(palette.secondary, 0.82),
        text_editor_widget::Status::Disabled | text_editor_widget::Status::Active => {
            with_alpha(palette.outline_variant, 0.62)
        }
    };

    text_editor_widget::Style {
        background: Background::Color(palette.surface),
        border: Border::default().width(1).color(border_color),
        icon: palette.secondary,
        placeholder: palette.outline_variant,
        value: palette.on_surface,
        selection: palette.primary,
    }
}

pub fn toggle_button(enabled: bool) -> impl Fn(&Theme, button::Status) -> button::Style {
    move |theme, status| {
        let palette = Palette::for_theme(theme);
        let background = if enabled {
            palette.primary_container
        } else {
            palette.surface_container
        };
        let border_color = match status {
            button::Status::Hovered | button::Status::Pressed => palette.primary,
            _ => with_alpha(palette.outline_variant, 0.62),
        };

        button::Style {
            background: Some(Background::Color(background)),
            text_color: palette.on_surface,
            border: Border::default()
                .width(if enabled { 0.0 } else { 1.0 })
                .color(border_color)
                .rounded(10),
            shadow: Shadow::default(),
        }
    }
}

pub fn toggle_knob(enabled: bool) -> impl Fn(&Theme) -> container::Style {
    move |theme| {
        let palette = Palette::for_theme(theme);
        let color = if enabled {
            Color::WHITE
        } else {
            palette.secondary
        };

        container::Style::default()
            .background(color)
            .border(Border::default().rounded(8))
    }
}

pub fn pick_list(theme: &Theme, status: pick_list_widget::Status) -> pick_list_widget::Style {
    let palette = Palette::for_theme(theme);
    let border_color = match status {
        pick_list_widget::Status::Hovered | pick_list_widget::Status::Opened => {
            with_alpha(palette.primary, 0.88)
        }
        pick_list_widget::Status::Active => with_alpha(palette.outline_variant, 0.62),
    };

    pick_list_widget::Style {
        text_color: palette.on_surface,
        placeholder_color: palette.secondary,
        handle_color: palette.primary,
        background: Background::Color(palette.surface),
        border: Border::default().width(0).color(border_color),
    }
}

pub fn pick_list_menu(theme: &Theme) -> menu_widget::Style {
    let palette = Palette::for_theme(theme);

    menu_widget::Style {
        background: Background::Color(palette.surface_high),
        border: Border::default()
            .width(1)
            .color(with_alpha(palette.outline_variant, 0.28)),
        text_color: palette.on_surface,
        selected_text_color: palette.on_primary_container,
        selected_background: Background::Color(palette.primary_container),
    }
}

fn panel_with_border(background: Color, border: Color, text: Color) -> container::Style {
    container::Style::default()
        .background(background)
        .color(text)
        .border(Border::default().width(1).color(with_alpha(border, 0.54)))
}

fn with_alpha(color: Color, alpha: f32) -> Color {
    Color { a: alpha, ..color }
}
