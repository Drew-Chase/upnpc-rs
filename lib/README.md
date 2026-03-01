# upnpc-rs

A fast, lightweight, and easy-to-use Rust library for managing UPnP port mappings on your network gateway.

## Installation

Add `upnpc-rs` to your `Cargo.toml`:

```toml
[dependencies]
upnpc-rs = {git="https://github.com/Drew-Chase/upnpc-rs.git"}
```

## Usage

### Adding a port mapping

Forward external port 8080 to local port 8080 over TCP:

```rust
use upnpc_rs::{add_port, Protocol};

fn main() -> anyhow::Result<()> {
    add_port(
        8080,                          // local port
        None,                          // auto-detect local IP
        Protocol::TCP,                 // protocol
        None,                          // external port (defaults to local port)
        Some("My server".to_string()), // description
        3600,                          // lease duration in seconds (0 = no expiration)
    )?;
    Ok(())
}
```

Forward to both TCP and UDP at once:

```rust
use upnpc_rs::{add_port, Protocol};

fn main() -> anyhow::Result<()> {
    add_port(8080, None, Protocol::Both, None, Some("Game server".to_string()), 0)?;
    Ok(())
}
```

Use a different external port:

```rust
use upnpc_rs::{add_port, Protocol};

fn main() -> anyhow::Result<()> {
    // External port 9090 forwards to local port 8080
    add_port(8080, None, Protocol::TCP, Some(9090), None, 3600)?;
    Ok(())
}
```

### Removing a port mapping

```rust
use upnpc_rs::{remove_port, Protocol};

fn main() -> anyhow::Result<()> {
    remove_port(8080, Protocol::TCP)?;
    Ok(())
}
```

### Listing all port mappings

```rust
use upnpc_rs::list_ports;

fn main() -> anyhow::Result<()> {
    let ports = list_ports()?;
    for port in &ports {
        println!("{port}");
    }
    Ok(())
}
```

Each entry is a [`PortEntry`] struct containing details like the external/internal port, protocol, internal client, description, and lease duration.

`PortEntry` implements `Display`, so printing it produces output like:

```text
tcp 8080->192.168.1.100:8080 "My server" (Lease: 1h)
```

### Serialization

Both `PortEntry` and `Protocol` derive `serde::Serialize`, so you can easily convert them to JSON or any other format supported by serde:

```rust
use upnpc_rs::list_ports;

fn main() -> anyhow::Result<()> {
    let ports = list_ports()?;
    let json = serde_json::to_string_pretty(&ports)?;
    println!("{json}");
    Ok(())
}
```

## Requirements

- A UPnP-enabled gateway/router on the local network
- IPv4 network (the library binds to the local IPv4 address)
