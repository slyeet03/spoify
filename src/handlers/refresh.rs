use crate::{
    app::App,
    settings::refresh::refresh,
    structs::{Key, Settings, Themes},
};

pub fn refresh_event(app: &mut App, key: &mut Key, theme: &mut Themes, settings: &mut Settings) {
    refresh(app, key, theme, settings);
}
