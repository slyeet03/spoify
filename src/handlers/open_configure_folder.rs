use crate::app::App;
use crate::settings::open_configure::open_configure;
use crate::structs::Key;

pub fn open_config_folder(app: &mut App, key: &mut Key) {
    open_configure(app, key)
}
