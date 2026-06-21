mod app;
mod connection;
mod connector;
mod i18n;
mod persistence;
mod product;
mod query;
mod schema;
mod system_metrics;
mod theme;
mod ui;

use app::{Akt, Message, UiFontPreference};
use iced::{application, window, Font, Result, Size, Task};
use product::APP_NAME;

const WINDOW_SIZE: Size = Size::new(1280.0, 800.0);

fn main() -> Result {
    application(APP_NAME, update, view)
        .default_font(system_ui_font())
        .window(fixed_window_settings())
        .subscription(|state| state.subscription())
        .theme(|state| state.theme().iced_theme())
        .run_with(|| (Akt::default(), Task::none()))
}

fn system_ui_font() -> Font {
    let preferences = persistence::load_preferences().unwrap_or_default();
    Font::with_name(UiFontPreference::from_config(&preferences.ui_font).ui_font_name())
}

fn fixed_window_settings() -> window::Settings {
    window::Settings {
        size: WINDOW_SIZE,
        min_size: Some(WINDOW_SIZE),
        max_size: Some(WINDOW_SIZE),
        decorations: false,
        resizable: false,
        ..window::Settings::default()
    }
}

fn update(state: &mut Akt, message: Message) -> Task<Message> {
    state.update(message)
}

fn view(state: &Akt) -> iced::Element<'_, Message> {
    ui::view(state)
}
