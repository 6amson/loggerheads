// src/logger.rs
use crate::config::ConfigStruct;
use std::{fs::OpenOptions, io::Write,sync::Arc};
use tokio::sync::Mutex;

pub type LogWriter = Arc<Mutex<dyn Write + Send + Sync>>;

pub fn logger(config: &ConfigStruct) -> std::io::Result<LogWriter> {
    let log_path = config.log_dir.join("loggerheads.log");
    if let Some(dir) = log_path.parent(){
        std::fs::create_dir_all(dir).expect("Error creating directories.");
    }
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path)?;
    Ok(Arc::new(Mutex::new(file)))
}

pub async fn write_log(writer: &LogWriter, message: &str) {
    let mut w = writer.lock().await;
    let _ = writeln!(w, "{message}");
}