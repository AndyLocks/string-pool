use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::exit;
use crate::commands::unwrap_dir;

pub fn get(dir: Option<PathBuf>, key: &str) -> std::io::Result<()> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {
        fs::create_dir_all(&dir)?;
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                if name == key {
                    std::io::copy(&mut File::open(path)?, &mut std::io::stdout().lock())?;
                    return Ok(())
                }
            }
        }
    }

    exit(1)
}