pub mod filewatch;
pub mod network;
pub mod process;
pub mod usb;

use crate::{config::ConfigStruct, logger::LogWriter};
use tokio::task;

pub fn start_event_watchers(
    config: &ConfigStruct,
    writer: LogWriter,
) -> Vec<task::JoinHandle<()>> {
    vec![
        tokio::spawn(filewatch::watch(config.clone(), writer.clone())),
        tokio::spawn(process::watch(config.clone(), writer.clone())),
        tokio::spawn(usb::watch(config.clone(), writer.clone())),
        tokio::spawn(network::watch(config.clone(), writer.clone())),
    ]
}
