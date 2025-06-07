use crate::{
    config::ConfigStruct,
    logger::LogWriter,
    platform::{
        logger::write_log,
        types::{EventType, LogEvent, LogLevel},
        utils::{current_timestamp, format_log_event},
    },
};
use notify::{
    Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::fs;


pub async fn watch(config_struct: ConfigStruct, writer: LogWriter) {
    let msg = format!("[{}] Filewatcher started, watching {:?} directory.", current_timestamp(), config_struct.watcher_dir);
    write_log(&writer, &msg).await;
    // File system event channel (from notify crate)
    let (tx, rx) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(2)),
    )
    .expect("Failed to create watcher");

    // Make room for dynamic path setting from users' cli config??
    let path = PathBuf::from(&config_struct.watcher_dir);

    watcher
        .watch(&path, RecursiveMode::Recursive)
        .expect("Failed to watch directory");

    for res in rx {
        match res {
            Ok(event) => {
                let event_type = match event.kind {
                    EventKind::Create(_) => EventType::FileWatch,
                    EventKind::Modify(_) => EventType::FileWatch,
                    EventKind::Remove(_) => EventType::FileWatch,
                    EventKind::Access(_) => EventType::FileWatch,
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
                let file_size_mb = file_size as f64 / 1024.0 / 1024.0;
                let details = format!("File with size {:.2}MB {} at {}", file_size_mb, action, path_str);
                println!("🔍 File event detected: {} - {}", action, path_str);

                let log_event = LogEvent {
                    level: LogLevel::INFO,
                    event_type,
                    timestamp: current_timestamp(),
                    details,
                };

                let formatted = format_log_event(&config_struct, &log_event);
                crate::logger::write_log(&writer, &formatted).await;
            }
            Err(e) => {
                let error_event = LogEvent {
                    level: LogLevel::ERROR,
                    event_type: EventType::FileWatch,
                    timestamp: current_timestamp(),
                    details: format!("File watcher error: {}", e),
                };
                let formatted = format_log_event(&config_struct, &error_event);
                crate::logger::write_log(&writer, &formatted).await;
            }
        }
    }
}
