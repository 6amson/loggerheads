// cli.rs
use crate::platform::{types::LogFormat, utils::default_root_dir};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "loggerheads")]
#[command(about = "Cross-platform system monitoring tool", long_about = None)]

pub struct CliArgs {
    #[arg(long, value_enum, default_value_t = LogFormat::Plaintext)]
    pub log_format: LogFormat,

    #[arg(long, default_value_t = default_root_dir())]
    pub watcher_dir: String,

    #[arg(long, default_value_t = 10)]
    pub interval: u64,

    #[arg(long, default_value_t = 10.0)]
    pub cpu_threshold: f32,

    #[arg(long, short = 'i', default_value = "eth0")]
    pub interface: String,
}
