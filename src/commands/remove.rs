use crate::commands::unwrap_dir;
use std::fs;
use std::path::PathBuf;

pub fn remove(dir: Option<PathBuf>, key: &str) -> std::io::Result<()> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {
        return Ok(());
    }

    fs::remove_file(dir.join(key))?;

    Ok(())
}
