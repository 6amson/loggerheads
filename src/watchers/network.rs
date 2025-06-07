// use crate::config::ConfigStruct;
// use crate::{
//     logger::LogWriter,
//     platform::logger::write_log,
//     platform::types::{EventType, LogEvent, LogLevel},
//     platform::utils::{current_timestamp, format_log_event},
// };
// use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
// use pnet::datalink::{self, Channel::Ethernet};
// use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
// use pnet::packet::icmp::IcmpPacket;
// use pnet::packet::ipv4::Ipv4Packet;
// use pnet::packet::tcp::TcpPacket;
// use pnet::packet::udp::UdpPacket;
// use pnet::packet::Packet;
// use std::collections::HashMap;
// use tokio::time::{interval, Duration};

// #[derive(Debug, Clone, PartialEq)]
// pub struct NetworkConnection {
//     pub local_addr: String,
//     pub remote_addr: String,
//     pub state: String,
//     pub protocol: String,
//     pub process_name: String,
//     pub pid: Option<u32>,
// }

// #[cfg(feature = "pnet")]
// fn watch_packets(interface_name: &str) -> Option<String> {
//     let interfaces = datalink::interfaces();
//     let interface = interfaces
//         .into_iter()
//         .find(|iface| iface.name == interface_name)?;

//     let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
//         Ok(Ethernet(_, rx)) => ((), rx),
//         Ok(_) => return Some("Unhandled channel type".to_string()),
//         Err(e) => return Some(format!("Failed to create datalink channel: {}", e)),
//     };

//     if let Ok(packet) = rx.next() {
//         // Parse packet function inline:
//         if let Some(eth) = EthernetPacket::new(packet) {
//             let src_mac = eth.get_source();
//             let dst_mac = eth.get_destination();
//             let mut previous_connections: HashMap<String, String> = HashMap::new();
//             let mut current_connections: HashMap<String, String> = HashMap::new();

//             match eth.get_ethertype() {
//                 EtherTypes::Ipv4 => {
//                     if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
//                         let src_ip = ipv4.get_source();
//                         let dst_ip = ipv4.get_destination();
//                         let key = format!("Device: {}", src_mac);

//                         let connection_info = match ipv4.get_next_level_protocol() {
//                             pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
//                                 if let Some(icmp) = IcmpPacket::new(ipv4.payload()) {
//                                     format!(
//                                         "Ethernet: {} -> {}\n\
//                              IP:       {} -> {}\n\
//                              ICMP:     Type {} Code {}\n\
//                              Length:   {} bytes",
//                                         src_mac,
//                                         dst_mac,
//                                         src_ip,
//                                         dst_ip,
//                                         icmp.get_icmp_type().0,
//                                         icmp.get_icmp_code().0,
//                                         packet.len()
//                                     )
//                                 } else {
//                                     format!(
//                                         "Ethernet: {} -> {}\n\
//                              IP:       {} -> {}\n\
//                              ICMP:     [Parse Error]\n\
//                              Length:   {} bytes",
//                                         src_mac,
//                                         dst_mac,
//                                         src_ip,
//                                         dst_ip,
//                                         packet.len()
//                                     )
//                                 }
//                             }
//                             pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
//                                 if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
//                                     let src_port = tcp.get_source();
//                                     let dst_port = tcp.get_destination();
//                                     let flags = tcp.get_flags();

//                                     format!(
//                                         "Ethernet: {} -> {}\n\
//                              IP:       {} -> {}\n\
//                              TCP:      {} -> {} (flags: 0x{:02x})\n\
//                              Length:   {} bytes",
//                                         src_mac,
//                                         dst_mac,
//                                         src_ip,
//                                         dst_ip,
//                                         src_port,
//                                         dst_port,
//                                         flags,
//                                         packet.len()
//                                     )
//                                 } else {
//                                     format!(
//                                         "Ethernet: {} -> {}\n\
//                              IP:       {} -> {}\n\
//                              TCP:      [Parse Error]\n\
//                              Length:   {} bytes",
//                                         src_mac,
//                                         dst_mac,
//                                         src_ip,
//                                         dst_ip,
//                                         packet.len()
//                                     )
//                                 }
//                             }
//                             pnet::packet::ip::IpNextHeaderProtocols::Udp => {
//                                 if let Some(udp) = UdpPacket::new(ipv4.payload()) {
//                                     format!(
//                                         "Ethernet: {} -> {}\n\
//                              IP:       {} -> {}\n\
//                              UDP:      {} -> {}\n\
//                              Length:   {} bytes",
//                                         src_mac,
//                                         dst_mac,
//                                         src_ip,
//                                         dst_ip,
//                                         udp.get_source(),
//                                         udp.get_destination(),
//                                         packet.len()
//                                     )
//                                 } else {
//                                     format!(
//                                         "Ethernet: {} -> {}\n\
//                              IP:       {} -> {}\n\
//                              UDP:      [Parse Error]\n\
//                              Length:   {} bytes",
//                                         src_mac,
//                                         dst_mac,
//                                         src_ip,
//                                         dst_ip,
//                                         packet.len()
//                                     )
//                                 }
//                             }
//                             _ => {
//                                 format!(
//                                     "Ethernet: {} -> {}\n\
//                          IP:       {} -> {}\n\
//                          Protocol: {:?}\n\
//                          Length:   {} bytes",
//                                     src_mac,
//                                     dst_mac,
//                                     src_ip,
//                                     dst_ip,
//                                     ipv4.get_next_level_protocol(),
//                                     packet.len()
//                                 )
//                             }
//                         };

//                         // Store current connection
//                         current_connections.insert(key.clone(), connection_info.clone());

//                         // Check for new device
//                         if !previous_connections.contains_key(&key) {
//                             let protocol_name = match ipv4.get_next_level_protocol() {
//                                 pnet::packet::ip::IpNextHeaderProtocols::Icmp => "ICMP",
//                                 pnet::packet::ip::IpNextHeaderProtocols::Tcp => "TCP",
//                                 pnet::packet::ip::IpNextHeaderProtocols::Udp => "UDP",
//                                 _ => "UNKNOWN",
//                             };

//                             let details = format!(
//                                 "NEW DEVICE DETECTED OVER {}-PACKET | {}",
//                                 protocol_name, connection_info
//                             );
//                             return Some(details);
//                         }
//                     }
//                 }
//                 _ => {
//                     return Some(format!(
//                         "{:?} PACKET FOUND | Eth src={} dst={} | Ethertype {:?} | Length {}",
//                         eth.get_ethertype(),
//                         src_mac,
//                         dst_mac,
//                         eth.get_ethertype(),
//                         packet.len()
//                     ));
//                 }
//             }

//             // Check for devices that left the network (do this once after processing all packets)
//             for (key, info) in &previous_connections {
//                 if !current_connections.contains_key(key) {
//                     let details = format!("DEVICE LEFT THE NETWORK | {}", info);
//                     return Some(details);
//                 }
//             }
//         }

//         // fallback summary if no ethernet packet found
//         Some(format!(
//             "NO PACKET FOUND | Length: {} | First bytes: {:?}",
//             packet.len(),
//             &packet[0..std::cmp::min(10, packet.len())]
//         ))
//     } else {
//         None
//     }
// }

// #[cfg(feature = "netstat2")]
// pub async fn watch(config: ConfigStruct, writer: LogWriter) {
//     let msg = format!("[{}] Networkwatcher started", current_timestamp());
//     write_log(&writer, &msg).await;

//     let mut previous_connections: HashMap<String, String> = HashMap::new();
//     let mut ticker = interval(Duration::from_secs(config.interval));

//     loop {
//         ticker.tick().await;

//         #[cfg(feature = "pnet")]
//         {
//             if let Some(packet_log) = watch_packets("eth0") {
//                 let log_event = LogEvent {
//                     level: LogLevel::DEBUG,
//                     event_type: EventType::NetworkChange,
//                     timestamp: current_timestamp(),
//                     details: packet_log,
//                 };
//                 write_log(&writer, &format_log_event(&config, &log_event)).await;
//             }
//         }

//         let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
//         let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

//         match get_sockets_info(af_flags, proto_flags) {
//             Ok(sockets) => {
//                 let mut current_connections = HashMap::new();

//                 for socket in sockets {
//                     match &socket.protocol_socket_info {
//                         ProtocolSocketInfo::Tcp(tcp) => {
//                             let key = format!("TCP:{}:{}", tcp.local_addr, tcp.remote_addr);

//                             let connection_info = format!(
//                                 "TCP | {}:{} -> {}:{} PID: {} | State: {:?}",
//                                 tcp.local_addr,
//                                 tcp.local_port,
//                                 tcp.remote_addr,
//                                 tcp.remote_port,
//                                 socket
//                                     .associated_pids
//                                     .first()
//                                     .map_or("unknown".to_string(), |pid| pid.to_string()),
//                                 tcp.state,
//                             );

//                             current_connections.insert(key.clone(), connection_info.clone());

//                             if !previous_connections.contains_key(&key) {
//                                 let log_event = LogEvent {
//                                     level: LogLevel::INFO,
//                                     event_type: EventType::NetworkChange,
//                                     timestamp: current_timestamp(),
//                                     details: format!("NEW CONNECTION | {}", connection_info),
//                                 };
//                                 write_log(&writer, &format_log_event(&config, &log_event)).await;
//                             }
//                             // else {
//                             //     let log_event = LogEvent {
//                             //         level: LogLevel::INFO,
//                             //         event_type: EventType::NetworkChange,
//                             //         timestamp: current_timestamp(),
//                             //         details: format!("SUSTAINED CONNECTION | {}", connection_info),
//                             //     };
//                             //     write_log(&writer, &format_log_event(&config, &log_event)).await;
//                             // }
//                         }

//                         ProtocolSocketInfo::Udp(udp) => {
//                             let key = format!("UDP:{}:{}", udp.local_addr, udp.local_port);

//                             let connection_info = format!(
//                                 "UDP | {}:{} | PID: {}",
//                                 udp.local_addr,
//                                 udp.local_port,
//                                 socket
//                                     .associated_pids
//                                     .first()
//                                     .map_or("unknown".to_string(), |pid| pid.to_string())
//                             );

//                             current_connections.insert(key.clone(), connection_info.clone());

//                             if !previous_connections.contains_key(&key) {
//                                 let log_event = LogEvent {
//                                     level: LogLevel::INFO,
//                                     event_type: EventType::NetworkChange,
//                                     timestamp: current_timestamp(),
//                                     details: format!("NEW UDP CONNECTION | {}", connection_info),
//                                 };
//                                 write_log(&writer, &format_log_event(&config, &log_event)).await;
//                             }
//                         }
//                     }
//                 }

//                 for (key, info) in &previous_connections {
//                     if !current_connections.contains_key(key) {
//                         let log_event = LogEvent {
//                             level: LogLevel::INFO,
//                             event_type: EventType::NetworkChange,
//                             timestamp: current_timestamp(),
//                             details: format!("CONNECTION CLOSED | {}", info),
//                         };
//                         write_log(&writer, &format_log_event(&config, &log_event)).await;
//                     }
//                 }

//                 previous_connections = current_connections;
//             }
//             Err(e) => {
//                 write_log(
//                     &writer,
//                     &format!("[{}] Error getting socket info: {}", current_timestamp(), e),
//                 )
//                 .await;
//             }
//         }
//     }
// }

// src/watchers/network.rs
use crate::config::ConfigStruct;
use crate::platform::{
    logger::write_log,
    types::{EventType, LogEvent, LogLevel},
    utils::{current_timestamp, format_log_event},
};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use pnet::datalink::{self, Channel::Ethernet, DataLinkReceiver, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::icmp::IcmpPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tokio::time::interval;

// Configuration constants
const MAX_BATCH_SIZE: usize = 10;
const LOG_BUFFER_SIZE: usize = 5;
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

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
    config: ConfigStruct,
    writer: crate::platform::logger::LogWriter,
    previous_connections: HashMap<String, NetworkConnection>,
    log_buffer: Vec<String>,
    interface: Option<NetworkInterface>,
}

impl NetworkMonitor {
    pub fn new(
        config: ConfigStruct,
        writer: crate::platform::logger::LogWriter,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let interface = Self::get_best_interface(&config)?;

        if let Some(ref iface) = interface {
            println!(
                "Selected network interface: {} ({})",
                iface.name,
                if iface.is_up() { "UP" } else { "DOWN" }
            );
        }

        Ok(Self {
            config,
            writer,
            previous_connections: HashMap::with_capacity(1000),
            log_buffer: Vec::with_capacity(LOG_BUFFER_SIZE),
            interface,
        })
    }

    fn get_best_interface(
        config: &ConfigStruct,
    ) -> Result<Option<NetworkInterface>, Box<dyn std::error::Error + Send + Sync>> {
        let interfaces = datalink::interfaces();

        // Log available interfaces for debugging
        println!("Available network interfaces:");
        for iface in &interfaces {
            println!(
                "  - {} (up: {}, loopback: {}, ips: {:?})",
                iface.name,
                iface.is_up(),
                iface.is_loopback(),
                iface.ips
            );
        }

        // Try to use configured interface first
        if let Some(ref interface_name) = config.network_interface {
            if let Some(iface) = interfaces.iter().find(|i| i.name == *interface_name) {
                return Ok(Some(iface.clone()));
            }
            eprintln!(
                "Warning: Configured interface '{}' not found",
                interface_name
            );
        }

        // Find best available interface
        let best_interface = interfaces
            .into_iter()
            .filter(|iface| iface.is_up() && !iface.is_loopback())
            .find(|iface| !iface.ips.is_empty());

        match best_interface {
            Some(iface) => {
                println!("Auto-selected interface: {}", iface.name);
                Ok(Some(iface))
            }
            None => {
                eprintln!("Warning: No suitable network interface found");
                Ok(None)
            }
        }
    }

    async fn flush_log_buffer(&mut self) {
        if !self.log_buffer.is_empty() {
            let combined_log = self.log_buffer.join("\n");
            write_log(&self.writer, &combined_log).await;
            self.log_buffer.clear();
        }
    }

    fn add_to_log_buffer(&mut self, message: String) {
        self.log_buffer.push(message);
    }

    fn create_log_event(&self, level: LogLevel, details: String) -> String {
        let log_event = LogEvent {
            level,
            event_type: EventType::NetworkWatch,
            timestamp: current_timestamp(),
            details,
        };
        format_log_event(&self.config, &log_event)
    }
}

#[cfg(feature = "pnet")]
fn process_packet_batch(
    rx: &mut Box<dyn DataLinkReceiver>,
    previous_devices: &mut HashMap<String, String>,
    batch_size: usize,
) -> Vec<String> {
    let mut logs = Vec::new();
    let mut current_devices = HashMap::new();
    let mut packets_processed = 0;

    while packets_processed < batch_size {
        match rx.next() {
            Ok(packet) => {
                if let Some(log_entry) =
                    process_single_packet(packet, &mut current_devices, previous_devices)
                {
                    logs.push(log_entry);
                }
                packets_processed += 1;
            }
            Err(_) => break, // No more packets available
        }
    }

    // Check for devices that left the network
    for (key, info) in previous_devices.iter() {
        if !current_devices.contains_key(key) {
            logs.push(format!("DEVICE LEFT THE NETWORK | {}", info));
        }
    }

    // Update previous devices for next iteration
    *previous_devices = current_devices;
    logs
}

#[cfg(feature = "pnet")]
fn process_single_packet(
    packet: &[u8],
    current_devices: &mut HashMap<String, String>,
    previous_devices: &HashMap<String, String>,
) -> Option<String> {
    let eth = EthernetPacket::new(packet)?;
    let src_mac = eth.get_source();
    let dst_mac = eth.get_destination();

    match eth.get_ethertype() {
        EtherTypes::Ipv4 => {
            let ipv4 = Ipv4Packet::new(eth.payload())?;
            let src_ip = ipv4.get_source();
            let dst_ip = ipv4.get_destination();
            let key = format!("Device: {}", src_mac);

            let connection_info = match ipv4.get_next_level_protocol() {
                pnet::packet::ip::IpNextHeaderProtocols::Icmp => {
                    format_icmp_packet(&ipv4, src_mac, dst_mac, src_ip, dst_ip, packet.len())
                }
                pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                    format_tcp_packet(&ipv4, src_mac, dst_mac, src_ip, dst_ip, packet.len())
                }
                pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                    format_udp_packet(&ipv4, src_mac, dst_mac, src_ip, dst_ip, packet.len())
                }
                _ => {
                    format!(
                        "Ethernet: {} -> {}\nIP: {} -> {}\nProtocol: {:?}\nLength: {} bytes",
                        src_mac,
                        dst_mac,
                        src_ip,
                        dst_ip,
                        ipv4.get_next_level_protocol(),
                        packet.len()
                    )
                }
            };

            current_devices.insert(key.clone(), connection_info.clone());

            // Check for new device
            if !previous_devices.contains_key(&key) {
                let protocol_name = match ipv4.get_next_level_protocol() {
                    pnet::packet::ip::IpNextHeaderProtocols::Icmp => "ICMP",
                    pnet::packet::ip::IpNextHeaderProtocols::Tcp => "TCP",
                    pnet::packet::ip::IpNextHeaderProtocols::Udp => "UDP",
                    _ => "UNKNOWN",
                };

                Some(format!(
                    "NEW DEVICE DETECTED OVER {}-PACKET | {}",
                    protocol_name, connection_info
                ))
            } else {
                None
            }
        }
        _ => Some(format!(
            "NON-IPv4 PACKET | Eth: {} -> {} | Type: {:?} | Length: {}",
            src_mac,
            dst_mac,
            eth.get_ethertype(),
            packet.len()
        )),
    }
}

#[cfg(feature = "pnet")]
fn format_icmp_packet(
    ipv4: &Ipv4Packet,
    src_mac: pnet::util::MacAddr,
    dst_mac: pnet::util::MacAddr,
    src_ip: std::net::Ipv4Addr,
    dst_ip: std::net::Ipv4Addr,
    packet_len: usize,
) -> String {
    if let Some(icmp) = IcmpPacket::new(ipv4.payload()) {
        format!(
            "Ethernet: {} -> {}\nIP: {} -> {}\nICMP: Type {} Code {}\nLength: {} bytes",
            src_mac,
            dst_mac,
            src_ip,
            dst_ip,
            icmp.get_icmp_type().0,
            icmp.get_icmp_code().0,
            packet_len
        )
    } else {
        format!(
            "Ethernet: {} -> {}\nIP: {} -> {}\nICMP: [Parse Error]\nLength: {} bytes",
            src_mac, dst_mac, src_ip, dst_ip, packet_len
        )
    }
}

#[cfg(feature = "pnet")]
fn format_tcp_packet(
    ipv4: &Ipv4Packet,
    src_mac: pnet::util::MacAddr,
    dst_mac: pnet::util::MacAddr,
    src_ip: std::net::Ipv4Addr,
    dst_ip: std::net::Ipv4Addr,
    packet_len: usize,
) -> String {
    if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
        format!(
            "Ethernet: {} -> {}\nIP: {} -> {}\nTCP: {}:{} -> {}:{} (flags: 0x{:02x})\nLength: {} bytes",
            src_mac, dst_mac, src_ip, dst_ip,
            src_ip, tcp.get_source(), dst_ip, tcp.get_destination(),
            tcp.get_flags(), packet_len
        )
    } else {
        format!(
            "Ethernet: {} -> {}\nIP: {} -> {}\nTCP: [Parse Error]\nLength: {} bytes",
            src_mac, dst_mac, src_ip, dst_ip, packet_len
        )
    }
}

#[cfg(feature = "pnet")]
fn format_udp_packet(
    ipv4: &Ipv4Packet,
    src_mac: pnet::util::MacAddr,
    dst_mac: pnet::util::MacAddr,
    src_ip: std::net::Ipv4Addr,
    dst_ip: std::net::Ipv4Addr,
    packet_len: usize,
) -> String {
    if let Some(udp) = UdpPacket::new(ipv4.payload()) {
        format!(
            "Ethernet: {} -> {}\nIP: {} -> {}\nUDP: {}:{} -> {}:{}\nLength: {} bytes",
            src_mac,
            dst_mac,
            src_ip,
            dst_ip,
            src_ip,
            udp.get_source(),
            dst_ip,
            udp.get_destination(),
            packet_len
        )
    } else {
        format!(
            "Ethernet: {} -> {}\nIP: {} -> {}\nUDP: [Parse Error]\nLength: {} bytes",
            src_mac, dst_mac, src_ip, dst_ip, packet_len
        )
    }
}

fn process_socket_connections(
    sockets: &[netstat2::SocketInfo],
    previous_connections: &mut HashMap<String, NetworkConnection>,
    now: Instant,
) -> (Vec<String>, Vec<String>) {
    let mut new_connections = Vec::new();
    let mut closed_connections = Vec::new();
    let mut current_connections = HashMap::with_capacity(sockets.len());

    // Process current sockets
    for socket in sockets {
        let (key, connection, info) = match &socket.protocol_socket_info {
            ProtocolSocketInfo::Tcp(tcp) => {
                let key = format!(
                    "TCP:{}:{}:{}",
                    tcp.local_addr, tcp.local_port, tcp.remote_addr
                );
                let connection = NetworkConnection {
                    local_addr: format!("{}:{}", tcp.local_addr, tcp.local_port),
                    remote_addr: format!("{}:{}", tcp.remote_addr, tcp.remote_port),
                    state: format!("{:?}", tcp.state),
                    protocol: "TCP".to_string(),
                    pid: socket.associated_pids.first().copied(),
                    last_seen: now,
                };

                let info = format!(
                    "TCP | {} -> {} | PID: {} | State: {:?}",
                    connection.local_addr,
                    connection.remote_addr,
                    connection
                        .pid
                        .map_or("unknown".to_string(), |p| p.to_string()),
                    tcp.state
                );

                (key, connection, info)
            }
            ProtocolSocketInfo::Udp(udp) => {
                let key = format!("UDP:{}:{}", udp.local_addr, udp.local_port);
                let connection = NetworkConnection {
                    local_addr: format!("{}:{}", udp.local_addr, udp.local_port),
                    remote_addr: String::new(),
                    state: "LISTEN".to_string(),
                    protocol: "UDP".to_string(),
                    pid: socket.associated_pids.first().copied(),
                    last_seen: now,
                };

                let info = format!(
                    "UDP | {} | PID: {}",
                    connection.local_addr,
                    connection
                        .pid
                        .map_or("unknown".to_string(), |p| p.to_string())
                );

                (key, connection, info)
            }
        };

        current_connections.insert(key.clone(), connection.clone());

        // Check for new connections
        if !previous_connections.contains_key(&key) {
            new_connections.push(format!("NEW {} CONNECTION | {}", connection.protocol, info));
        }
    }

    // Check for closed connections (with timeout consideration)
    for (key, old_connection) in previous_connections.iter() {
        if !current_connections.contains_key(key) {
            // Connection might be temporarily unavailable, check timeout
            if now.duration_since(old_connection.last_seen) > CONNECTION_TIMEOUT {
                closed_connections.push(format!(
                    "CONNECTION CLOSED | {} | {} -> {}",
                    old_connection.protocol, old_connection.local_addr, old_connection.remote_addr
                ));
            }
        }
    }

    // Update previous connections
    *previous_connections = current_connections;

    (new_connections, closed_connections)
}

pub async fn watch(config: ConfigStruct, writer: crate::platform::logger::LogWriter) {
    println!("[{}] Network watcher starting...", current_timestamp());

    // Check network prerequisites before starting
    if let Err(e) = check_prerequisites() {
        eprintln!("Network watcher prerequisites failed: {}", e);
        eprintln!("Network monitoring may not work properly.");
        eprintln!("Run with appropriate privileges:");
        eprintln!("  Linux: sudo ./program or setcap cap_net_raw+ep ./program");
        eprintln!("  Windows: Run as Administrator");
        eprintln!("  macOS: sudo ./program");
    }

    // Initialize monitor
    let mut monitor = match NetworkMonitor::new(config.clone(), writer.clone()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Failed to initialize NetworkMonitor: {}", e);
            return;
        }
    };

    let startup_msg = format!(
        "[{}] Network watcher started successfully",
        current_timestamp()
    );
    write_log(&writer, &startup_msg).await;

    // Setup packet capture if pnet feature is enabled and interface is available
    #[cfg(feature = "pnet")]
    let mut packet_receiver = if let Some(ref interface) = monitor.interface {
        match datalink::channel(interface, Default::default()) {
            Ok(Ethernet(_, rx)) => {
                println!(
                    "Packet capture initialized on interface: {}",
                    interface.name
                );
                Some(rx)
            }
            Ok(_) => {
                eprintln!(
                    "Warning: Unhandled channel type for interface: {}",
                    interface.name
                );
                None
            }
            Err(e) => {
                eprintln!("Warning: Failed to create packet capture channel: {}", e);
                eprintln!("Continuing with connection monitoring only...");
                None
            }
        }
    } else {
        None
    };

    #[cfg(feature = "pnet")]
    let mut previous_devices: HashMap<String, String> = HashMap::new();

    let mut ticker = interval(Duration::from_secs(config.interval));
    let mut iteration_count = 0u64;

    loop {
        ticker.tick().await;
        iteration_count += 1;
        let now = Instant::now();

        // Process packet capture (if available)
        #[cfg(feature = "pnet")]
        if let Some(ref mut rx) = packet_receiver {
            let packet_logs = process_packet_batch(rx, &mut previous_devices, MAX_BATCH_SIZE);
            for log_entry in packet_logs {
                let formatted_log = monitor.create_log_event(LogLevel::DEBUG, log_entry);
                monitor.add_to_log_buffer(formatted_log);
            }
        }

        // Process socket connections
        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

        match get_sockets_info(af_flags, proto_flags) {
            Ok(sockets) => {
                let (new_connections, closed_connections) =
                    process_socket_connections(&sockets, &mut monitor.previous_connections, now);

                // Log new connections
                for conn_info in new_connections {
                    let formatted_log = monitor.create_log_event(LogLevel::INFO, conn_info);
                    monitor.add_to_log_buffer(formatted_log);
                    // Flush the buffer if it has reached the limit
                    if monitor.log_buffer.len() >= LOG_BUFFER_SIZE {
                        monitor.flush_log_buffer().await;
                    }
                }

                // Log closed connections
                for conn_info in closed_connections {
                    let formatted_log = monitor.create_log_event(LogLevel::INFO, conn_info);
                    monitor.add_to_log_buffer(formatted_log);
                }

                // Periodic status log
                if iteration_count % 10 == 0 {
                    let status_msg = format!(
                        "Network Monitor Status | Active connections: {} | Iteration: {}",
                        monitor.previous_connections.len(),
                        iteration_count
                    );
                    let formatted_log = monitor.create_log_event(LogLevel::DEBUG, status_msg);
                    monitor.add_to_log_buffer(formatted_log);
                }
            }
            Err(e) => {
                let error_msg = format!("Error getting socket info: {}", e);
                let formatted_log = monitor.create_log_event(LogLevel::ERROR, error_msg);
                monitor.add_to_log_buffer(formatted_log);
            }
        }

        // Flush log buffer periodically
        if iteration_count % 5 == 0 || monitor.log_buffer.len() >= LOG_BUFFER_SIZE {
            monitor.flush_log_buffer().await;
        }
    }
}

// Helper function to check if the watcher can run
pub fn check_prerequisites() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("Checking network watcher prerequisites...");

    // Check if running with sufficient privileges
    #[cfg(unix)]
    {
        let euid = unsafe { libc::geteuid() };
        if euid != 0 {
            println!("Warning: Not running as root. Packet capture may not work.");
            println!("Consider running with: sudo ./program or setcap cap_net_raw+ep ./program");
        }
    }

    // Check available interfaces
    let interfaces = datalink::interfaces();
    if interfaces.is_empty() {
        return Err("No network interfaces found".into());
    }

    let active_interfaces: Vec<_> = interfaces
        .iter()
        .filter(|i| i.is_up() && !i.is_loopback())
        .collect();

    if active_interfaces.is_empty() {
        return Err("No active network interfaces found".into());
    }

    println!("Prerequisites check passed!");
    println!(
        "Active interfaces: {:?}",
        active_interfaces
            .iter()
            .map(|i| &i.name)
            .collect::<Vec<_>>()
    );

    Ok(())
}
