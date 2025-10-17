use std::fs;
use std::io::Error;
use std::path::PathBuf;
use crate::commands::unwrap_dir;

pub fn list(dir: Option<PathBuf>) -> Result<(), Error> {
    let dir = unwrap_dir(dir);

    if !dir.exists() {
        return Ok(());
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                println!("{}", name.to_str().unwrap());
            }
        }
    }

    Ok(())
}