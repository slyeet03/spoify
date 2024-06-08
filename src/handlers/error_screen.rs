use crate::{app::App, enums::Menu};

pub fn go_to_error_event(app: &mut App) {
    if app.selected_menu == Menu::Error {
        app.selected_menu = Menu::Default;
    } else {
        app.selected_menu = Menu::Error;
    }
}
