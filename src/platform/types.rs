// src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    ProcessStart,
    UsbConnect,
    NetworkChange,
    FileChange,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    INFO,
    DEBUG,
    WARN,
    ERROR,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEvent {
    pub level: LogLevel,
    pub event_type: EventType,
    pub timestamp: String,
    pub details: String,
    pub process_info: String,
}
