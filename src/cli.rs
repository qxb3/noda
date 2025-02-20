use clap::{Parser, Subcommand};

/// Noda Cli.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct NodaCli {
    #[command(subcommand)]
    pub command: NodaCommands,
}

/// Cli Commands.
#[derive(Subcommand)]
pub enum NodaCommands {
    /// Start noda without daemon.
    Start,

    /// Start noda in daemon.
    Daemon,
}
