use crate::commands::unwrap_dir;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio, exit};

fn choose_editor() -> String {
    if let Ok(ed) = env::var("EDITOR") {
        return ed;
    }

    let fallback = ["editor", "nano", "vim", "vi"];

    for &cmd in &fallback {
        if which::which(cmd).is_ok() {
            return cmd.to_string();
        }
    }

    eprintln!("No editor found");
    exit(1)
}

pub fn edit(dir: Option<PathBuf>, key: &str) -> Result<(), String> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {
        if let Err(err) = fs::create_dir_all(&dir) {
            eprintln!("IO Error: {err}");
            exit(1)
        }
    }

    let status = Command::new(choose_editor())
        .arg(&dir.join(key))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Error: Failed to run the editor");

    if !status.success() {
        return Err("Error: Editor exited with error".into());
    }

    Ok(())
}
