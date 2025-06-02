use crate::{
    config::ConfigStruct,
    logger::LogWriter,
    platform::{
        logger::write_log,
        types::{EventType, LogEvent, LogLevel},
        utils::{current_timestamp, format_log_event, gather_process_info},
    },
};
use notify::{
    Config, EventKind, RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher,
};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs;
// use tokio::sync::mpsc;

pub async fn watch(config_struct: ConfigStruct, writer: LogWriter) {
    let msg = format!("[{}] Filewatcher started", current_timestamp());
    write_log(&writer, &msg).await;
    // File system event channel (from notify crate)
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .expect("Failed to create watcher");

    // Make room for dynamic path setting from users' cli config??
    let path = PathBuf::from("./src/platform");

    watcher
        .watch(&path, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    for res in rx {
        match res {
            Ok(event) => {
                let event_type = match event.kind {
                    EventKind::Create(_) => EventType::FileChange,
                    EventKind::Modify(_) => EventType::FileChange,
                    EventKind::Remove(_) => EventType::FileChange,
                    EventKind::Access(_) => EventType::FileChange,
                    _ => {
                        println!("⏭️  Skipping event type: {:?}", event.kind);
                        continue;
                    }
                };

                let path_str = event
                    .paths
                    .get(0)
                    .map(|p| p.display().to_string())
                    .unwrap_or_else(|| "<unknown path>".into());

                let action = match event.kind {
                    EventKind::Create(_) => "created",
                    EventKind::Modify(_) => "modified",
                    EventKind::Remove(_) => "deleted",
                    EventKind::Access(_) => "accessed",
                    _ => "unknown",
                };
                let file_size = fs::metadata(&path_str).map(|m| m.len()).unwrap_or(0);
                let details = format!("File with size {} {} at {}", file_size, action, path_str);
                println!("🔍 File event detected: {} - {}", action, path_str);

                let process_info = gather_process_info(&path_str);

                let log_event = LogEvent {
                    level: LogLevel::INFO,
                    event_type,
                    timestamp: current_timestamp(),
                    details,
                    process_info,
                };

                let formatted = format_log_event(&config_struct, &log_event);
                crate::logger::write_log(&writer, &formatted).await;
            }
            Err(e) => {
                let error_event = LogEvent {
                    level: LogLevel::ERROR,
                    event_type: EventType::FileChange,
                    timestamp: current_timestamp(),
                    details: format!("File watcher error: {}", e),
                    process_info: gather_process_info("N/A"),
                };
                let formatted = format_log_event(&config_struct, &error_event);
                crate::logger::write_log(&writer, &formatted).await;
            }
        }
    }
}
