use crate::NewRelease;
use crate::UserPlaylist;
use crate::{
    app::App,
    structs::{Key, Settings, Themes},
    util::startup,
};

pub fn refresh(app: &mut App, key: &mut Key, theme: &mut Themes, settings: &mut Settings,userplaylist: &mut UserPlaylist, newrelease: &mut NewRelease) {
    startup(app, key, theme, settings,userplaylist,newrelease);
}
