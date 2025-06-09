// src/utils.rs
use super::types::LogEvent;
use crate::config::ConfigStruct;
use crate::platform::types::{LogFormat};
use chrono::DateTime;
use chrono::Utc;
use std::collections::HashMap;
use sysinfo::{ Pid, System, Users};

pub fn current_timestamp() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn format_log_event(config: &ConfigStruct, event: &LogEvent) -> String {
    match config.log_format {
        LogFormat::Json => serde_json::to_string(event).unwrap_or_else(|_| format!("{:?}", event)),
        LogFormat::Plaintext => format!(
            "Log: {:?}\nTime: {}\nType: {:?}\nDetails: {}\n",
            event.level, event.timestamp, event.event_type, event.details
        ),
    }
}

/// Gathers system process info and file metadata for logs
pub fn gather_process_info(cpu_threshold: &f32) -> HashMap<Pid, (f32, f32, String, String, String)> {
    let mut sys = System::new_all();

    // Important: Refresh twice with a small delay for accurate CPU readings
    sys.refresh_all();
    std::thread::sleep(std::time::Duration::from_millis(200));
    sys.refresh_processes();

    let users = Users::new();
    let mut process_info = HashMap::new();

    println!(
        "DEBUG: Total processes in system: {}",
        sys.processes().len()
    );
    println!("DEBUG: CPU threshold: {:.1}%", cpu_threshold);

    let mut high_cpu_count = 0;
    let mut total_checked = 0;

    for (pid, process) in sys.processes() {
        total_checked += 1;
        let cpu_usage = process.cpu_usage();

        let cpu_thres = cpu_threshold;

        // Debug: Print first 10 processes to see what we're getting
        if total_checked <= 10 {
            println!(
                "DEBUG: PID {} - CPU: {:.2}% - Name: {}",
                pid,
                cpu_usage,
                process.name()
            );
        }

        if &cpu_usage < cpu_thres {
            continue;
        }

        high_cpu_count += 1;
        println!(
            "DEBUG: Found high CPU process - PID: {} CPU: {:.2}%",
            pid, cpu_usage
        );

        let memory_mb = process.memory() as f32 / 1024.0 / 1024.0; // Convert to MB
        let cmd = if process.cmd().is_empty() {
            process.name().to_string()
        } else {
            process.cmd().join(" ")
        };

        let user_id = process.user_id();
        let username = match user_id {
            Some(uid) => match Users::get_user_by_id(&users, uid) {
                Some(user) => user.name().to_string(),
                None => format!("uid:{:?}", uid),
            },
            None => "<unknown>".to_string(),
        };

        let start_time = DateTime::from_timestamp(process.start_time() as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "<unknown>".into());

        process_info.insert(*pid, (cpu_usage, memory_mb, cmd, username, start_time));
    }

    println!(
        "DEBUG: Checked {} processes, found {} above {:.1}% CPU",
        total_checked, high_cpu_count, cpu_threshold
    );

    process_info
}

pub fn default_root_dir() -> String {
    if cfg!(target_os = "windows") {
        String::from("./")
    } else {
        String::from("./src/platform")
    }
}