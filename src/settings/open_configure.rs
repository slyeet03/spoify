use crate::app::App;
use crate::structs::Key;
use crate::util::get_project_dir;
use std::process::Command;

pub fn open_configure(app: &mut App, key: &mut Key) {
    let project_dir = get_project_dir(&app.file_name);
    let configure_dir = project_dir.join("configure");

    #[cfg(target_os = "windows")]
    let spawn_command = Command::new("cmd")
        .args(["/C", &format!("explorer {}", configure_dir.display())])
        .spawn();

    #[cfg(not(target_os = "windows"))]
    let spawn_command = Command::new("open").arg(&configure_dir).spawn();

    match spawn_command {
        Ok(_) => println!("Press {} to refresh", key.refresh_key),
        Err(e) => eprintln!("Failed to spawn terminal: {}", e),
    }
}
