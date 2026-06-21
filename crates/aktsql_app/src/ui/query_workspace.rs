use super::*;

pub(in crate::ui) fn schema_refresh_button(
    state: &Akt,
    label: &'static str,
) -> Element<'static, Message> {
    let mut action = button(button_label(
        if state.schema_loading() {
            i18n::texts(state.language()).running
        } else {
            label
        },
        12,
    ))
    .height(28)
    .padding([0, 10])
    .style(theme::secondary_button);

    if !state.schema_loading() {
        action = action.on_press(Message::RefreshQuerySchema);
    }

    action.into()
}

pub(in crate::ui) fn chip(label: String, primary: bool) -> Element<'static, Message> {
    let text_style = if primary {
        theme::on_primary_text
    } else {
        theme::on_surface_text
    };
    container(
        text(label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(text_style),
    )
    .height(24)
    .padding([4, 9])
    .align_y(Vertical::Center)
    .style(if primary {
        theme::status_chip
    } else {
        theme::panel_flat
    })
    .into()
}
