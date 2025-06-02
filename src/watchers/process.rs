use crate::{
    config::ConfigStruct, logger::write_log, logger::LogWriter, platform::utils::current_timestamp,
};

pub async fn watch(config: ConfigStruct, writer: LogWriter) {
    // Simulate an event
    let msg = format!("[{}] Processwatcher started", current_timestamp());
    write_log(&writer, &msg).await;
    // Add real logic later

    use crate::{
        config::ConfigStruct,
        logger::LogWriter,
        platform::{
            logger::write_log,
            types::{EventType, LogEvent, LogLevel},
            utils::{current_timestamp, format_log_event, gather_process_info},
        },
    };
    use tokio::time::{interval, Duration};

    const CPU_THRESHOLD: f32 = 10.0; // only log processes above 10% CPU

    pub async fn watch(config: ConfigStruct, writer: LogWriter) {
        let msg = format!(
            "[{}] Process watcher started (CPU > {:.1}%)",
            current_timestamp(),
            CPU_THRESHOLD
        );
        write_log(&writer, &msg).await;

        let mut ticker = interval(Duration::from_secs(5));

        loop {
            ticker.tick().await;

            let processes = gather_process_info(CPU_THRESHOLD);
            for (pid, (cpu, mem, cmd, user, start_time)) in processes.iter() {
                let details = format!(
                    "PID: {} | CPU: {:.2}% | MEM: {:.2} MB | USER: {} | START: {} | CMD: {}",
                    pid, cpu, mem, user, start_time, cmd
                );

                let event = LogEvent { level: (), event_type: (), timestamp: (), details: (), process_info: () } {
                    level: LogLevel::INFO,
                    event_type: EventType::Process,
                    timestamp: current_timestamp(),
                    details,
                };

                let formatted = format_log_event(&config, &event);
                write_log(&writer, &formatted).await;
            }
        }
    }
}
