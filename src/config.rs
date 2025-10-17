use std::env;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub directory: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            directory: env::home_dir().unwrap().join(".local/share/string-pool").to_str().unwrap().to_string(),
        }
    }
}