use ratatui::widgets::{ListState, TableState};

use crate::app::App;

// Helper functions for cursor movement and character deletion
pub fn move_cursor_left(app: &mut App) {
    let cursor_moved_left = app.cursor_position.saturating_sub(1);
    app.cursor_position = clamp_cursor(app, cursor_moved_left);
}

pub fn move_cursor_right(app: &mut App) {
    let cursor_moved_right = app.cursor_position.saturating_add(1);
    app.cursor_position = clamp_cursor(app, cursor_moved_right);
}

pub fn delete_char(app: &mut App) {
    let is_not_cursor_leftmost = app.cursor_position != 0;
    if is_not_cursor_leftmost {
        let current_index = app.cursor_position;
        let from_left_to_current_index = current_index - 1;

        // Getting all characters before the selected character.
        let before_char_to_delete = app.input.chars().take(from_left_to_current_index);
        // Getting all characters after selected character.
        let after_char_to_delete = app.input.chars().skip(current_index);

        // Put all characters together except the selected one.
        // By leaving the selected one out, it is forgotten and therefore deleted.
        app.input = before_char_to_delete.chain(after_char_to_delete).collect();
        move_cursor_left(app);
    }
}

pub fn clamp_cursor(app: &mut App, new_cursor_pos: usize) -> usize {
    new_cursor_pos.clamp(0, app.input.len())
}
pub fn reset_cursor(app: &mut App) {
    app.cursor_position = 0;
}

pub fn down_key_for_table(names: Vec<String>, mut state: TableState) -> TableState {
    let length: usize = names.len();
    let next_index: usize = state.selected().unwrap_or(0) + 1;
    state.select(Some(next_index % length));

    state
}

pub fn down_key_for_list(names: Vec<String>, mut state: ListState) -> ListState {
    let length: usize = names.len();
    let next_index: usize = state.selected().unwrap_or(0) + 1;
    state.select(Some(next_index % length));

    state
}

pub fn up_key_for_table(names: Vec<String>, mut state: TableState) -> TableState {
    let length: usize = names.len();
    let prev_index: usize = if state.selected().unwrap_or(0) == 0 {
        length - 1
    } else {
        state.selected().unwrap_or(0) - 1
    };
    state.select(Some(prev_index));

    state
}

pub fn up_key_for_list(names: Vec<String>, mut state: ListState) -> ListState {
    let length: usize = names.len();
    let prev_index: usize = if state.selected().unwrap_or(0) == 0 {
        length - 1
    } else {
        state.selected().unwrap_or(0) - 1
    };
    state.select(Some(prev_index));

    state
}
