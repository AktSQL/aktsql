use super::*;
use iced::widget::column;

mod alter_table;
mod create_table;
pub(super) use alter_table::alter_table_designer_panel;
pub(super) use create_table::create_table_designer_panel;

static MYSQL_COLUMN_TYPES: [&str; 9] = [
    "VARCHAR(255)",
    "TEXT",
    "BIGINT",
    "INT",
    "DECIMAL(18,2)",
    "DATETIME",
    "TIMESTAMP",
    "JSON",
    "BLOB",
];
static POSTGRES_COLUMN_TYPES: [&str; 9] = [
    "TEXT",
    "VARCHAR(255)",
    "BIGINT",
    "INTEGER",
    "NUMERIC(18,2)",
    "TIMESTAMPTZ",
    "BOOLEAN",
    "JSONB",
    "UUID",
];
static SQLITE_COLUMN_TYPES: [&str; 5] = ["TEXT", "INTEGER", "REAL", "NUMERIC", "BLOB"];
static SQL_SERVER_COLUMN_TYPES: [&str; 7] = [
    "NVARCHAR(255)",
    "BIGINT",
    "INT",
    "DECIMAL(18,2)",
    "DATETIME2",
    "BIT",
    "VARBINARY(MAX)",
];
static ORACLE_COLUMN_TYPES: [&str; 6] = [
    "VARCHAR2(255)",
    "NUMBER",
    "NUMBER(18,2)",
    "TIMESTAMP",
    "CLOB",
    "BLOB",
];
static MONGODB_FIELD_TYPES: [&str; 6] = ["String", "Number", "Boolean", "Date", "Object", "Array"];

pub(super) fn create_database_dialog<'a>(
    state: &'a Akt,
    draft: &'a CreateDatabaseDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let driver = state.connection_manager().form().driver;
    let charset_options = driver
        .charset_options()
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();
    let collation_options = driver
        .collation_options(draft.charset())
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();
    let mut form_fields = column![
        text(texts.create_database)
            .size(20)
            .wrapping(Wrapping::None)
            .style(theme::primary_text),
        dialog_text_input(
            texts.database,
            "new_database",
            draft.name(),
            CreateDatabaseField::Name,
        ),
        dialog_pick_list(
            texts.charset,
            charset_options,
            draft.charset().to_owned(),
            CreateDatabaseField::Charset,
        ),
        dialog_pick_list(
            texts.collation,
            collation_options,
            draft.collation().to_owned(),
            CreateDatabaseField::Collation,
        ),
    ]
    .spacing(14);

    if matches!(
        driver,
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb
    ) {
        form_fields = form_fields
            .push(dialog_text_input(
                texts.owner,
                "postgres",
                draft.owner(),
                CreateDatabaseField::Owner,
            ))
            .push(dialog_text_input(
                texts.template,
                "template0",
                draft.template(),
                CreateDatabaseField::Template,
            ))
            .push(dialog_text_input(
                texts.tablespace,
                "pg_default",
                draft.tablespace(),
                CreateDatabaseField::Tablespace,
            ));
    }

    if driver == DatabaseDriver::MongoDb {
        form_fields = form_fields.push(dialog_text_input(
            texts.initial_collection,
            "_aktsql_manager_init",
            draft.initial_collection(),
            CreateDatabaseField::InitialCollection,
        ));
    }

    container(
        container(
            form_fields.push(
                row![
                    Space::with_width(Length::Fill),
                    button(button_label(texts.cancel, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::secondary_button)
                        .on_press(Message::CancelCreateDatabase),
                    button(button_label(texts.create_database, 12))
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 14])
                        .style(theme::primary_button)
                        .on_press(Message::SubmitCreateDatabase),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ),
        )
        .style(theme::panel)
        .width(500)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

fn grid_header(label: &'static str, portion: u16) -> Element<'static, Message> {
    container(
        text(label)
            .size(9)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
    )
    .width(Length::FillPortion(portion))
    .into()
}

fn small_command_button(label: &'static str, message: Message) -> Element<'static, Message> {
    button(button_label(label, 10))
        .height(24)
        .padding([0, 8])
        .style(theme::secondary_button)
        .on_press(message)
        .into()
}

fn index_column_chip(label: String, message: Message) -> Element<'static, Message> {
    button(
        text(label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(theme::on_surface_text),
    )
    .height(24)
    .padding([0, 9])
    .style(theme::secondary_button)
    .on_press(message)
    .into()
}

fn index_column_toggle_chip(
    label: String,
    message: Message,
    selected: bool,
) -> Element<'static, Message> {
    button(
        text(label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(if selected {
                theme::on_primary_text
            } else {
                theme::on_surface_text
            }),
    )
    .height(24)
    .padding([0, 9])
    .style(if selected {
        theme::primary_button
    } else {
        theme::secondary_button
    })
    .on_press(message)
    .into()
}

fn row_select_button(label: String, selected: bool, message: Message) -> Element<'static, Message> {
    button(
        text(label)
            .size(10)
            .wrapping(Wrapping::None)
            .style(if selected {
                theme::on_primary_text
            } else {
                theme::on_surface_text
            }),
    )
    .width(34)
    .height(24)
    .padding(0)
    .style(if selected {
        theme::primary_button
    } else {
        theme::secondary_button
    })
    .on_press(message)
    .into()
}

fn icon_command_button(label: &'static str, message: Message) -> Element<'static, Message> {
    icon_command_button_enabled(label, Some(message))
}

fn icon_command_button_enabled(
    label: &'static str,
    message: Option<Message>,
) -> Element<'static, Message> {
    let mut action = button(
        text(label)
            .size(12)
            .wrapping(Wrapping::None)
            .style(theme::on_primary_text),
    )
    .width(28)
    .height(24)
    .padding(0)
    .style(theme::secondary_button);

    if let Some(message) = message {
        action = action.on_press(message);
    }

    action.into()
}

fn icon_danger_button_enabled(
    label: &'static str,
    message: Option<Message>,
) -> Element<'static, Message> {
    let mut action = button(text(label).size(12).wrapping(Wrapping::None))
        .width(28)
        .height(24)
        .padding(0)
        .style(theme::danger_button);

    if let Some(message) = message {
        action = action.on_press(message);
    }

    action.into()
}

fn small_danger_button(label: &'static str, message: Message) -> Element<'static, Message> {
    button(button_label(label, 10))
        .width(46)
        .height(24)
        .padding(0)
        .style(theme::danger_button)
        .on_press(message)
        .into()
}

pub(super) fn rename_database_dialog<'a>(
    state: &'a Akt,
    draft: &'a RenameDatabaseDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());

    container(
        container(
            column![
                text(texts.rename_database)
                    .size(20)
                    .wrapping(Wrapping::None)
                    .style(theme::primary_text),
                readonly_dialog_field(texts.database, draft.database()),
                database_edit_text_input(
                    texts.database_name,
                    "new_database_name",
                    draft.new_name(),
                    DatabaseEditField::NewName,
                ),
                row![
                    Space::with_width(Length::Fill),
                    button(button_label(texts.cancel, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::secondary_button)
                        .on_press(Message::CancelDatabaseEdit),
                    button(button_label(texts.rename_database, 12))
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 14])
                        .style(theme::primary_button)
                        .on_press(Message::SubmitRenameDatabase),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(14),
        )
        .style(theme::panel)
        .width(500)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn alter_database_charset_dialog<'a>(
    state: &'a Akt,
    draft: &'a AlterDatabaseCharsetDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());
    let driver = state.connection_manager().form().driver;
    let charset_options = driver
        .charset_options()
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();
    let collation_options = driver
        .collation_options(draft.charset())
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();

    container(
        container(
            column![
                text(texts.edit_database_charset)
                    .size(20)
                    .wrapping(Wrapping::None)
                    .style(theme::primary_text),
                readonly_dialog_field(texts.database, draft.database()),
                database_edit_pick_list(
                    texts.charset,
                    charset_options,
                    draft.charset().to_owned(),
                    DatabaseEditField::Charset,
                ),
                database_edit_pick_list(
                    texts.collation,
                    collation_options,
                    draft.collation().to_owned(),
                    DatabaseEditField::Collation,
                ),
                row![
                    Space::with_width(Length::Fill),
                    button(button_label(texts.cancel, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::secondary_button)
                        .on_press(Message::CancelDatabaseEdit),
                    button(button_label(texts.edit_database_charset, 12))
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 14])
                        .style(theme::primary_button)
                        .on_press(Message::SubmitAlterDatabaseCharset),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(14),
        )
        .style(theme::panel)
        .width(500)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

pub(super) fn rename_table_dialog<'a>(
    state: &'a Akt,
    draft: &'a RenameTableDraft,
) -> Element<'a, Message> {
    let texts = i18n::texts(state.language());

    container(
        container(
            column![
                text(texts.rename_table)
                    .size(20)
                    .wrapping(Wrapping::None)
                    .style(theme::primary_text),
                readonly_dialog_field(texts.section_tables, draft.table()),
                table_edit_text_input(
                    texts.rename_table,
                    "new_table_name",
                    draft.new_name(),
                    TableEditField::NewName,
                ),
                row![
                    Space::with_width(Length::Fill),
                    button(button_label(texts.cancel, 12))
                        .width(112)
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 12])
                        .style(theme::secondary_button)
                        .on_press(Message::CancelTableEdit),
                    button(button_label(texts.rename_table, 12))
                        .height(FORM_ACTION_HEIGHT)
                        .padding([0, 14])
                        .style(theme::primary_button)
                        .on_press(Message::SubmitRenameTable),
                ]
                .spacing(10)
                .align_y(Alignment::Center),
            ]
            .spacing(14),
        )
        .style(theme::panel)
        .width(500)
        .padding(18),
    )
    .style(theme::modal_scrim)
    .width(Length::Fill)
    .height(Length::Fill)
    .align_x(Horizontal::Center)
    .align_y(Vertical::Center)
    .into()
}

fn readonly_dialog_field(label: &'static str, value: &str) -> Element<'static, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        container(
            text(value.to_owned())
                .size(14)
                .style(theme::on_surface_text)
        )
        .style(theme::panel_low)
        .height(32)
        .width(Length::Fill)
        .padding([7, 10]),
    ]
    .spacing(4)
    .width(Length::Fill)
    .into()
}

fn alter_table_type_options(driver: DatabaseDriver) -> Vec<String> {
    let options = match driver {
        DatabaseDriver::MySql | DatabaseDriver::MariaDb | DatabaseDriver::TiDb => {
            MYSQL_COLUMN_TYPES.as_slice()
        }
        DatabaseDriver::PostgreSql | DatabaseDriver::CockroachDb => {
            POSTGRES_COLUMN_TYPES.as_slice()
        }
        DatabaseDriver::Sqlite => SQLITE_COLUMN_TYPES.as_slice(),
        DatabaseDriver::SqlServer => SQL_SERVER_COLUMN_TYPES.as_slice(),
        DatabaseDriver::Oracle => ORACLE_COLUMN_TYPES.as_slice(),
        DatabaseDriver::MongoDb => MONGODB_FIELD_TYPES.as_slice(),
    };

    options.iter().map(|value| String::from(*value)).collect()
}

fn alter_table_text_input<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    field: AlterTableField,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .on_input(move |value| Message::AlterTableFieldChanged(field, value))
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

fn create_table_text_input<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    field: CreateTableField,
) -> Element<'a, Message> {
    column![
        text(label)
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .on_input(move |value| Message::CreateTableFieldChanged(field, value))
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

fn create_table_column_input<'a>(
    placeholder: &'static str,
    value: &'a str,
    index: usize,
    field: CreateTableColumnField,
) -> Element<'a, Message> {
    grid_text_input(placeholder, value, move |value| {
        Message::CreateTableColumnFieldChanged(index, field, value)
    })
}

fn alter_table_column_input<'a>(
    placeholder: &'static str,
    value: &'a str,
    index: usize,
    field: CreateTableColumnField,
) -> Element<'a, Message> {
    grid_text_input(placeholder, value, move |value| {
        Message::AlterTableColumnFieldChanged(index, field, value)
    })
}

fn create_table_column_type_pick_list(
    driver: DatabaseDriver,
    selected: &str,
    index: usize,
) -> Element<'static, Message> {
    pick_list(
        alter_table_type_options(driver),
        Some(selected.to_owned()),
        move |value| {
            Message::CreateTableColumnFieldChanged(index, CreateTableColumnField::DataType, value)
        },
    )
    .style(theme::pick_list)
    .menu_style(theme::pick_list_menu)
    .padding(PICK_LIST_PADDING)
    .text_size(13)
    .width(Length::Fill)
    .into()
}

fn alter_table_column_type_pick_list(
    driver: DatabaseDriver,
    selected: &str,
    index: usize,
) -> Element<'static, Message> {
    pick_list(
        alter_table_type_options(driver),
        Some(selected.to_owned()),
        move |value| {
            Message::AlterTableColumnFieldChanged(index, CreateTableColumnField::DataType, value)
        },
    )
    .style(theme::pick_list)
    .menu_style(theme::pick_list_menu)
    .padding(PICK_LIST_PADDING)
    .text_size(13)
    .width(Length::Fill)
    .into()
}

fn alter_table_index_type_pick_list(
    driver: DatabaseDriver,
    selected: &str,
) -> Element<'static, Message> {
    pick_list(
        engine::schema::index_type_options(driver),
        Some(engine::schema::normalized_index_type(driver, selected)),
        |value| Message::AlterTableFieldChanged(AlterTableField::IndexType, value),
    )
    .style(theme::pick_list)
    .menu_style(theme::pick_list_menu)
    .padding(PICK_LIST_PADDING)
    .text_size(13)
    .width(Length::Fill)
    .into()
}

fn alter_table_constraint_type_pick_list(
    driver: DatabaseDriver,
    selected: &str,
) -> Element<'static, Message> {
    let options = engine::schema::constraint_type_options(driver);
    if options.is_empty() {
        return container(
            text(engine::schema::default_constraint_type(driver))
                .size(12)
                .wrapping(Wrapping::None)
                .style(theme::secondary_text),
        )
        .style(theme::panel_low)
        .height(32)
        .padding([7, 10])
        .width(Length::Fill)
        .into();
    }

    pick_list(
        options,
        Some(engine::schema::normalized_constraint_type(driver, selected)),
        |value| Message::AlterTableFieldChanged(AlterTableField::ConstraintKind, value),
    )
    .style(theme::pick_list)
    .menu_style(theme::pick_list_menu)
    .padding(PICK_LIST_PADDING)
    .text_size(13)
    .width(Length::Fill)
    .into()
}

fn create_table_index_input<'a>(
    placeholder: &'static str,
    value: &'a str,
    index: usize,
    field: CreateTableIndexField,
) -> Element<'a, Message> {
    grid_text_input(placeholder, value, move |value| {
        Message::CreateTableIndexFieldChanged(index, field, value)
    })
}

fn create_table_index_type_pick_list(
    driver: DatabaseDriver,
    selected: &str,
    index: usize,
) -> Element<'static, Message> {
    pick_list(
        engine::schema::index_type_options(driver),
        Some(engine::schema::normalized_index_type(driver, selected)),
        move |value| {
            Message::CreateTableIndexFieldChanged(index, CreateTableIndexField::IndexType, value)
        },
    )
    .style(theme::pick_list)
    .menu_style(theme::pick_list_menu)
    .padding(PICK_LIST_PADDING)
    .text_size(13)
    .width(Length::Fill)
    .into()
}

fn create_table_constraint_input<'a>(
    placeholder: &'static str,
    value: &'a str,
    index: usize,
    field: CreateTableConstraintField,
) -> Element<'a, Message> {
    grid_text_input(placeholder, value, move |value| {
        Message::CreateTableConstraintFieldChanged(index, field, value)
    })
}

fn grid_text_input<'a>(
    placeholder: &'static str,
    value: &'a str,
    on_input: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message> {
    container(
        text_input(placeholder, value)
            .on_input(on_input)
            .style(theme::form_text_input)
            .width(Length::Fill)
            .padding([6, 0])
            .size(12),
    )
    .height(28)
    .clip(true)
    .width(Length::Fill)
    .into()
}

fn database_edit_pick_list(
    label: &'static str,
    options: Vec<String>,
    selected: String,
    field: DatabaseEditField,
) -> Element<'static, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        pick_list(options, Some(selected), move |value| {
            Message::DatabaseEditFieldChanged(field, value)
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

fn database_edit_text_input<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    field: DatabaseEditField,
) -> Element<'a, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .on_input(move |value| Message::DatabaseEditFieldChanged(field, value))
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

fn table_edit_text_input<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    field: TableEditField,
) -> Element<'a, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .on_input(move |value| Message::TableEditFieldChanged(field, value))
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

fn dialog_pick_list(
    label: &'static str,
    options: Vec<String>,
    selected: String,
    field: CreateDatabaseField,
) -> Element<'static, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        pick_list(options, Some(selected), move |value| {
            Message::CreateDatabaseFieldChanged(field, value)
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

fn dialog_text_input<'a>(
    label: &'static str,
    placeholder: &'static str,
    value: &'a str,
    field: CreateDatabaseField,
) -> Element<'a, Message> {
    column![
        text(label.to_uppercase())
            .size(11)
            .wrapping(Wrapping::None)
            .style(theme::secondary_text),
        container(
            text_input(placeholder, value)
                .on_input(move |value| Message::CreateDatabaseFieldChanged(field, value))
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
