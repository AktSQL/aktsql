use super::*;
use iced::widget::column;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidebarIcon {
    Database,
    Terminal,
    Table,
    Function,
    History,
    Settings,
    Support,
    Plus,
}

pub(super) fn sidebar(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let is_database_workspace = state.selected() == Section::Databases
        && state.database_workspace_active()
        && state.connection_manager().selected_profile_id().is_some();
    let new_connection_slot: Element<'_, Message> = if state.selected() == Section::Settings {
        Space::with_height(0).into()
    } else {
        mouse_area(
            container(
                row![
                    sidebar_text_icon(SidebarIcon::Plus, true, true),
                    text(texts.new_connection)
                        .size(13)
                        .style(theme::on_primary_text),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .height(44)
            .padding([0, 12])
            .style(theme::sidebar_primary_action)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center),
        )
        .on_press(Message::NewConnectionProfile)
        .into()
    };
    let tools = sidebar_tools(state);

    let content = column![
        container(column![sidebar_header(state), new_connection_slot].spacing(14)).padding(
            Padding {
                top: 4.0,
                right: 4.0,
                bottom: 10.0,
                left: 4.0,
            },
        ),
        horizontal_rule(1),
        object_tree_panel(state, is_database_workspace),
        horizontal_rule(1),
        tools,
    ]
    .spacing(10)
    .padding(12);

    container(content)
        .style(theme::sidebar)
        .width(SIDEBAR_WIDTH)
        .height(Length::Fill)
        .into()
}

fn sidebar_tools(state: &Akt) -> Element<'static, Message> {
    let tools = [
        Section::QueryExplorer,
        Section::History,
        Section::Settings,
        Section::Support,
    ];

    tools
        .into_iter()
        .fold(
            column![section_header("Tools")].spacing(6),
            |column, section| {
                column.push(sidebar_item(
                    section,
                    state.selected() == section,
                    state.language(),
                ))
            },
        )
        .into()
}

fn object_tree_panel(state: &Akt, is_database_workspace: bool) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let has_connection = state.connection_manager().selected_profile_id().is_some();
    let title = if has_connection {
        texts.database_explorer
    } else {
        texts.connections
    };
    let mut tree = column![section_header(title)].spacing(6);

    if has_connection {
        tree = tree.push(connection_root_item(state, is_database_workspace));
        tree = tree.push(query_sidebar_schema(state));
    } else {
        tree = tree.push(connection_manager_link(state));
    }

    container(tree)
        .height(Length::Fill)
        .width(Length::Fill)
        .into()
}

fn connection_manager_link(state: &Akt) -> Element<'static, Message> {
    let texts = i18n::texts(state.language());
    sidebar_navigation_row(
        texts.connections,
        SidebarIcon::Database,
        state.selected() == Section::Databases,
        Message::SelectSection(Section::Databases),
    )
}

fn connection_root_item(state: &Akt, active: bool) -> Element<'static, Message> {
    let label = state.connection_manager().active_label();

    row![
        container(Space::new(4, 34))
            .style(if active {
                theme::active_marker
            } else {
                theme::transparent
            })
            .width(4)
            .height(34),
        mouse_area(
            container(
                row![
                    sidebar_text_icon(SidebarIcon::Database, active, false),
                    text(label).size(13).style(if active {
                        theme::primary_text
                    } else {
                        theme::secondary_text
                    }),
                ]
                .spacing(12)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .height(34)
            .padding([0, 14])
            .style(theme::sidebar_item(active))
            .align_y(Vertical::Center),
        )
        .on_press(Message::SelectSection(Section::Databases))
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .into()
}

fn sidebar_navigation_row(
    label: &'static str,
    icon: SidebarIcon,
    active: bool,
    message: Message,
) -> Element<'static, Message> {
    row![
        container(Space::new(4, SIDEBAR_ITEM_HEIGHT))
            .style(if active {
                theme::active_marker
            } else {
                theme::transparent
            })
            .width(4)
            .height(SIDEBAR_ITEM_HEIGHT),
        mouse_area(
            container(
                row![
                    sidebar_text_icon(icon, active, false),
                    text(label).size(16).style(if active {
                        theme::primary_text
                    } else {
                        theme::secondary_text
                    }),
                ]
                .spacing(14)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .height(SIDEBAR_ITEM_HEIGHT)
            .padding([0, 16])
            .style(theme::sidebar_item(active))
            .align_y(Vertical::Center),
        )
        .on_press(message)
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .into()
}

fn section_header(label: &'static str) -> Element<'static, Message> {
    container(
        text(label.to_uppercase())
            .size(10)
            .style(theme::secondary_text),
    )
    .padding(Padding {
        top: 8.0,
        right: 8.0,
        bottom: 4.0,
        left: 8.0,
    })
    .width(Length::Fill)
    .into()
}

fn sidebar_text_icon(
    icon: SidebarIcon,
    active: bool,
    on_primary: bool,
) -> Element<'static, Message> {
    let label = match icon {
        SidebarIcon::Database => "◎",
        SidebarIcon::Terminal => ">_",
        SidebarIcon::Table => "TBL",
        SidebarIcon::Function => "FN",
        SidebarIcon::History => "HIS",
        SidebarIcon::Settings => "SET",
        SidebarIcon::Support => "?",
        SidebarIcon::Plus => "+",
    };
    let style = if on_primary {
        theme::on_primary_text
    } else if active {
        theme::primary_text
    } else {
        theme::secondary_text
    };

    container(
        text(label)
            .size(
                if matches!(
                    icon,
                    SidebarIcon::Table
                        | SidebarIcon::Function
                        | SidebarIcon::History
                        | SidebarIcon::Settings
                ) {
                    10
                } else if icon == SidebarIcon::Terminal {
                    13
                } else {
                    16
                },
            )
            .style(style),
    )
    .width(SIDEBAR_ICON_SIZE)
    .height(SIDEBAR_ICON_SIZE)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

fn section_sidebar_icon(section: Section) -> SidebarIcon {
    match section {
        Section::Databases => SidebarIcon::Database,
        Section::QueryExplorer => SidebarIcon::Terminal,
        Section::Tables => SidebarIcon::Table,
        Section::Functions => SidebarIcon::Function,
        Section::History => SidebarIcon::History,
        Section::Settings => SidebarIcon::Settings,
        Section::Support => SidebarIcon::Support,
    }
}

fn section_label_text(
    section: Section,
    active: bool,
    language: Language,
) -> Element<'static, Message> {
    let style = if active {
        theme::primary_text
    } else {
        theme::secondary_text
    };

    text(section_label(section, language))
        .size(16)
        .style(style)
        .into()
}

fn sidebar_header(state: &Akt) -> Element<'static, Message> {
    let label = state.connection_manager().active_label();
    let texts = i18n::texts(state.language());

    if state.selected() == Section::Settings {
        return column![
            text(texts.connected.to_uppercase())
                .size(10)
                .style(theme::secondary_text),
            text(label.to_uppercase())
                .size(13)
                .style(theme::primary_text),
        ]
        .spacing(4)
        .into();
    }

    column![
        text(format!("{}:{}", texts.connected, label).to_uppercase())
            .size(10)
            .style(theme::secondary_text),
    ]
    .spacing(4)
    .into()
}

fn sidebar_item(section: Section, active: bool, language: Language) -> Element<'static, Message> {
    row![
        container(Space::new(4, SIDEBAR_ITEM_HEIGHT))
            .style(if active {
                theme::active_marker
            } else {
                theme::transparent
            })
            .width(4)
            .height(SIDEBAR_ITEM_HEIGHT),
        mouse_area(
            container(
                row![
                    sidebar_text_icon(section_sidebar_icon(section), active, false),
                    section_label_text(section, active, language),
                ]
                .spacing(14)
                .align_y(Alignment::Center),
            )
            .width(Length::Fill)
            .height(SIDEBAR_ITEM_HEIGHT)
            .padding([0, 16])
            .style(theme::sidebar_item(active))
            .align_y(Vertical::Center),
        )
        .on_press(Message::SelectSection(section))
    ]
    .spacing(0)
    .align_y(Alignment::Center)
    .into()
}
