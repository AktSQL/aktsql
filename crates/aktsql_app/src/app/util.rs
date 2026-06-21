use super::*;
use iced::keyboard::{key, Key, Modifiers};

pub(super) fn toggle_order_key(
    order_by: &mut Vec<ResultSortKey>,
    column_index: usize,
    column_name: String,
) {
    if let Some(position) = order_by
        .iter()
        .position(|key| key.column_index == column_index)
    {
        if let Some(next) = order_by[position].direction.next() {
            order_by[position].direction = next;
        } else {
            order_by.remove(position);
        }
    } else {
        order_by.push(ResultSortKey::new(column_index, column_name));
    }
}

pub(super) fn shift_schema_indices_after(
    indices: &mut BTreeSet<usize>,
    parent_index: usize,
    offset: usize,
) {
    if offset == 0 {
        return;
    }

    *indices = indices
        .iter()
        .map(|index| {
            if *index > parent_index {
                *index + offset
            } else {
                *index
            }
        })
        .collect();
}

pub(super) fn shift_schema_index_after(
    index: &mut Option<usize>,
    parent_index: usize,
    offset: usize,
) {
    if let Some(value) = index.as_mut() {
        if *value > parent_index {
            *value += offset;
        }
    }
}

pub(super) fn csv_row(values: &[String]) -> String {
    let mut row = values
        .iter()
        .map(|value| csv_cell(value))
        .collect::<Vec<_>>()
        .join(",");
    row.push('\n');
    row
}

pub(super) fn handle_key_press(key: Key, modifiers: Modifiers) -> Option<Message> {
    match key.as_ref() {
        Key::Named(key::Named::Enter) if modifiers.command() => Some(Message::ExecuteQuery),
        Key::Named(key::Named::F5) => Some(Message::RunQuickAction(QuickAction::RefreshData)),
        Key::Named(key::Named::F6) => Some(Message::ToggleTheme),
        Key::Named(key::Named::F9) => Some(Message::CommitTransaction),
        _ => None,
    }
}

fn csv_cell(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') || value.contains('\r') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}
