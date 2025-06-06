use crate::config::ConfigStruct;
use crate::{
    logger::LogWriter,
    platform::logger::write_log,
    platform::types::{EventType, LogEvent, LogLevel},
    platform::utils::{current_timestamp, format_log_event},
};
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};
use pnet::datalink::{self, Channel::Ethernet};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::collections::HashMap;
use tokio::time::{interval, Duration};

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkConnection {
    pub local_addr: String,
    pub remote_addr: String,
    pub state: String,
    pub protocol: String,
    pub process_name: String,
    pub pid: Option<u32>,
}

#[cfg(feature = "pnet")]
fn watch_packets(interface_name: &str) -> Option<String> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == interface_name)?;

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(_, rx)) => ((), rx),
        Ok(_) => return Some("Unhandled channel type".to_string()),
        Err(e) => return Some(format!("Failed to create datalink channel: {}", e)),
    };

    if let Ok(packet) = rx.next() {
        // Parse packet function inline:
        if let Some(eth) = EthernetPacket::new(packet) {
            let src_mac = eth.get_source();
            let dst_mac = eth.get_destination();

            match eth.get_ethertype() {
                EtherTypes::Ipv4 => {
                    if let Some(ipv4) = Ipv4Packet::new(eth.payload()) {
                        let src_ip = ipv4.get_source();
                        let dst_ip = ipv4.get_destination();

                        match ipv4.get_next_level_protocol() {
                            pnet::packet::ip::IpNextHeaderProtocols::Tcp => {
                                if let Some(tcp) = TcpPacket::new(ipv4.payload()) {
                                    let src_port = tcp.get_source();
                                    let dst_port = tcp.get_destination();
                                    let flags = tcp.get_flags();

                                    return Some(format!(
                                        "PACKET | Eth src={} dst={} | IP {} -> {} | TCP {}->{} flags=0x{:x} | Length {}",
                                        src_mac,
                                        dst_mac,
                                        src_ip,
                                        dst_ip,
                                        src_port,
                                        dst_port,
                                        flags,
                                        packet.len()
                                    ));
                                }
                            }
                            pnet::packet::ip::IpNextHeaderProtocols::Udp => {
                                if let Some(udp) = UdpPacket::new(ipv4.payload()) {
                                    return Some(format!(
                                        "PACKET | Eth src={} dst={} | IP {} -> {} | UDP {}->{} | Length {}",
                                        src_mac,
                                        dst_mac,
                                        src_ip,
                                        dst_ip,
                                        udp.get_source(),
                                        udp.get_destination(),
                                        packet.len()
                                    ));
                                }
                            }
                            _ => {
                                return Some(format!(
                                    "PACKET | Eth src={} dst={} | IP {} -> {} | Protocol {:?} | Length {}",
                                    src_mac,
                                    dst_mac,
                                    src_ip,
                                    dst_ip,
                                    ipv4.get_next_level_protocol(),
                                    packet.len()
                                ));
                            }
                        }
                    }
                }
                _ => {
                    return Some(format!(
                        "PACKET | Eth src={} dst={} | Ethertype {:?} | Length {}",
                        src_mac,
                        dst_mac,
                        eth.get_ethertype(),
                        packet.len()
                    ));
                }
            }
        }

        // fallback summary if no ethernet packet found
        Some(format!(
            "PACKET | Length: {} | First bytes: {:?}",
            packet.len(),
            &packet[0..std::cmp::min(10, packet.len())]
        ))
    } else {
        None
    }
}

#[cfg(feature = "netstat2")]
pub async fn watch(config: ConfigStruct, writer: LogWriter) {
    let msg = format!("[{}] Networkwatcher started", current_timestamp());
    write_log(&writer, &msg).await;

    let mut previous_connections: HashMap<String, String> = HashMap::new();
    let mut ticker = interval(Duration::from_secs(config.interval));

    loop {
        ticker.tick().await;

        #[cfg(feature = "pnet")]
        {
            if let Some(packet_log) = watch_packets("eth0") {
                let log_event = LogEvent {
                    level: LogLevel::DEBUG,
                    event_type: EventType::NetworkChange,
                    timestamp: current_timestamp(),
                    details: packet_log,
                };
                write_log(&writer, &format_log_event(&config, &log_event)).await;
            }
        }

        let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
        let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

        match get_sockets_info(af_flags, proto_flags) {
            Ok(sockets) => {
                let mut current_connections = HashMap::new();

                for socket in sockets {
                    match &socket.protocol_socket_info {
                        ProtocolSocketInfo::Tcp(tcp) => {
                            let key = format!("TCP:{}:{}", tcp.local_addr, tcp.remote_addr);

                            let connection_info = format!(
                                "TCP | {}:{} -> {}:{} | PID: {} | State: {:?}",
                                tcp.local_addr,
                                tcp.local_port,
                                tcp.remote_addr,
                                tcp.remote_port,
                                socket
                                    .associated_pids
                                    .first()
                                    .map_or("unknown".to_string(), |pid| pid.to_string()),
                                tcp.state,
                            );

                            current_connections.insert(key.clone(), connection_info.clone());

                            if !previous_connections.contains_key(&key) {
                                let log_event = LogEvent {
                                    level: LogLevel::INFO,
                                    event_type: EventType::NetworkChange,
                                    timestamp: current_timestamp(),
                                    details: format!("NEW CONNECTION | {}", connection_info),
                                };
                                write_log(&writer, &format_log_event(&config, &log_event)).await;
                            } else {
                                let log_event = LogEvent {
                                    level: LogLevel::INFO,
                                    event_type: EventType::NetworkChange,
                                    timestamp: current_timestamp(),
                                    details: format!("SUSTAINED CONNECTION | {}", connection_info),
                                };
                                write_log(&writer, &format_log_event(&config, &log_event)).await;
                            }
                        }

                        ProtocolSocketInfo::Udp(udp) => {
                            let key = format!("UDP:{}:{}", udp.local_addr, udp.local_port);

                            let connection_info = format!(
                                "UDP | {}:{} | PID: {}",
                                udp.local_addr,
                                udp.local_port,
                                socket
                                    .associated_pids
                                    .first()
                                    .map_or("unknown".to_string(), |pid| pid.to_string())
                            );

                            current_connections.insert(key.clone(), connection_info.clone());

                            if !previous_connections.contains_key(&key) {
                                let log_event = LogEvent {
                                    level: LogLevel::INFO,
                                    event_type: EventType::NetworkChange,
                                    timestamp: current_timestamp(),
                                    details: format!("NEW UDP CONNECTION | {}", connection_info),
                                };
                                write_log(&writer, &format_log_event(&config, &log_event)).await;
                            }
                        }
                    }
                }

                for (key, info) in &previous_connections {
                    if !current_connections.contains_key(key) {
                        let log_event = LogEvent {
                            level: LogLevel::INFO,
                            event_type: EventType::NetworkChange,
                            timestamp: current_timestamp(),
                            details: format!("CONNECTION CLOSED | {}", info),
                        };
                        write_log(&writer, &format_log_event(&config, &log_event)).await;
                    }
                }

                previous_connections = current_connections;
            }
            Err(e) => {
                write_log(
                    &writer,
                    &format!("[{}] Error getting socket info: {}", current_timestamp(), e),
                )
                .await;
            }
        }
    }
}
