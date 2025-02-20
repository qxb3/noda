use std::{collections::HashMap, future::pending};

use crate::NodaResult;

/// Notification D-Bus Server.
pub struct NotificationServer {
    notifications: Vec<()>
}

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl NotificationServer {
    /// When there is a new notification.
    fn notify(
        &mut self,
        app_name: String,
        _replace_id: u32,
        _app_icon: String,
        _summary: String,
        _body: String,
        _actions: Vec<String>,
        _hints: HashMap<String, zbus::zvariant::Value<'_>>,
        _expire_timeout: i32
    ) -> zbus::fdo::Result<u32> {
        println!("{app_name}");
        Ok(3)
    }

    /// Server info.
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        (
            "noda",
            "qxbthree",
            "0.0.1",
            "1.2"
        )
    }
}

impl NotificationServer {
    /// Start D-Bus notification server.
    pub async fn start() -> NodaResult<()> {
        // Creates new dbus server.
        let dbus_server = NotificationServer {
            notifications: Vec::new()
        };

        // Register D-Bus session.
        let _conn = zbus::conn::Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", dbus_server)?
            .build()
            .await?;

        pending::<()>().await;

        Ok(())
    }
}
