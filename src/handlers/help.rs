use crate::{app::App, enums::Menu};

pub fn go_to_help_event(app: &mut App) {
    if app.selected_menu == Menu::Help {
        app.selected_menu = Menu::Default;
    } else {
        app.selected_menu = Menu::Help;
    }
}
