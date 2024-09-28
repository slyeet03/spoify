use serde::Deserialize;
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use crate::app::App;
use crate::util::get_project_dir;

#[derive(Deserialize, Debug)]
struct Creds(HashMap<String, String>);

pub fn read_creds(app: &mut App) -> HashMap<String, String> {
    let project_dir = get_project_dir(&app.file_name);
    let path = project_dir.join("configure").join("creds.yml");

    let file = File::open(&path).expect("Unable to open creds file");
    let reader = BufReader::new(file);
    let Creds(creds) = serde_yaml::from_reader(reader).expect("Unable to parse creds from YAML");
    creds
}

pub fn set_creds(app: &mut App) {
    let creds = read_creds(app);

    if let Some(value_str) = creds.get("Client ID") {
        app.client_id = value_str.as_str().to_string();
    }

    if let Some(value_str) = creds.get("Client Secret") {
        app.client_secret = value_str.as_str().to_string();
    }
}
