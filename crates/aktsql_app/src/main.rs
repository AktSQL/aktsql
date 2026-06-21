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

use app::{Akt, Message};
use iced::{application, window, Font, Result, Size, Task};
use product::APP_NAME;

const WINDOW_SIZE: Size = Size::new(1280.0, 800.0);
const UI_FONT: Font = Font::with_name("Noto Sans CJK SC");

fn main() -> Result {
    application(APP_NAME, update, view)
        .font(include_bytes!("../assets/fonts/NotoSansCJK-Regular.ttc").as_slice())
        .default_font(UI_FONT)
        .window(fixed_window_settings())
        .subscription(|state| state.subscription())
        .theme(|state| state.theme().iced_theme())
        .run_with(|| (Akt::default(), Task::none()))
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
