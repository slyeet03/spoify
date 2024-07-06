use crate::app::App;
use crate::structs::Key;
use std::env;
use std::path::PathBuf;
use std::process::Command;

pub fn open_configure(app: &mut App, key: &mut Key) {
    let mut yaml_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    yaml_path.push("..");
    yaml_path.push(app.file_name.clone());
    yaml_path.push("configure");

    let yaml_file = yaml_path.clone();

    #[cfg(target_os = "windows")]
    let spawn_command = Command::new("cmd")
        .args(["/C", &format!("explorer {}", yaml_file.display())])
        .spawn();

    #[cfg(not(target_os = "windows"))]
    let command = format!("cd {}", yaml_file.display());

    #[cfg(not(target_os = "windows"))]
    let spawn_command = Command::new("sh")
        .args(["-c", &format!("{}", command), "-ls"])
        .spawn();

    match spawn_command {
        Ok(_) => println!(
            "Press {}, then run spoify again for the keybinds to change",
            key.exit_application_key
        ),
        Err(e) => eprintln!("Failed to spawn terminal: {}", e),
    }
}
