mod cli;
mod daemon;
mod server;
mod noda;

use clap::Parser;
use cli::{NodaCli, NodaCommands};
use std::{error::Error, io::Write};

use daemon::start_daemon;
use noda::start_noda;

/// Type alias for Result.
pub type NodaResult<T> = Result<T, Box<dyn Error + Send + Sync>>;

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
        NodaCommands::Daemon => start_daemon()?,
        NodaCommands::Start => start_noda().await?,
    }

    Ok(())
}
