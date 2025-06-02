use crate::{config::ConfigStruct, logger::write_log, logger::LogWriter, platform::utils::current_timestamp};

pub async fn watch(config: ConfigStruct, writer: LogWriter) {
    // Simulate an event
    let msg = format!("[{}] USBwatcher started", current_timestamp());
    write_log(&writer, &msg).await;
    // Add real logic later
}
