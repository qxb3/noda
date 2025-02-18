mod dbus;

use dbus::NotifDbusServer;
use std::error::Error;

/// Type alias for Result.
pub type NodaResult<T> = Result<T, Box<dyn Error>>;

#[tokio::main]
async fn main() -> NodaResult<()> {
    NotifDbusServer::start().await?;

    Ok(())
}
