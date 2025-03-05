mod cli;
mod daemon;
mod server;

use clap::Parser;
use cli::{NodaCli, NodaCommands};
use daemon::start_daemon;
use server::NotificationServer;
use std::{error::Error, io::Write};

/// Type alias for Result.
pub type NodaResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> NodaResult<()> {
    // Init logger.
    env_logger::builder()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                buf.timestamp(),
                record.level(),
                record.args()
            )
        })
        .filter_level(log::LevelFilter::Trace)
        .target(env_logger::Target::Stdout)
        .init();

    // Parse cli.
    let cli = NodaCli::parse();

    match cli.command {
        // Start the start command.
        NodaCommands::Start => {
            log::info!("noda starting...");

            NotificationServer::start().await?;
        }

        // Start the daemon command.
        NodaCommands::Daemon => start_daemon()?,
    }

    Ok(())
}
