use crate::commands::unwrap_dir;

pub fn dir() {
    println!("{}", unwrap_dir(None).display());
}