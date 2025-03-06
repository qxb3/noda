use crate::{server::{Notification, NotificationServer}, NodaResult};

/// Noda Events.
pub enum NodaEvent {
    Notification(Notification)
}

/// Starts noda.
pub async fn start_noda() -> NodaResult<()> {
    log::info!("noda starting...");

    // Channel to sync up the ui.
    let (tx, mut rx) = tokio::sync::mpsc::channel::<NodaEvent>(100);

    // Starts up notification server.
    NotificationServer::start(tx.clone()).await?;

    Ok(())
}
