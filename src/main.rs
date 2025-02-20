mod notification_server;

use daemonize::Daemonize;
use notification_server::NotificationServer;
use std::{env, error::Error, fs::File, process::Command};

/// Type alias for Result.
pub type NodaResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> NodaResult<()> {
    // Check if this current process has already been daemonized.
    if env::var("NODA_DAEMONIZE").is_err() {
        // Temporary files for stdout & stderr logging.
        let stdout = File::create("/tmp/noda.log").unwrap();
        let stderr = File::create("/tmp/noda.err").unwrap();

        // Create and start a daemon.
        Daemonize::new()
            .pid_file("/tmp/noda.pid")
            .chown_pid_file(true)
            .stdout(stdout)
            .stderr(stderr)
            .start()?;

        // Run noda again but in daemon context.
        Command::new(env::current_exe()?)
            .env("NODA_DAEMONIZE", "true")
            .spawn()?;

        return Ok(());
    }

    NotificationServer::start().await?;

    Ok(())
}
