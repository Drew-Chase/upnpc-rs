use crate::command_line::{Actions, CommandLineArguments, OutputFormat};
use clap::Parser;
use upnpc_rs::{PortEntry, add_port, list_ports, remove_port};

mod command_line;

fn main() -> anyhow::Result<()> {
    let args = CommandLineArguments::parse();
    match args.command {
        Actions::Add {
            ip,
            port,
            protocol,
            description,
            external_port,
            lease_duration,
        } => {
            add_port(
                port,
                ip,
                match protocol {
                    command_line::Protocol::Both => upnpc_rs::Protocol::Both,
                    command_line::Protocol::TCP => upnpc_rs::Protocol::TCP,
                    command_line::Protocol::UDP => upnpc_rs::Protocol::UDP,
                },
                external_port,
                description,
                Some(lease_duration),
            )?;
            println!("Successfully added port {}", port);
        }
        Actions::Remove { port, protocol } => {
            remove_port(
                port,
                match protocol {
                    command_line::Protocol::Both => upnpc_rs::Protocol::Both,
                    command_line::Protocol::TCP => upnpc_rs::Protocol::TCP,
                    command_line::Protocol::UDP => upnpc_rs::Protocol::UDP,
                },
            )?;
            println!("Successfully removed port {}", port);
        }
        Actions::List { ip, format } => {
            let entires = list_ports()?;
            let entires: Vec<PortEntry> = match ip {
                Some(ip) => entires
                    .iter()
                    .filter(|i| i.remote_host.ne(&ip))
                    .cloned()
                    .collect(),
                None => entires,
            };
            let output = match format {
                OutputFormat::Text => entires
                    .iter()
                    .map(|i| format!("{}", i))
                    .collect::<Vec<String>>()
                    .join("\n"),
                OutputFormat::Json => serde_json::to_string(&entires)?,
                OutputFormat::YAML => serde_yaml::to_string(&entires)?,
            };
            println!("{}", output);
        }
    }
    Ok(())
}
