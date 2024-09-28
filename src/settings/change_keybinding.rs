use crate::app::App;
use crate::structs::Key;
use crate::util::get_project_dir;
use std::process::Command;

pub fn change_keybinding(app: &mut App, key: &mut Key) {
    let project_dir = get_project_dir(&app.file_name);
    let mut path = project_dir.join("configure");
    let mac_path = path.clone();
    path = path.join("keybindings.yml");

    #[cfg(target_os = "windows")]
    let spawn_command = Command::new("cmd")
        .args(["/C", &format!("notepad {}", path.display())])
        .spawn();

    #[cfg(not(target_os = "windows"))]
    let spawn_command = Command::new("open").arg(mac_path).spawn();

    match spawn_command {
        Ok(_) => println!("Press {} to refresh", key.refresh_key),
        Err(e) => eprintln!("Failed to spawn terminal: {}", e),
    }
}
