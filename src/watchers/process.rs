use crate::{
    config::ConfigStruct,
    logger::write_log,
    logger::LogWriter,
    platform::types::{EventType, LogEvent, LogLevel},
    platform::utils::current_timestamp,
    platform::utils::format_log_event,
    platform::utils::gather_process_info,
};
use tokio::time::{interval, Duration};

pub async fn watch(config: ConfigStruct, writer: LogWriter) {
    let msg = format!(
        "[{}] Process watcher started (CPU threshold: {:.1}%)",
        current_timestamp(),
        config.cpu_threshold
    );
    write_log(&writer, &msg).await;

    let mut ticker = interval(Duration::from_secs(config.interval));

    loop {
        ticker.tick().await;

        let processes = gather_process_info(&config.cpu_threshold);

        // Debug: Log how many processes we found
        let debug_msg = format!(
            "[{}] Scanning processes - Found {} above {:.1}% CPU threshold",
            current_timestamp(),
            processes.len(),
            config.cpu_threshold
        );
        write_log(&writer, &debug_msg).await;

        if processes.is_empty() {
            // Log that no high-CPU processes were found
            let no_process_msg = format!(
                "[{}] No processes found above {:.1}% CPU threshold",
                current_timestamp(),
                config.cpu_threshold
            );
            write_log(&writer, &no_process_msg).await;
            continue;
        }

        for (pid, (cpu, mem, cmd, user, start_time)) in processes.iter() {
            let details = format!(
                "PID: {} | CPU: {:.2}% | MEM: {:.2} MB\n\
                 USER: {} | START: {}\n\
                 CMD: {}",
                pid, cpu, mem, user, start_time, cmd
            );

            let log_event = LogEvent {
                level: LogLevel::WARN,
                event_type: EventType::ProcessWatch,
                timestamp: current_timestamp(),
                details,
            };

            let formatted = format_log_event(&config, &log_event);
            write_log(&writer, &formatted).await;
        }

        // Add a small delay to prevent overwhelming the logs
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
