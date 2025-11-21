use kade::prelude::*;

use bytes::Bytes;
use clap::Parser;
use rustyline::{error::ReadlineError, DefaultEditor};
use std::{path::Path, process, str, time::Duration};

#[derive(Parser, Debug)]
#[command(name = "kade-cli", version)]
struct Cli {
    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(trailing_var_arg = true)]
    command: Vec<String>,
}

async fn print_info(client: &mut Client, print_help: bool) -> Result<()> {
    let version = client.version().await?;
    let version_str = str::from_utf8(&version)?;

    if print_help {
        println!("Kade server {version_str}\n");
        println!("Available commands:");
        println!("  help                         Show this help message");
        println!("  exit                         Exit the CLI (or use Ctrl+C)");
        println!("  ping [message]               Ping the server with an optional message");
        println!("  get <key>                    Get the value of a key");
        println!("  set <key> <value> [expires]  Set a key-value pair with optional expiration in milliseconds");
        println!("  publish <channel> <message>  Publish a message to a channel");
        println!("  subscribe <channel...>       Subscribe to one or more channels");
        println!("  dump [output]                Dump database state to a file (default: state.kade)");
        println!("  load [input]                 Load database state from a file (default: state.kade)");
        println!("\nExamples:");
        println!("  set mykey myvalue");
        println!("  set mykey myvalue 5000      (expires in 5 seconds)");
        println!("  get mykey");
        println!("  publish mychannel \"Hello World\"");
        println!("  subscribe channel1 channel2");
    } else {
        println!("{version_str}");
    }

    Ok(())
}

async fn execute_command(client: &mut Option<Client>, cmd: String) -> Result<()> {
    let mut lexer = shlex::Shlex::new(&cmd);
    let args: Vec<String> = lexer.by_ref().collect();

    if args.is_empty() {
        return Err("Empty command".into());
    }

    let conn = client.as_mut().ok_or("Not connected to the server")?;

    match args[0].to_lowercase().as_str() {
        "ping" => {
            let msg = args[1..].join(" ");
            let msg = if msg.is_empty() { None } else { Some(Bytes::from(msg)) };
            let value = conn.ping(msg).await?;

            println!("{}", str::from_utf8(&value)?)
        }

        "get" => {
            let key = args.get(1).ok_or("GET command requires a key")?;
            let value = conn.get(key).await?;

            match value {
                Some(v) => print_value(&v),
                None => println!("(nil)"),
            }
        }

        "set" => {
            let key = args.get(1).ok_or("SET command requires a key")?;
            let value = args.get(2).ok_or("SET command requires a value")?;

            if let Some(ms) = args.get(3) {
                let expires = Duration::from_millis(ms.parse()?);
                conn.set_expires(key, Bytes::from(value.clone()), expires).await?;
            } else {
                conn.set(key, Bytes::from(value.clone())).await?;
            }

            println!("OK");
        }

        "publish" => {
            let channel = args.get(1).ok_or("Publish command requires a channel")?;
            let message = args[2..].join(" ");

            if message.is_empty() {
                return Err("Publish command requires a message".into());
            }

            conn.publish(channel, Bytes::from(message)).await?;
            println!("Publish OK");
        }

        "dump" => {
            let output = args.get(1).map(String::as_str).unwrap_or("state.kade");
            conn.dump(Path::new(output)).await?;
            println!("Database state dumped to {:?}", output);
        }

        "load" => {
            let input = args.get(1).map(String::as_str).unwrap_or("state.kade");
            conn.load(Path::new(input)).await?;
            println!("Database state loaded from {:?}", input);
        }

        "subscribe" => {
            let channels = args.get(1..).ok_or("Subscribe command requires at least one channel")?.to_vec();
            let conn = client.take().ok_or("Not connected to the server")?;

            println!("Reading messages... (Ctrl+C to quit)");

            let mut messages = 0;
            let mut subscriber = conn.subscribe(channels).await?;

            while let Some(msg) = subscriber.next_message().await? {
                print!("\r{messages}) ");
                print_value(&msg.content);
                messages += 1;
            }
        }

        "help" | "?" => print_info(conn, true).await?,
        "version" | "v" => print_info(conn, false).await?,
        "exit" | "quit" => process::exit(0),

        _ => return Err(format!("Unknown command '{}'", args[0]).into()),
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let addr = format!("{}:{}", cli.host, cli.port);
    let mut client = Some(Client::connect(&addr).await?);

    if !cli.command.is_empty() {
        let cmd = cli.command.join(" ");
        return execute_command(&mut client, cmd).await;
    }

    let mut rl = DefaultEditor::new()?;

    loop {
        let repl = rl.readline(&format!("{addr}> "));

        match repl {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                if let Err(error) = execute_command(&mut client, line).await {
                    println!("(error) {error}");

                    if client.is_none() {
                        println!("Reconnecting to the server...");
                        client = Some(Client::connect(&addr).await?);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => return Ok(()),
            Err(ReadlineError::Eof) => return Ok(()),
            Err(err) => panic!("unknown error '{err}'"),
        }
    }
}

fn print_value(value: &[u8]) {
    if let Ok(string) = str::from_utf8(value) {
        println!("\"{string}\"");
    } else {
        println!("{value:?}");
    }
}
