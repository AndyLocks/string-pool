use std::env;
use std::path::PathBuf;
use crate::config::Config;

pub mod get;
pub mod commands;
pub mod add;
pub mod list;
pub mod remove;

fn unwrap_dir(dir: Option<PathBuf>) -> PathBuf {
    dir.or_else(|| env::var("STRING_POOL_DIR").ok().map(PathBuf::from))
        .unwrap_or(
            confy::load::<Config>("string-pool", "config")
                .unwrap_or_default()
                .directory
                .into(),
        )
}