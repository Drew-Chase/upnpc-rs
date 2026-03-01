#![doc = include_str!("../README.md")]

use anyhow::Result;
use igd_next::{PortMappingProtocol, SearchOptions, search_gateway};
use local_ip_address::local_ip;
use serde::Serialize;
use std::fmt::Display;
use std::net::{IpAddr, SocketAddr, SocketAddrV4};
use std::time::Duration;

/// The network protocol for a port mapping.
#[derive(Debug, Clone, Serialize)]
pub enum Protocol {
    /// Apply the operation to both TCP and UDP.
    /// Skipped during serialization since it represents a combined action rather than a single protocol.
    #[serde(skip)]
    Both,
    /// Transmission Control Protocol.
    TCP,
    /// User Datagram Protocol.
    UDP,
}

/// A UPnP port mapping entry retrieved from the gateway.
#[derive(Serialize, Debug, Clone)]
pub struct PortEntry {
    /// The remote host for which the mapping is valid
    /// Can be an IP address or a host name
    pub remote_host: String,
    /// The external port of the mapping
    pub external_port: u16,
    /// The protocol of the mapping
    pub protocol: Protocol,
    /// The internal (local) port
    pub internal_port: u16,
    /// The internal client of the port mapping
    /// Can be an IP address or a host name
    pub internal_client: String,
    /// A flag whether this port mapping is enabled
    pub enabled: bool,
    /// A description for this port mapping
    pub description: String,
    /// The lease duration of this port mapping in seconds
    pub lease_duration: u32,
}

/// Adds a port mapping on the UPnP gateway.
///
/// Discovers the local network gateway and requests a port forwarding rule
/// that maps an external port to the given internal `port`.
///
/// # Arguments
///
/// * `port` - The internal (local) port to forward traffic to.
/// * `ip` - The local IP address to forward to. Defaults to the auto-detected local IP.
/// * `protocol` - The protocol to forward (`TCP`, `UDP`, or `Both`).
/// * `external_port` - The external port on the gateway. Defaults to the same as `port`.
/// * `description` - An optional description for the mapping.
/// * `lease_duration` - Lease duration in seconds. Use `0` for no expiration.
///
/// # Errors
///
/// Returns an error if gateway discovery fails or the gateway rejects the mapping.
pub fn add_port(
    port: u16,
    ip: Option<String>,
    protocol: Protocol,
    external_port: Option<u16>,
    description: Option<String>,
    lease_duration: u32,
) -> Result<()> {
    let local_ip = local_ip()?.to_string();
    let gateway = search_gateway(SearchOptions {
        bind_addr: SocketAddr::V4(SocketAddrV4::new(local_ip.parse()?, 0)),
        ..Default::default()
    })?;
    let ip = ip.unwrap_or(local_ip.clone()).parse::<IpAddr>()?;
    let ip = SocketAddr::new(ip, port);
    let description = description.unwrap_or_default();
    let external_port = external_port.unwrap_or(port);

    match protocol {
        Protocol::TCP => gateway.add_port(
            PortMappingProtocol::TCP,
            external_port,
            ip,
            lease_duration,
            description.as_str(),
        )?,
        Protocol::UDP => gateway.add_port(
            PortMappingProtocol::UDP,
            external_port,
            ip,
            lease_duration,
            description.as_str(),
        )?,
        Protocol::Both => {
            gateway.add_port(
                PortMappingProtocol::TCP,
                external_port,
                ip,
                lease_duration,
                description.as_str(),
            )?;
            gateway.add_port(
                PortMappingProtocol::UDP,
                external_port,
                ip,
                lease_duration,
                description.as_str(),
            )?;
        }
    };

    Ok(())
}
/// Removes a port mapping from the UPnP gateway.
///
/// # Arguments
///
/// * `port` - The external port of the mapping to remove.
/// * `protocol` - The protocol of the mapping (`TCP`, `UDP`, or `Both`).
///
/// # Errors
///
/// Returns an error if gateway discovery fails or the mapping cannot be removed.
pub fn remove_port(port: u16, protocol: Protocol) -> Result<()> {
    let local_ip = local_ip()?.to_string();
    let gateway = search_gateway(SearchOptions {
        bind_addr: SocketAddr::V4(SocketAddrV4::new(local_ip.parse()?, 0)),
        ..Default::default()
    })?;

    match protocol {
        Protocol::TCP => gateway.remove_port(PortMappingProtocol::TCP, port)?,
        Protocol::UDP => gateway.remove_port(PortMappingProtocol::UDP, port)?,
        Protocol::Both => {
            gateway.remove_port(PortMappingProtocol::UDP, port)?;
            gateway.remove_port(PortMappingProtocol::TCP, port)?;
        }
    };

    Ok(())
}
/// Lists all active port mappings on the UPnP gateway.
///
/// Iterates through every port mapping entry on the gateway and returns them
/// as a vector of [`PortEntry`] values.
///
/// # Errors
///
/// Returns an error if gateway discovery fails or the local IP cannot be determined.
pub fn list_ports() -> Result<Vec<PortEntry>> {
    let local_ip = local_ip()?.to_string();
    let gateway = search_gateway(SearchOptions {
        bind_addr: SocketAddr::V4(SocketAddrV4::new(local_ip.parse()?, 0)),
        ..Default::default()
    })?;

    let mut entries = vec![];
    let mut index = 0;
    loop {
        match gateway.get_generic_port_mapping_entry(index) {
            Ok(entry) => {
                entries.push(PortEntry {
                    remote_host: entry.remote_host,
                    external_port: entry.external_port,
                    protocol: match entry.protocol {
                        PortMappingProtocol::TCP => Protocol::TCP,
                        PortMappingProtocol::UDP => Protocol::UDP,
                    },
                    internal_port: entry.internal_port,
                    internal_client: entry.internal_client,
                    enabled: entry.enabled,
                    description: entry.port_mapping_description,
                    lease_duration: entry.lease_duration,
                });

                index += 1;
            }
            Err(_) => break, // This would indicate that no more entries exist
        }
    }

    Ok(entries)
}

impl Display for PortEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // 0 UDP  9308->192.168.1.238:9308  '192.168.1.238:9308 to 9308 (UDP)' '' 0
        let lease_duration = if self.lease_duration == 0 {
            "no expiration".to_string()
        } else {
            let duration = Duration::from_secs(self.lease_duration as u64);
            let hours = duration.as_secs() / 3600;
            let minutes = (duration.as_secs() % 3600) / 60;
            let seconds = duration.as_secs() % 60;

            let mut parts = Vec::new();
            if hours > 0 {
                parts.push(format!("{}h", hours));
            }
            if minutes > 0 {
                parts.push(format!("{}m", minutes));
            }
            if seconds > 0 || parts.is_empty() {
                parts.push(format!("{}s", seconds));
            }
            parts.join(" ")
        };
        write!(
            f,
            "{} {}->{}:{} \"{}\" (Lease: {})",
            self.protocol,
            self.external_port,
            self.internal_client,
            self.internal_port,
            self.description,
            lease_duration
        )
    }
}

impl Display for Protocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Protocol::Both => write!(f, "both"),
            Protocol::TCP => write!(f, "tcp"),
            Protocol::UDP => write!(f, "udp"),
        }
    }
}
