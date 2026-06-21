use super::*;
use iced::widget::{column, row};

pub(super) fn result_grid<'a>(
    result: &'a QueryResult,
    language: Language,
    order_by: &'a [ResultSortKey],
) -> Element<'a, Message> {
    if result.columns.is_empty() {
        return container(
            text(format!(
                "{} {}",
                result.row_count(),
                i18n::texts(language).rows_affected
            ))
            .size(14)
            .style(theme::on_surface_text),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center)
        .into();
    }

    let grid_width = RESULT_INDEX_WIDTH + (result.columns.len().max(1) as f32) * RESULT_CELL_WIDTH;
    let header = result.columns.iter().enumerate().fold(
        row![result_index_header_cell()]
            .spacing(0)
            .align_y(Alignment::Center),
        |row, (column_index, column_name)| {
            row.push(result_sort_header_cell(column_name, column_index, order_by))
        },
    );

    let rows = result.rows.iter().enumerate().fold(
        column![].spacing(0),
        |column, (row_index, row_values)| {
            let data_row = row_values.iter().fold(
                row![result_index_cell(row_index)]
                    .spacing(0)
                    .align_y(Alignment::Center),
                |row, value| row.push(result_data_cell(value, row_index)),
            );

            column.push(data_row)
        },
    );

    let grid = column![header, rows]
        .width(grid_width)
        .height(Length::Shrink);

    container(
        scrollable(grid)
            .direction(ScrollDirection::Both {
                vertical: Scrollbar::new(),
                horizontal: Scrollbar::new(),
            })
            .width(Length::Fill)
            .height(Length::Fill)
            .style(theme::scrollable),
    )
    .style(theme::query_canvas)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

pub(super) fn result_header_cell(value: String) -> Element<'static, Message> {
    container(text(value).size(12).style(theme::primary_text))
        .width(RESULT_CELL_WIDTH)
        .height(34)
        .padding([8, 10])
        .style(theme::result_header_cell)
        .into()
}

pub(super) fn result_data_cell<'a>(value: &'a str, row_index: usize) -> Element<'a, Message> {
    let content: Element<'a, Message> = if is_status_value(value) {
        container(
            text(value.to_ascii_uppercase())
                .size(10)
                .style(theme::primary_text),
        )
        .style(theme::status_chip)
        .padding([3, 8])
        .into()
    } else {
        text(value).size(11).style(theme::on_surface_text).into()
    };

    container(content)
        .width(RESULT_CELL_WIDTH)
        .height(30)
        .padding([6, 10])
        .style(theme::result_data_cell(row_index))
        .into()
}

fn result_index_header_cell() -> Element<'static, Message> {
    container(text("#").size(12).style(theme::secondary_text))
        .width(RESULT_INDEX_WIDTH)
        .height(34)
        .padding([8, 10])
        .style(theme::result_header_cell)
        .into()
}

fn result_sort_header_cell<'a>(
    value: &'a str,
    column_index: usize,
    order_by: &'a [ResultSortKey],
) -> Element<'a, Message> {
    let sort_state = order_by
        .iter()
        .enumerate()
        .find(|(_, key)| key.column_index() == column_index)
        .map(|(index, key)| (index + 1, key.direction()));
    let label = match sort_state {
        Some((priority, SortDirection::Asc)) => {
            std::borrow::Cow::Owned(format!("{value} ↑{priority}"))
        }
        Some((priority, SortDirection::Desc)) => {
            std::borrow::Cow::Owned(format!("{value} ↓{priority}"))
        }
        None => std::borrow::Cow::Borrowed(value),
    };

    button(
        container(
            text(label)
                .size(12)
                .wrapping(Wrapping::None)
                .style(theme::primary_text),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_y(Vertical::Center),
    )
    .width(RESULT_CELL_WIDTH)
    .height(34)
    .padding([0, 10])
    .style(theme::result_header_button(sort_state.is_some()))
    .on_press(Message::SortResultByColumn(column_index))
    .into()
}

fn is_status_value(value: &str) -> bool {
    match value.len() {
        2 => value.eq_ignore_ascii_case("ok"),
        4 => value.eq_ignore_ascii_case("done"),
        5 => value.eq_ignore_ascii_case("ready"),
        6 => value.eq_ignore_ascii_case("active") || value.eq_ignore_ascii_case("failed"),
        7 => value.eq_ignore_ascii_case("pending"),
        _ => false,
    }
}

fn result_index_cell(row_index: usize) -> Element<'static, Message> {
    container(
        text((row_index + 1).to_string())
            .size(11)
            .style(theme::secondary_text),
    )
    .width(RESULT_INDEX_WIDTH)
    .height(30)
    .padding([6, 10])
    .align_x(Horizontal::Right)
    .style(theme::result_data_cell(row_index))
    .into()
}
