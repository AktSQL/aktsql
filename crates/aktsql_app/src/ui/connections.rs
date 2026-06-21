use super::*;
use iced::widget::column;

pub(super) fn connection_manager_view(
    manager: &ConnectionManager,
    language: Language,
    connection_testing: bool,
    connection_connecting: bool,
) -> Element<'_, Message> {
    container(
        row![
            connection_profile_list(manager, language),
            connection_form(manager, language, connection_testing, connection_connecting),
        ]
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .style(theme::workspace)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn connection_profile_list(
    manager: &ConnectionManager,
    language: Language,
) -> Element<'_, Message> {
    let texts = i18n::texts(language);
    let header = container(
        column![
            text(texts.connections)
                .size(28)
                .style(theme::on_surface_text),
            row![
                button(button_label(texts.new_connection.to_uppercase(), 12))
                    .width(Length::Fill)
                    .height(LIST_COMMAND_HEIGHT)
                    .padding([0, 12])
                    .style(theme::primary_button)
                    .on_press(Message::NewConnectionProfile),
                button(button_label(manager.list_filter_label(), 11))
                    .width(LIST_FILTER_WIDTH)
                    .height(LIST_COMMAND_HEIGHT)
                    .padding([0, 0])
                    .style(theme::secondary_button)
                    .on_press(Message::ToggleConnectionFilter),
            ]
            .spacing(8)
            .align_y(Alignment::Center),
            search_input(manager.search_query(), language),
        ]
        .spacing(12),
    )
    .width(Length::Fill)
    .padding([20, 24]);

    let visible_profiles = manager.visible_profiles();
    let profiles = if visible_profiles.is_empty() {
        column![
            text(texts.no_matching_connections.to_uppercase())
                .size(11)
                .style(theme::secondary_text),
            text(texts.adjust_search_or_filter)
                .size(11)
                .style(theme::secondary_text),
        ]
        .spacing(6)
        .padding(16)
    } else {
        visible_profiles
            .into_iter()
            .fold(column![].spacing(4), |column, profile| {
                let active = manager.selected_profile_id() == Some(profile.id);
                let card_height = if active {
                    CONNECTION_CARD_ACTIVE_HEIGHT
                } else {
                    CONNECTION_CARD_HEIGHT
                };
                let label = profile.form.name.clone();
                let location = if profile.form.driver.requires_port() {
                    format!("{}:{}", profile.form.location, profile.form.port)
                } else {
                    profile.form.location.clone()
                };

                let marker = container(Space::new(4, card_height))
                    .style(if active {
                        theme::active_marker
                    } else {
                        theme::workspace
                    })
                    .width(4)
                    .height(card_height);

                let card_content = row![
                    marker,
                    container(
                        column![
                            row![
                                text(label).size(14).style(if active {
                                    theme::primary_text
                                } else {
                                    theme::on_surface_text
                                }),
                                Space::with_width(Length::Fill),
                                driver_badge(driver_badge_label(profile.form.driver)),
                            ]
                            .width(Length::Fill)
                            .align_y(Alignment::Center),
                            text(location.clone()).size(12).style(theme::secondary_text),
                            if active {
                                active_session_row(language)
                            } else {
                                Space::with_height(0).into()
                            },
                        ]
                        .spacing(8)
                        .width(Length::Fill),
                    )
                    .padding([13, 16])
                    .width(Length::Fill),
                ]
                .spacing(0)
                .height(card_height);

                let card = button(card_content)
                    .width(Length::Fill)
                    .height(card_height)
                    .padding(0)
                    .style(theme::connection_card_button(active))
                    .on_press(Message::SelectConnectionProfile(profile.id));

                column.push(card)
            })
    };

    container(
        column![
            header,
            divider(),
            scrollable(container(profiles).padding(8).width(Length::Fill))
                .height(Length::Fill)
                .style(theme::scrollable),
        ]
        .height(Length::Fill),
    )
    .style(theme::sidebar)
    .width(CONNECTION_LIST_WIDTH)
    .height(Length::Fill)
    .into()
}

fn active_session_row(language: Language) -> Element<'static, Message> {
    row![
        container(Space::new(7, 7))
            .style(theme::success_dot)
            .width(7)
            .height(7),
        text(i18n::texts(language).active_session.to_uppercase())
            .size(10)
            .style(theme::secondary_text),
    ]
    .spacing(8)
    .align_y(Alignment::Center)
    .into()
}

fn driver_badge_label(driver: DatabaseDriver) -> String {
    match driver {
        DatabaseDriver::MySql => String::from("MYSQL 8.0"),
        DatabaseDriver::PostgreSql => String::from("POSTGRESQL 15"),
        DatabaseDriver::SqlServer => String::from("SQL SERVER 2019"),
        _ => driver.to_string().to_uppercase(),
    }
}

fn driver_badge(label: String) -> Element<'static, Message> {
    container(text(label).size(10).style(theme::secondary_text))
        .style(theme::panel_low)
        .padding([2, 6])
        .into()
}

fn search_input(query: &str, language: Language) -> Element<'_, Message> {
    let texts = i18n::texts(language);

    column![
        text(texts.search.to_uppercase())
            .size(10)
            .style(theme::secondary_text),
        container(
            text_input(texts.search_placeholder, query)
                .on_input(Message::ConnectionSearchChanged)
                .style(theme::form_text_input)
                .width(Length::Fill)
                .padding([6, 0])
                .size(13),
        )
        .height(30)
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

fn connection_form(
    manager: &ConnectionManager,
    language: Language,
    connection_testing: bool,
    connection_connecting: bool,
) -> Element<'_, Message> {
    let texts = i18n::texts(language);
    let form = manager.form();
    let errors = manager.validation_errors();
    let driver = pick_list(
        DatabaseDriver::ALL,
        Some(form.driver),
        Message::ConnectionDriverSelected,
    )
    .style(theme::pick_list)
    .menu_style(theme::pick_list_menu)
    .padding(PICK_LIST_PADDING)
    .text_size(14)
    .width(Length::Fill);

    let form_content = column![
        row![
            container(text(">_").size(15).style(theme::on_primary_text))
                .style(theme::form_header_icon)
                .width(42)
                .height(42)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center),
            column![
                text(texts.connection_settings)
                    .size(22)
                    .style(theme::on_surface_text),
                text(format!(
                    "{} / {}",
                    form.name,
                    driver_header_label(form.driver)
                ))
                .size(13)
                .style(theme::secondary_text),
            ]
            .width(Length::Fill)
            .spacing(2),
        ]
        .width(Length::Fill)
        .spacing(14)
        .align_y(Alignment::Center),
        form_section(
            column![
                row![
                    form_slot(
                        field_input(
                            texts.profile_name,
                            texts.profile_name_placeholder,
                            &form.name,
                            ConnectionField::Name,
                        ),
                        1,
                    ),
                    form_slot(driver_field(driver.into(), language), 1),
                ]
                .spacing(18),
                row![
                    form_slot(fixed_width_field(location_field(form, language), 400.0), 2,),
                    form_slot(
                        fixed_width_field(
                            field_input(
                                texts.port,
                                form.driver.default_port(),
                                &form.port,
                                ConnectionField::Port
                            ),
                            128.0,
                        ),
                        1,
                    ),
                ]
                .spacing(18),
                row![
                    form_slot(
                        fixed_width_field(
                            field_input(
                                texts.username,
                                texts.username_placeholder,
                                &form.username,
                                ConnectionField::Username
                            ),
                            320.0,
                        ),
                        1,
                    ),
                    form_slot(
                        password_input(&form.password, texts.password, texts.password_placeholder),
                        1,
                    ),
                ]
                .spacing(18),
            ]
            .spacing(12)
            .into(),
        ),
        form_section(
            row![
                form_slot(
                    toggle_setting_row(
                        texts.ssl_encrypted_connection,
                        texts.requires_valid_certificate_path,
                        form.ssl_enabled,
                        Message::ConnectionSslToggled(!form.ssl_enabled),
                    ),
                    1,
                ),
                form_slot(
                    toggle_setting_row(
                        texts.ssh_tunnel,
                        texts.route_connection_through_proxy_host,
                        form.ssh_tunnel_enabled,
                        Message::ConnectionSshToggled(!form.ssh_tunnel_enabled),
                    ),
                    1,
                ),
            ]
            .spacing(18)
            .into(),
        ),
        legacy_warning_panel(form.driver, language),
        form_section(advanced_connection_panel(form, language)),
        validation_panel(errors, language),
        action_bar(
            manager,
            texts,
            errors,
            connection_testing,
            connection_connecting,
        ),
    ]
    .spacing(12)
    .width(Length::Fill);

    let bounded_form = container(form_content)
        .width(Length::Fill)
        .max_width(CONNECTION_FORM_MAX_WIDTH);

    container(
        scrollable(
            container(bounded_form)
                .width(Length::Fill)
                .align_x(Horizontal::Center)
                .padding(Padding::from([16.0, 28.0])),
        )
        .width(Length::Fill)
        .anchor_top()
        .style(theme::scrollable),
    )
    .style(theme::workspace)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn form_section<'a>(content: Element<'a, Message>) -> Element<'a, Message> {
    container(content)
        .style(theme::form_section)
        .width(Length::Fill)
        .padding([14, 16])
        .into()
}

fn action_bar<'a>(
    manager: &'a ConnectionManager,
    texts: &i18n::Texts,
    errors: &[String],
    connection_testing: bool,
    connection_connecting: bool,
) -> Element<'a, Message> {
    let validation_label = if errors.is_empty() {
        String::from(texts.validation_ready)
    } else {
        format!("{}: {}", texts.validation_issues, errors.len())
    };

    container(
        row![
            column![
                text(validation_label.to_uppercase())
                    .size(10)
                    .style(theme::secondary_text),
                text(&manager.form().name)
                    .size(12)
                    .style(theme::on_surface_text),
            ]
            .spacing(2)
            .width(Length::Fill),
            test_connection_button(texts.test, texts.running, connection_testing),
            connect_profile_button(manager, texts, connection_connecting),
            delete_profile_button(manager.selected_profile_id(), texts.delete),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
    )
    .style(theme::form_action_bar)
    .width(Length::Fill)
    .padding([10, 12])
    .into()
}

fn test_connection_button(
    label: &'static str,
    running_label: &'static str,
    connection_testing: bool,
) -> Element<'static, Message> {
    let mut action = button(button_label(
        if connection_testing {
            running_label
        } else {
            label
        },
        14,
    ))
    .width(FORM_TEST_WIDTH)
    .height(FORM_ACTION_HEIGHT)
    .padding([0, 12])
    .style(theme::secondary_button);

    if !connection_testing {
        action = action.on_press(Message::RequestTestConnection);
    }

    action.into()
}

fn connect_profile_button(
    manager: &ConnectionManager,
    texts: &i18n::Texts,
    connection_connecting: bool,
) -> Element<'static, Message> {
    let label = if connection_connecting {
        texts.running
    } else if manager.is_new_profile() {
        texts.save
    } else {
        texts.connect
    };
    let message = if manager.is_new_profile() {
        Message::SaveConnectionProfile
    } else {
        Message::ConnectConnectionProfile
    };
    let mut action = button(button_label(label, 12))
        .width(FORM_SAVE_WIDTH)
        .height(FORM_ACTION_HEIGHT)
        .padding([0, 12])
        .style(theme::primary_button);

    if !connection_connecting {
        action = action.on_press(message);
    }

    action.into()
}

fn driver_header_label(driver: DatabaseDriver) -> String {
    driver.to_string()
}

fn location_field(form: &ConnectionForm, language: Language) -> Element<'_, Message> {
    if form.driver == DatabaseDriver::Sqlite {
        return sqlite_file_field(form, language);
    }

    field_input(
        connection_location_label(form.driver, language),
        "127.0.0.1",
        &form.location,
        ConnectionField::Location,
    )
}

fn sqlite_file_field(form: &ConnectionForm, language: Language) -> Element<'_, Message> {
    let texts = i18n::texts(language);

    column![
        text(texts.database_file.to_uppercase())
            .size(11)
            .style(theme::secondary_text),
        row![
            text_input("./database.sqlite", &form.location)
                .on_input(|value| Message::ConnectionFieldChanged(ConnectionField::Location, value))
                .style(theme::form_text_input)
                .width(Length::Fill)
                .padding([6, 0])
                .size(14),
            button(button_label("BROWSE", 11))
                .width(78)
                .height(28)
                .padding([0, 8])
                .style(theme::secondary_button)
                .on_press(Message::BrowseDatabaseFile),
        ]
        .spacing(10)
        .align_y(Alignment::Center),
        container(Space::with_height(1))
            .style(theme::input_underline)
            .height(1)
            .width(Length::Fill),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

fn connection_location_label(driver: DatabaseDriver, language: Language) -> &'static str {
    if driver.uses_network() {
        i18n::texts(language).host_endpoint
    } else {
        i18n::texts(language).database_file
    }
}

fn advanced_connection_panel(form: &ConnectionForm, language: Language) -> Element<'_, Message> {
    let texts = i18n::texts(language);

    column![
        row![
            database_field_input(form, language),
            fixed_width_field(
                field_input(
                    texts.timeout_seconds,
                    texts.timeout_placeholder,
                    &form.timeout_seconds,
                    ConnectionField::TimeoutSeconds,
                ),
                180.0,
            ),
            Space::with_width(Length::Fill),
        ]
        .spacing(18)
        .align_y(Alignment::Start),
        row![
            fixed_width_field(charset_field(form, language), 210.0),
            collation_field(form, language),
        ]
        .spacing(18)
        .align_y(Alignment::Start),
        field_input(
            texts.notes,
            texts.notes_placeholder,
            &form.notes,
            ConnectionField::Notes
        ),
    ]
    .spacing(18)
    .width(Length::Fill)
    .into()
}

fn charset_field(form: &ConnectionForm, language: Language) -> Element<'_, Message> {
    let options = form
        .charset_options()
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();

    column![
        text(i18n::texts(language).charset.to_uppercase())
            .size(11)
            .style(theme::secondary_text),
        pick_list(options, Some(form.charset.clone()), |charset| {
            Message::ConnectionFieldChanged(ConnectionField::Charset, charset)
        })
        .style(theme::pick_list)
        .menu_style(theme::pick_list_menu)
        .padding(PICK_LIST_PADDING)
        .text_size(14)
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

fn collation_field(form: &ConnectionForm, language: Language) -> Element<'_, Message> {
    let options = form
        .collation_options()
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();

    column![
        text(i18n::texts(language).collation.to_uppercase())
            .size(11)
            .style(theme::secondary_text),
        pick_list(options, Some(form.collation.clone()), |collation| {
            Message::ConnectionFieldChanged(ConnectionField::Collation, collation)
        })
        .style(theme::pick_list)
        .menu_style(theme::pick_list_menu)
        .padding(PICK_LIST_PADDING)
        .text_size(14)
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

fn driver_field<'a>(driver: Element<'a, Message>, language: Language) -> Element<'a, Message> {
    column![
        text(i18n::texts(language).driver_type.to_uppercase())
            .size(11)
            .style(theme::secondary_text),
        driver,
        container(Space::with_height(1))
            .style(theme::input_underline)
            .height(1)
            .width(Length::Fill),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

fn toggle_setting_row(
    title: &'static str,
    detail: &'static str,
    enabled: bool,
    message: Message,
) -> Element<'static, Message> {
    container(
        row![
            column![
                text(title).size(12).style(theme::on_surface_text),
                text(detail.to_uppercase())
                    .size(10)
                    .style(theme::secondary_text),
            ]
            .spacing(2)
            .width(Length::Fill),
            toggle_switch(enabled, message),
        ]
        .align_y(Alignment::Center)
        .spacing(16),
    )
    .width(Length::Fill)
    .padding([2, 0])
    .into()
}

fn toggle_switch(enabled: bool, message: Message) -> Element<'static, Message> {
    let knob = container(Space::new(10, 10))
        .style(theme::toggle_knob(enabled))
        .width(10)
        .height(10);
    let content = if enabled {
        row![Space::with_width(18), knob, Space::with_width(4)]
    } else {
        row![Space::with_width(4), knob, Space::with_width(18)]
    };

    button(content.align_y(Alignment::Center))
        .width(32)
        .height(16)
        .padding(0)
        .style(theme::toggle_button(enabled))
        .on_press(message)
        .into()
}

fn delete_profile_button(
    profile_id: Option<usize>,
    label: &'static str,
) -> Element<'static, Message> {
    let mut action = button(button_label(label, 11))
        .width(82)
        .height(FORM_ACTION_HEIGHT)
        .padding(0)
        .style(theme::danger_button);

    if let Some(profile_id) = profile_id {
        action = action.on_press(Message::RequestDeleteConnection(profile_id));
    }

    action.into()
}

fn legacy_warning_panel(driver: DatabaseDriver, language: Language) -> Element<'static, Message> {
    if driver != DatabaseDriver::SqlServer {
        return Space::with_height(0).into();
    }

    let texts = i18n::texts(language);

    row![
        container(Space::new(2, Length::Fill))
            .style(theme::danger_marker)
            .width(2)
            .height(Length::Fill),
        container(
            column![
                text(format!("INFO  {}", texts.legacy_warning.to_uppercase()))
                    .size(11)
                    .style(theme::primary_text),
                text(texts.legacy_warning_body)
                    .size(11)
                    .style(theme::secondary_text),
            ]
            .spacing(7),
        )
        .style(theme::panel_flat)
        .width(Length::Fill)
        .padding([12, 16]),
    ]
    .spacing(0)
    .height(76)
    .into()
}
