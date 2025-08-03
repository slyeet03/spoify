use crate::NewRelease;
use crate::UserPlaylist;
use crate::{
    app::App,
    settings::refresh::refresh,
    structs::{Key, Settings, Themes},
};

pub fn refresh_event(app: &mut App, key: &mut Key, theme: &mut Themes, settings: &mut Settings,userplaylist:&mut UserPlaylist, newrelease: &mut NewRelease) {
    refresh(app, key, theme, settings,userplaylist,newrelease);
}
