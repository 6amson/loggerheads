pub mod filewatch;
pub mod process;
pub mod network;
use crate::{config::ConfigStruct, logger::LogWriter};
use tokio::task;


pub fn start_event_watchers(
    config: &ConfigStruct,
    writer: LogWriter,
) -> Vec<task::JoinHandle<()>> {
    let mut handles = Vec::new();
    handles.push(tokio::spawn(filewatch::watch(config.clone(), writer.clone())));
    handles.push(tokio::spawn(process::watch(config.clone(), writer.clone())));
    handles.push(tokio::spawn(network::watch(config.clone(), writer.clone())));

    handles 
}



