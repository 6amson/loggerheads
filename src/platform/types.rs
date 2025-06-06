// src/types.rs
use serde::{Deserialize, Serialize};
use clap::{ValueEnum};

#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    Process,
    NetworkChange,
    FileChange,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    INFO,
    DEBUG,
    ERROR,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEvent {
    pub level: LogLevel,
    pub event_type: EventType,
    pub timestamp: String,
    pub details: String,
}

#[derive(Debug, Clone, Deserialize, ValueEnum)]
pub enum LogFormat {
    Json,
    Plaintext,
}
