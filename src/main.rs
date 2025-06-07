// src/main.rs
mod config;
mod platform;
mod watchers;
use crate::platform::logger;
use crate::watchers::start_event_watchers;
use clap::Parser;
use config::ConfigStruct as Config;
use futures::future::join_all;
use logger::logger;
mod cli;

#[tokio::main]
async fn main() {
    let args = cli::CliArgs::parse();
    let config = Config::load(&args).expect("Failed to load config");
    let log_writer = logger(&config).expect("Logger failed to initialize");

    println!("Starting monitoring watchers...");
 
    let watcher_handles = start_event_watchers(&config, log_writer);

    let results = join_all(watcher_handles).await;

    // Log results
    for (i, result) in results.into_iter().enumerate() {
        let watcher_name = match i {
            0 => "File",
            1 => "Process",
            2 => "Network",
            _ => "Unknown",
        };

        match result {
            Ok(_) => println!("{} watcher completed", watcher_name),
            Err(e) => eprintln!("{} watcher failed: {:?}", watcher_name, e),
        }
    }
}
