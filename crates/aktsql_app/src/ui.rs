use crate::app::{
    Akt, AlterDatabaseCharsetDraft, AlterTableDraft, AlterTableField, AlterTableTab,
    CreateDatabaseDraft, CreateDatabaseField, CreateTableColumnDraft, CreateTableColumnField,
    CreateTableConstraintDraft, CreateTableConstraintField, CreateTableDraft, CreateTableField,
    CreateTableIndexDraft, CreateTableIndexField, CreateTableTab, DatabaseAction,
    DatabaseEditField, Message, RenameDatabaseDraft, RenameTableDraft, ResultSortKey,
    SchemaDeletionKind, Section, SortDirection, TableAction, TableEditField,
};
use crate::connection::{ConnectionField, ConnectionForm, ConnectionManager, DatabaseDriver};
use crate::i18n::{self, Language};
use crate::product::APP_SHORT_NAME;
use crate::query::{
    DatabaseDetailSectionKind, DatabaseDetails, QueryResult, SchemaObject, SchemaObjectKind,
    TableDetails,
};
use crate::theme;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::scrollable::{Direction as ScrollDirection, Scrollbar};
use iced::widget::text::Wrapping;
use iced::widget::{
    button, column, container, horizontal_rule, mouse_area, pick_list, row, scrollable, text,
    text_editor, text_input, Space, Stack,
};
use iced::{Alignment, Element, Length, Padding};

mod connections;
mod database_workbench;
mod dialogs;
mod form;
mod generic;
mod query_workspace;
mod results;
mod schema_designer;
mod settings;
mod shell;
mod sidebar;
pub(in crate::ui) use database_workbench::{
    database_workbench_view, detail_section_label, query_sidebar_schema,
};
use form::*;
pub(in crate::ui) use query_workspace::{chip, query_workspace_view, schema_refresh_button};
use results::*;

const TOP_BAR_HEIGHT: f32 = 48.0;
const STATUS_BAR_HEIGHT: f32 = 24.0;
const SIDEBAR_WIDTH: f32 = 300.0;
const SIDEBAR_ITEM_HEIGHT: f32 = 44.0;
const SIDEBAR_ICON_SIZE: f32 = 28.0;
const CONNECTION_LIST_WIDTH: f32 = 347.0;
const CONNECTION_FORM_MAX_WIDTH: f32 = 720.0;
const CONNECTION_CARD_ACTIVE_HEIGHT: f32 = 88.0;
const CONNECTION_CARD_HEIGHT: f32 = 68.0;
const TOP_COMMAND_HEIGHT: f32 = 28.0;
const TOP_REFRESH_WIDTH: f32 = 94.0;
const TOP_PRIMARY_WIDTH: f32 = 72.0;
const TITLE_LANGUAGE_WIDTH: f32 = 136.0;
const TITLE_LANGUAGE_DROPDOWN_RIGHT_OFFSET: f32 = 84.0;
const WINDOW_CONTROL_SIZE: f32 = 28.0;
const TITLE_BAR_PADDING: Padding = Padding {
    top: 4.0,
    right: 12.0,
    bottom: 4.0,
    left: 14.0,
};
const PICK_LIST_PADDING: Padding = Padding {
    top: 7.0,
    right: 14.0,
    bottom: 7.0,
    left: 16.0,
};
const LIST_COMMAND_HEIGHT: f32 = 40.0;
const LIST_FILTER_WIDTH: f32 = 70.0;
const FORM_ACTION_HEIGHT: f32 = 34.0;
const FORM_TEST_WIDTH: f32 = 136.0;
const FORM_SAVE_WIDTH: f32 = 220.0;
const RESULT_INDEX_WIDTH: f32 = 54.0;
const RESULT_CELL_WIDTH: f32 = 124.0;
const QUERY_CONTEXT_HEIGHT: f32 = 42.0;
const QUERY_EDITOR_HEIGHT: f32 = 240.0;
const QUERY_LINE_GUTTER_WIDTH: f32 = 48.0;

pub fn view(state: &Akt) -> Element<'_, Message> {
    let shell = column![
        shell::top_bar(state),
        row![sidebar::sidebar(state), workspace(state)].height(Length::Fill),
        status_bar(state),
    ]
    .height(Length::Fill);

    let content = Stack::new()
        .push(shell)
        .push_maybe(
            state
                .language_menu_open()
                .then(|| shell::language_dropdown(state.language())),
        )
        .push_maybe(
            state
                .test_result_open()
                .then(|| dialogs::test_result_dialog(state)),
        )
        .push_maybe(
            state
                .create_database_draft()
                .map(|draft| schema_designer::create_database_dialog(state, draft)),
        )
        .push_maybe(
            state
                .rename_database_draft()
                .map(|draft| schema_designer::rename_database_dialog(state, draft)),
        )
        .push_maybe(
            state
                .alter_database_charset_draft()
                .map(|draft| schema_designer::alter_database_charset_dialog(state, draft)),
        )
        .push_maybe(
            state
                .rename_table_draft()
                .map(|draft| schema_designer::rename_table_dialog(state, draft)),
        )
        .push_maybe(
            state
                .pending_delete_profile_id()
                .map(|id| dialogs::delete_confirmation_dialog(state, id)),
        )
        .push_maybe(state.pending_schema_deletion().map(|pending| {
            dialogs::schema_delete_confirmation_dialog(state, pending.kind(), pending.name())
        }))
        .width(Length::Fill)
        .height(Length::Fill);

    container(content)
        .style(theme::app_background)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn vertical_separator() -> Element<'static, Message> {
    container(Space::with_width(1))
        .style(theme::panel_low)
        .width(1)
        .height(16)
        .into()
}

fn horizontal_separator() -> Element<'static, Message> {
    container(Space::with_height(1))
        .style(theme::divider)
        .width(Length::Fill)
        .height(1)
        .into()
}

fn button_label(label: impl Into<String>, size: u16) -> Element<'static, Message> {
    container(text(label.into()).size(size))
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into()
}

fn section_label(section: Section, language: Language) -> &'static str {
    let texts = i18n::texts(language);

    match section {
        Section::Databases => texts.section_databases,
        Section::QueryExplorer => texts.section_query_explorer,
        Section::Tables => texts.section_tables,
        Section::Functions => texts.section_functions,
        Section::History => texts.section_history,
        Section::Settings => texts.section_settings,
        Section::Support => texts.section_support,
    }
}

fn workspace(state: &Akt) -> Element<'_, Message> {
    if state.selected() == Section::Databases {
        if state.database_workspace_active()
            && state.connection_manager().selected_profile_id().is_some()
        {
            return database_workbench_view(state);
        }

        return connections::connection_manager_view(
            state.connection_manager(),
            state.language(),
            state.connection_testing(),
            state.connection_connecting(),
        );
    }

    if state.selected() == Section::QueryExplorer {
        return query_console_view(state);
    }

    if state.selected() == Section::Tables
        && state.connection_manager().selected_profile_id().is_some()
    {
        return database_workbench_view(state);
    }

    if state.selected() == Section::Settings {
        return settings::appearance_settings_view(state);
    }

    generic::generic_workspace(state)
}

fn query_console_view(state: &Akt) -> Element<'_, Message> {
    container(query_workspace_view(state))
        .style(theme::workspace)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn status_bar(state: &Akt) -> Element<'_, Message> {
    let texts = i18n::texts(state.language());
    let latency = state
        .result_latency_ms()
        .map(|value| format!("{value}ms"))
        .unwrap_or_else(|| String::from("--"));
    let connection = state.connection_manager().active_label();

    container(
        row![
            text(APP_SHORT_NAME).size(11).style(theme::on_surface_text),
            vertical_separator(),
            text(state.status_message())
                .size(11)
                .style(theme::secondary_text),
            text(format!("{}: {}", texts.memory, state.memory_label()))
                .size(11)
                .style(theme::secondary_text),
            text(format!("{}: {}", texts.rows, state.result_row_count()))
                .size(11)
                .style(theme::secondary_text),
            text(format!("{}: {latency}", texts.latency))
                .size(11)
                .style(theme::primary_text),
            Space::with_width(Length::Fill),
            container(Space::new(8, 8))
                .style(theme::success_dot)
                .width(8)
                .height(8),
            text(format!("{}: {connection}", texts.connected).to_uppercase())
                .size(11)
                .style(theme::secondary_text),
        ]
        .spacing(14)
        .align_y(Alignment::Center)
        .padding([3, 16]),
    )
    .style(theme::top_bar)
    .height(STATUS_BAR_HEIGHT)
    .width(Length::Fill)
    .into()
}
