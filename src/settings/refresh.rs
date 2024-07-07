use crate::{
    app::App,
    structs::{Key, Settings, Themes},
    util::startup,
};

pub fn refresh(app: &mut App, key: &mut Key, theme: &mut Themes, settings: &mut Settings) {
    startup(app, key, theme, settings);
}
