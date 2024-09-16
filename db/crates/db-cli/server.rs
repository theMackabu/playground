mod verbose;

use clap::Parser;
use db_server::DEFAULT_PORT;
use std::path::PathBuf;
use tokio::net::TcpListener;
use tokio::signal;
use verbose::{InfoLevel, Verbosity};

use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::prelude::*;

#[derive(Parser, Debug)]
#[command(name = "db-server", version)]
struct Cli {
    #[arg(long, default_value_t = DEFAULT_PORT)]
    port: u16,

    #[arg(id = "hostname", long, default_value = "127.0.0.1")]
    host: String,

    #[clap(flatten)]
    verbose: Verbosity<InfoLevel>,

    #[arg(long, help = "Path to save/load database state")]
    state: Option<PathBuf>,
}

#[tokio::main]
pub async fn main() -> db_proto::Result<()> {
    let cli = Cli::parse();

    let formatting_layer_config = BunyanFormattingLayer::new("db-server".into(), std::io::stdout)
        .skip_fields(vec!["file", "line"].into_iter())
        .expect("Unable to create logger");

    tracing_subscriber::registry()
        .with(cli.verbose.log_level_filter())
        .with(JsonStorageLayer)
        .with(formatting_layer_config)
        .init();

    let addr = format!("{}:{}", cli.host, cli.port);
    let listener = TcpListener::bind(&addr).await?;

    Ok(db_server::run(listener, signal::ctrl_c(), cli.state).await)
}
