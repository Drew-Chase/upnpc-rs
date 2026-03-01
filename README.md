# UPNPc-rs
This is a Rust cli for UPnP (Universal Plug and Play) functionality, providing a simple interface to manage port mappings on UPnP-enabled routers.

This project aims to offer a robust and user-friendly solution for managing network port mappings through UPnP, enabling seamless integration with various devices and services. It includes both a library and a command-line interface (CLI) for easy interaction and automation.

Key Features:
- **Library Support**: Provides a comprehensive set of functions for programmatically managing UPnP port mappings.
- **CLI Interface**: Offers a straightforward command-line tool for manual and automated port mapping operations.
- **Cross-Platform Compatibility**: Supports multiple operating systems, ensuring broad applicability across different environments.
- **Error Handling**: Implements robust error handling to ensure reliable operation and provide meaningful feedback to users.

## Library
For information about the upnpc-rs library, please refer to the [library documentation](/lib/README.md).

## Installation
You can install the cli using cargo:
```bash
cargo install upnpc-rs
```
or by downloading the latest release from [GitHub Releases](https://github.com/Drew-Chase/upnpc-rs/releases)

## How to Use

```
Usage: upnpc <COMMAND>

Commands:
  add     Add a new mapping
  remove  Remove a new mapping
  list    List all port mappings
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### Adding a Port Mapping

```bash
# Maps the port on the current IP address using TCP and UDP
upnpc add <port>

# Maps the port on a specific IP address
upnpc add --ip <ip> <port>

# Maps the port using a specific protocol
upnpc add --ip <ip> --protocol <tcp|udp|both> <port>

# Maps the port with a 60 second expiration
upnpc add --expiration 60 <port>
```

### Removing a Port Mapping

```bash
# Remove a mapping by port
upnpc remove <port>

# Remove a mapping by port and protocol
upnpc remove --protocol <tcp|udp|both> <port>
```

### Listing Port Mappings

```bash
# List all mappings
upnpc list

# List all mappings for a specific IP or host
upnpc list [ip]
```

> ### Tip:
> Use `--help` on any command to see more details, e.g. `upnpc add --help`
