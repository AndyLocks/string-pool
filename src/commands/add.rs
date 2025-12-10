use std::{fs, io};
use std::fs::OpenOptions;
use std::io::IsTerminal;
use std::path::PathBuf;
use crate::commands::unwrap_dir;

pub fn add(dir: Option<PathBuf>, key: &str) -> std::io::Result<()> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(dir.join(key))?;

    if !io::stdin().is_terminal() {
        std::io::copy(&mut std::io::stdin().lock(), &mut file)?;
    }

    Ok(())
}