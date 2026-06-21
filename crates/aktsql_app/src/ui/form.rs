use super::*;
use iced::widget::column;

pub(super) fn form_slot<'a>(content: Element<'a, Message>, portion: u16) -> Element<'a, Message> {
    container(content)
        .width(Length::FillPortion(portion))
        .clip(true)
        .into()
}

pub(super) fn fixed_width_field<'a>(
    content: Element<'a, Message>,
    width: f32,
) -> Element<'a, Message> {
    container(content).width(width).clip(true).into()
}

pub(super) fn field_input<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    field: ConnectionField,
) -> Element<'a, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .on_input(move |value| Message::ConnectionFieldChanged(field, value))
                .style(theme::form_text_input)
                .width(Length::Fill)
                .padding([7, 0])
                .size(14),
        )
        .height(32)
        .clip(true)
        .width(Length::Fill),
        container(Space::with_height(1))
            .style(theme::input_underline)
            .height(1)
            .width(Length::Fill),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

pub(super) fn database_field_input<'a>(
    form: &'a ConnectionForm,
    language: Language,
) -> Element<'a, Message> {
    container(field_input(
        i18n::texts(language).database_name,
        i18n::texts(language).database_placeholder,
        &form.database,
        ConnectionField::Database,
    ))
    .width(280)
    .into()
}

pub(super) fn password_input<'a>(
    value: &'a str,
    label: &'static str,
    placeholder: &'static str,
) -> Element<'a, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .secure(true)
                .on_input(|value| Message::ConnectionFieldChanged(ConnectionField::Password, value))
                .style(theme::form_text_input)
                .width(Length::Fill)
                .padding([7, 0])
                .size(14),
        )
        .height(32)
        .clip(true)
        .width(Length::Fill),
        container(Space::with_height(1))
            .style(theme::input_underline)
            .height(1)
            .width(Length::Fill),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

pub(super) fn divider() -> Element<'static, Message> {
    container(Space::with_height(1))
        .style(theme::divider)
        .height(1)
        .width(Length::Fill)
        .into()
}

pub(super) fn validation_panel(errors: &[String], language: Language) -> Element<'_, Message> {
    if errors.is_empty() {
        return Space::with_height(0).into();
    }

    let content = errors.iter().fold(
        column![text(i18n::texts(language).validation_issues.to_uppercase())
            .size(11)
            .style(theme::primary_text)]
        .spacing(4),
        |column, error| {
            column.push(
                text(format!("- {}", error))
                    .size(12)
                    .style(theme::on_surface_text),
            )
        },
    );

    container(content)
        .style(theme::validation_error)
        .width(Length::Fill)
        .padding(10)
        .into()
}
