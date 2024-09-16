use db_proto::{clients::Client, DEFAULT_PORT};

use bytes::Bytes;
use clap::{Parser, Subcommand};
use std::num::ParseIntError;
use std::path::PathBuf;
use std::str;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(name = "db-client", version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,
}

#[derive(Subcommand, Debug)]
enum Command {
    Ping {
        /// Message to ping
        msg: Option<Bytes>,
    },
    /// Get the value of key.
    Get {
        /// Name of key to get
        key: String,
    },
    /// Set key to hold the string value.
    Set {
        /// Name of key to set
        key: String,

        /// Value to set.
        value: Bytes,

        /// Expire the value after specified amount of time
        #[arg(value_parser = duration_from_ms_str)]
        expires: Option<Duration>,
    },
    ///  Publisher to send a message to a specific channel.
    Publish {
        /// Name of channel
        channel: String,

        /// Message to publish
        message: Bytes,
    },
    /// Subscribe a client to a specific channel or channels.
    Subscribe {
        /// Specific channel or channels
        channels: Vec<String>,
    },
    Dump {
        /// Path to the output file (optional, defaults to "db-state.bin")
        #[arg(long, default_value = "db-state.bin")]
        output: PathBuf,
    },
    /// Load the database state from a file.
    Load {
        /// Path to the input file (optional, defaults to "db-state.bin")
        #[arg(long, default_value = "db-state.bin")]
        input: PathBuf,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> db_proto::Result<()> {
    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.host, cli.port);

    let mut client = Client::connect(&addr).await?;

    match cli.command {
        Command::Ping { msg } => {
            let value = client.ping(msg).await?;
            if let Ok(string) = str::from_utf8(&value) {
                println!("\"{}\"", string);
            } else {
                println!("{:?}", value);
            }
        }
        Command::Get { key } => {
            if let Some(value) = client.get(&key).await? {
                if let Ok(string) = str::from_utf8(&value) {
                    println!("\"{}\"", string);
                } else {
                    println!("{:?}", value);
                }
            } else {
                println!("(nil)");
            }
        }
        Command::Set { key, value, expires: None } => {
            client.set(&key, value).await?;
            println!("OK");
        }
        Command::Set { key, value, expires: Some(expires) } => {
            client.set_expires(&key, value, expires).await?;
            println!("OK");
        }
        Command::Publish { channel, message } => {
            client.publish(&channel, message).await?;
            println!("Publish OK");
        }
        Command::Subscribe { channels } => {
            if channels.is_empty() {
                return Err("channel(s) must be provided".into());
            }
            let mut subscriber = client.subscribe(channels).await?;

            while let Some(msg) = subscriber.next_message().await? {
                println!("got message from the channel: {}; message = {:?}", msg.channel, msg.content);
            }
        }
        Command::Dump { output } => {
            client.dump(&output).await?;
            println!("Database state dumped to {:?}", output);
        }
        Command::Load { input } => {
            client.load(&input).await?;
            println!("Database state loaded from {:?}", input);
        }
    }

    Ok(())
}

fn duration_from_ms_str(src: &str) -> Result<Duration, ParseIntError> {
    let ms = src.parse::<u64>()?;
    Ok(Duration::from_millis(ms))
}
