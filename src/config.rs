// src/config.rs
use crate::{cli::CliArgs, platform::types::LogFormat};
use serde::Deserialize;
use std::path::PathBuf;
use toml;

#[derive(Debug, Deserialize, Clone)]
pub struct PartialConfig {
    #[serde(default = "default_log_dir")]
    pub log_dir: PathBuf,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConfigStruct {
    pub log_dir: PathBuf,
    pub log_format: LogFormat,
    pub watcher_dir: PathBuf,
    pub interval: u64,
    pub cpu_threshold: f32,
    pub network_interface: Option<String>,
}

fn default_log_dir() -> PathBuf {
    "logs".into()
}

impl ConfigStruct {
    pub fn load(cli_args: &CliArgs) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string("loggerheads.toml")?;
        let partial: PartialConfig = toml::from_str(&content)?;
        let interface = cli_args.interface.clone();
        let network_interface = Some(interface).as_ref().cloned();

        Ok(ConfigStruct {
            log_dir: partial.log_dir,
            network_interface: network_interface,
            log_format: cli_args.log_format.clone(),
            watcher_dir: PathBuf::from(cli_args.watcher_dir.clone()),
            interval: cli_args.interval,
            cpu_threshold: cli_args.cpu_threshold,
        })
    }
}
