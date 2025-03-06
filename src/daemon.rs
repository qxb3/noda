use std::{env, fs::File, process::Command};

use daemonize::Daemonize;

use crate::NodaResult;

/// Starts daemon.
pub fn start_daemon() -> NodaResult<()> {
    // Check if this current process has already been daemonized.
    if env::var("___NODA_DAEMONIZE___").is_err() {
        // Temporary files for stdout & stderr logging.
        let stdout = File::create("/tmp/noda.log")?;

        log::info!("Daemon starting...");

        // Create and start a daemon.
        Daemonize::new()
            .pid_file("/tmp/noda.pid")
            .chown_pid_file(true)
            .stdout(stdout)
            .start()?;

        log::info!("Daemon started.");

        // Run noda start but in daemon context.
        Command::new(env::current_exe()?)
            .env("___NODA_DAEMONIZE___", "true")
            .args(["start"])
            .spawn()?;

        return Ok(());
    }

    Ok(())
}
