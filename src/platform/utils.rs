// src/utils.rs
use super::types::LogEvent;
use crate::config::ConfigStruct;
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;
use sysinfo::{Cpu, Pid, Process, System, Users};

pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_log_event(config: &ConfigStruct, event: &LogEvent) -> String {
    match config.log_format.as_str() {
        "json" => serde_json::to_string(event).unwrap_or_else(|_| format!("{:?}", event)),
        "csv" => format!(
            "{},{:?},{}",
            event.timestamp, event.event_type, event.details
        ),
        "plaintext" => format!(
            "Log: {:?}\nTime: {}\nType: {:?}\nDetails: {}\n",
            event.level, event.timestamp, event.event_type, event.details
        ),
        _ => format!(
            "[{}][{:?}] {}",
            event.timestamp, event.event_type, event.details
        ),
    }
}

/// Gathers system process info and file metadata for logs
pub fn gather_process_info(cpu_threshold: f32) -> HashMap<Pid, (f32, f32, String, String, String)> {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut process_info = HashMap::new();

    for (pid, process) in sys.processes() {
        let cpu_usage = process.cpu_usage();
        if cpu_usage < cpu_threshold {
            continue;
        }

        let memory_mb = process.memory() as f32 / 1024.0;
        let cmd = process.cmd().join(" ");
        let mut users = Users::new();
        let user_id = process.user_id().expect("Failed to get user ID");
        // .map(|uid| uid.to_string())
        // .unwrap_or_else(|| "<unknown>".to_owned());
        let user = Users::get_user_by_id(&users, user_id);

        let start_time = DateTime::from_timestamp(process.start_time() as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "<unknown>".into());

        process_info.insert(*pid, (cpu_usage, memory_mb, cmd, user, start_time));
    }

    process_info
}
