// src/config.rs
use serde::Deserialize;
use std::{fs, path::PathBuf};
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigStruct {
    pub log_dir: PathBuf,
    pub log_format: String, // "json", "plaintext", etc.
}

impl ConfigStruct {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string("loggerheads.toml")?;
        let config: ConfigStruct = toml::from_str(&content)?;
        Ok(config)
    }
} 