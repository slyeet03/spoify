use crate::app::App;
use crate::settings::change_keybinding::change_keybinding;
use crate::structs::Key;

pub fn change_keybindings(app: &mut App, key: &mut Key) {
    change_keybinding(app, key);
}
