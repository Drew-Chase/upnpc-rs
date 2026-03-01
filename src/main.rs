use std::time::Duration;
use crate::command_line::{Actions, CommandLineArguments, OutputFormat};
use clap::Parser;
use colored::Colorize;
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
            println!("{} {}", "Successfully added port".green(), port.to_string().bold());
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
            println!("{} {}", "Successfully removed port".green(), port.to_string().bold());
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
                    .map(|i| {

                        let lease_duration = if i.lease_duration == 0 {
                            "no expiration".green().to_string()
                        } else {
                            let duration = Duration::from_secs(i.lease_duration as u64);
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
                            parts.join(" ").yellow().to_string()
                        };
                        format!(
                            "{} {}{}{} \"{}\" (Lease: {})",
                            i.protocol.to_string().cyan().bold(),
                            i.external_port.to_string().yellow(),
                            "->".dimmed(),
                            format!("{}:{}", i.internal_client, i.internal_port).blue(),
                            i.description.italic(),
                            lease_duration)
                    })
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
