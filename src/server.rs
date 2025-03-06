use std::{collections::HashMap, future::pending};

use crate::{noda::NodaEvent, NodaResult};

/// Notification Urgency.
#[derive(Debug, PartialEq)]
pub enum Urgency {
    Low,
    Normal,
    Critical,
}

/// Notification Hint.
#[derive(Debug, PartialEq)]
pub enum Hint {
    ActionIcons(bool),
    Category(String),
    DesktopEntry(String),
    ImageData {
        width: i32,
        height: i32,
        row_side: i32,
        has_alpha: bool,
        bits_per_sample: i32,
        channels: i32,
        data: Vec<u8>,
    },
    ImagePath(String),
    Resident(bool),
    SoundFile(String),
    SoundName(String),
    SuppressSound(bool),
    Transient(bool),
    X(i32),
    Y(i32),
    Urgency(Urgency),
}

/// A struct representing a Notification.
#[derive(Debug)]
pub struct Notification {
    pub app_name: String,
    pub replace_id: u32,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub actions: Vec<String>,
    pub hints: Vec<Hint>,
    pub expire_timeout: i32,
}

/// Notification D-Bus Server.
pub struct NotificationServer {
    notifications: Vec<Notification>,
    channel_sender: tokio::sync::mpsc::Sender<NodaEvent>
}

#[zbus::interface(name = "org.freedesktop.Notifications")]
impl NotificationServer {
    /// When there is a new notification.
    fn notify(
        &mut self,
        app_name: String,
        replace_id: u32,
        app_icon: String,
        summary: String,
        body: String,
        actions: Vec<String>,
        hints: HashMap<String, zvariant::Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        let mut parsed_hints = vec![];

        for (name, value) in hints.iter() {
            match name.as_str() {
                "action-icons" => {
                    let action_icons: bool = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast action-icons: {err}")))?;

                    parsed_hints.push(Hint::ActionIcons(action_icons));
                }
                "category" => {
                    let category: String = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast category: {err}")))?;

                    parsed_hints.push(Hint::Category(category));
                }
                "desktop-entry" => {
                    let desktop_entry: String = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast desktop-entry: {err}")))?;

                    parsed_hints.push(Hint::DesktopEntry(desktop_entry));
                }
                "image-data" => {
                    if let zvariant::Value::Structure(structure) = value {
                        let fields = structure.fields();

                        let width = fields
                            .get(0)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing width parameter".into()))?
                            .downcast_ref::<i32>()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected an i32 for width parameter".into()))?;

                        let height = fields
                            .get(1)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing height parameter".into()))?
                            .downcast_ref::<i32>()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected an i32 for height parameter".into()))?;

                        let row_side = fields
                            .get(2)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing row_side parameter".into()))?
                            .downcast_ref::<i32>()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected an i32 for row_side parameter".into()))?;

                        let has_alpha = fields
                            .get(3)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing has_alpha parameter".into()))?
                            .downcast_ref::<bool>()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected a boolean for has_alpha parameter".into()))?;

                        let bits_per_sample = fields
                            .get(4)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing bits_per_sample parameter".into()))?
                            .downcast_ref::<i32>()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected an i32 for bits_per_sample parameter".into()))?;

                        let channels = fields
                            .get(5)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing channels parameter".into()))?
                            .downcast_ref::<i32>()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected an i32 for channels parameter".into()))?;

                        let data: Vec<u8> = fields
                            .get(6)
                            .ok_or_else(|| zbus::fdo::Error::Failed("Missing image data parameter".into()))?
                            .to_owned()
                            .try_into()
                            .map_err(|_| zbus::fdo::Error::Failed("Expected an Array of bytes for image data".into()))?;

                        parsed_hints.push(Hint::ImageData {
                            width,
                            height,
                            row_side,
                            has_alpha,
                            bits_per_sample,
                            channels,
                            data,
                        });
                    }
                }
                "image-path" => {
                    let image_path = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast image-path: {err}")))?;

                    parsed_hints.push(Hint::ImagePath(image_path));
                },
                "resident" => {
                    let resident = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast resident: {err}")))?;

                    parsed_hints.push(Hint::Resident(resident));
                },
                "sound-file" => {
                    let sound_file = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast sound-file: {err}")))?;

                    parsed_hints.push(Hint::SoundFile(sound_file));
                },
                "sound-name" => {
                    let sound_name = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast sound-name: {err}")))?;

                    parsed_hints.push(Hint::SoundName(sound_name));
                }
                "suppress-sound" => {
                    let suppress_sound = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast suppress-sound: {err}")))?;

                    parsed_hints.push(Hint::SuppressSound(suppress_sound));
                }
                "transient" => {
                    let transient = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast transient: {err}")))?;

                    parsed_hints.push(Hint::Transient(transient));
                }
                "x" => {
                    let x = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast x: {err}")))?;

                    parsed_hints.push(Hint::X(x));
                }
                "y" => {
                    let y = value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(format!("Failed to downcast y: {err}")))?;

                    parsed_hints.push(Hint::Y(y));
                }
                "urgency" => {
                    if let zvariant::Value::U8(b) = value {
                        let urgency = match b {
                            0 => Urgency::Low,
                            1 => Urgency::Normal,
                            2 => Urgency::Critical,
                            _ => unreachable!(),
                        };

                        parsed_hints.push(Hint::Urgency(urgency));
                    }
                }
                _ => {}
            }
        }

        let notification = Notification {
            app_name,
            replace_id,
            app_icon,
            summary,
            body,
            actions,
            hints: parsed_hints,
            expire_timeout,
        };

        log::info!(
            "New Notification: {} {}",
            &notification.app_name,
            &notification.summary
        );

        self.notifications.push(notification);

        Ok(3)
    }

    /// Server capabilities.
    fn get_capabilities(&self) -> Vec<&str> {
        vec!["action-icons", "actions", "body", "body-hyperlinks", "persistent", "sound"]
    }

    /// Server info.
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("noda", "qxbthree", "0.0.1", "1.2")
    }
}

impl NotificationServer {
    /// Start D-Bus notification server.
    pub async fn start(channel_sender: tokio::sync::mpsc::Sender<NodaEvent>) -> NodaResult<()> {
        tokio::spawn(async move {
            // Creates new dbus server.
            let dbus_server = NotificationServer {
                notifications: Vec::new(),
                channel_sender
            };

            // Register D-Bus session.
            let _conn = zbus::conn::Builder::session()?
                .name("org.freedesktop.Notifications")?
                .serve_at("/org/freedesktop/Notifications", dbus_server)?
                .build()
                .await?;

            log::info!("Notification server started.");

            pending::<()>().await;

            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        })
            .await?
            .map_err(|err| format!("Failed to start notification server: {err}"))?;

        Ok(())
    }
}
