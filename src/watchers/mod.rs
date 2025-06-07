pub mod filewatch;
pub mod process;

use crate::{config::ConfigStruct, logger::LogWriter};
use tokio::task;

#[cfg(feature = "netstat2")]
pub mod network;


pub fn start_event_watchers(
    config: &ConfigStruct,
    writer: LogWriter,
) -> Vec<task::JoinHandle<()>> {
    let mut handles = Vec::new();
    handles.push(tokio::spawn(filewatch::watch(config.clone(), writer.clone())));
    handles.push(tokio::spawn(process::watch(config.clone(), writer.clone())));

    #[cfg(feature = "netstat2")]
    handles.push(tokio::spawn(network::watch(config.clone(), writer.clone())));

    handles 
}
