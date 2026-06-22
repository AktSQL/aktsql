use super::*;
use iced::keyboard::{key, Key, Modifiers};

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

pub(super) fn handle_key_press(key: Key, _modifiers: Modifiers) -> Option<Message> {
    match key.as_ref() {
        Key::Named(key::Named::F5) => Some(Message::RunQuickAction(QuickAction::RefreshData)),
        Key::Named(key::Named::F6) => Some(Message::ToggleTheme),
        _ => None,
    }
}
