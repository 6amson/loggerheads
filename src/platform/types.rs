// src/types.rs
use serde::{Deserialize, Serialize};
use clap::{ValueEnum};
use std::collections::HashMap;
use std::time::{Instant};
#[derive(Debug, Serialize, Deserialize)]
pub enum EventType {
    ProcessWatch,
    NetworkWatch,
    FileWatch,
}
use pnet::datalink::{NetworkInterface};

use crate::config::ConfigStruct;


#[derive(Debug, Serialize, Deserialize)]
pub enum LogLevel {
    INFO,
    DEBUG,
    ERROR,
    WARN,
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

#[derive(Debug, Clone)]
pub struct NetworkConnection {
    pub local_addr: String,
    pub state: String,
    pub remote_addr: String,
    pub protocol: String,
    pub pid: Option<u32>,
    pub last_seen: Instant,
}

pub struct NetworkMonitor {
  pub  config: ConfigStruct,
   pub writer: crate::platform::logger::LogWriter,
  pub  previous_connections: HashMap<String, NetworkConnection>,
  pub  log_buffer: Vec<String>,
  pub  interface: Option<NetworkInterface>,
}