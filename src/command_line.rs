use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "upnpc", bin_name = "upnpc", about, version)]
#[command(after_help = "\
    Examples:
        # Shows more information about the add command
        upnpc add --help

        # Maps the port on the current ip address using TCP and UPD
        upnpc add <port>

        # Maps the port on the specified ip address using TCP and UPD
        upnpc add --ip <ip> <port>

        # Maps the port on the specified ip address using the specified protocol
        upnpc add --ip <ip> --protocol <tcp|udp|both> <port>

        # Maps the port with a 60 second expiration
        upnpc add --expiration 60 <port>

        # Shows more information about the remove command
        upnpc remove --help

        # Remove a mapping with a specific port
        upnpc remove <port>

        # Remove a mapping with a specific port and protocol
        upnpc remove --protocol <tcp|udp|both> <port>

        # List all mappings
        upnpc list

        # Shows more information about the list command
        upnpc list --help

        # List all mappings for a specific ip or host
        upnpc list [ip]

")]
pub struct CommandLineArguments {
    #[command(subcommand)]
    pub command: Actions,
}

#[derive(Debug, Subcommand)]
pub enum Actions {
    #[clap(name = "add", about = "Add a new mapping")]
    #[command(after_help = "\
    Examples:
        # Maps the port on the current ip address using TCP and UPD
        upnpc add <port>

        # Maps the port on the specified ip address using TCP and UPD
        upnpc add --ip <ip> <port>

        # Maps the port on the specified ip address using the specified protocol
        upnpc add --ip <ip> --protocol <tcp|udp|both> <port>
")]
    Add {
        /// Internal port (required)
        port: u16,
        /// Ip address to forward to (defaults to the local ip
        #[arg(long, short)]
        ip: Option<String>,
        /// The protocol to use
        #[arg(long, short, default_value = "both")]
        protocol: Protocol,
        /// The external port to use (defaults to the internal port)
        #[arg(long)]
        external_port: Option<u16>,
        /// The description of the mapping
        #[arg(long, short)]
        description: Option<String>,
        /// How long in seconds the mapping should last, 0 means forever
        #[arg(short='e', long="expiration", default_value = "0")]
        lease_duration: u32,
    },
    #[clap(name = "remove", about = "Remove a new mapping")]
    #[command(after_help = "\
    Examples:
        # Remove a mapping with a specific port
        upnpc remove <port>

        # Remove a mapping with a specific port and protocol
        upnpc remove --protocol <tcp|udp|*> <port>
")]
    Remove{
        port: u16,
        #[clap(long, short, default_value = "both")]
        protocol: Protocol
    },
    #[clap(name = "list", about = "List all port mappings")]
    #[command(after_help = "\
    Examples:
        # List all mappings
        upnpc list

        # List all mappings for a specific ip or host
        upnpc list [ip]
")]
    List{
        #[clap(long, short, default_value = "text")]
        format: OutputFormat,
        /// Only show mappings from a specific ip address
        ip: Option<String>,
    },
}


#[derive(Debug, clap::ValueEnum, Clone)]
pub enum OutputFormat {
    Text,
    Json,
    #[clap(alias="yml")]
    YAML,
}

#[derive(Debug, Clone)]
#[derive(clap::ValueEnum)]
pub enum Protocol {
    #[clap(alias = "*")]
    Both,
    TCP,
    UDP,
}