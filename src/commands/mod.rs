use std::env;
use std::path::PathBuf;

pub mod add;
pub mod build;
pub mod commands;
pub mod get;
pub mod list;
pub mod remove;

fn unwrap_dir(dir: Option<PathBuf>) -> PathBuf {
    dir.or_else(|| env::var("STRING_POOL_DIR").ok().map(PathBuf::from))
        .unwrap_or(env::home_dir().expect("Error: home directory was not found. Try setting the environment variable `STRING_POOL_DIR`, or use the `--dir` flag").join(".local/share/string-pool"))
}
