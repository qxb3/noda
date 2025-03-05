use std::{collections::HashMap, future::pending};

use crate::NodaResult;

/// Notification Urgency.
#[derive(Debug, PartialEq)]
enum Urgency {
    Low,
    Normal,
    Critical,
}

/// Notification Hint.
#[derive(Debug, PartialEq)]
enum Hint {
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
struct Notification {
    app_name: String,
    replace_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: Vec<Hint>,
    expire_timeout: i32,
}

/// Notification D-Bus Server.
pub struct NotificationServer {
    notifications: Vec<Notification>,
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
                    parsed_hints.push(Hint::ActionIcons(value.downcast_ref().map_err(
                        |err| zbus::fdo::Error::UnknownProperty(err.to_string()),
                    )?))
                }
                "category" => {
                    parsed_hints.push(Hint::Category(value.downcast_ref().map_err(
                        |err| zbus::fdo::Error::UnknownProperty(err.to_string()),
                    )?))
                }
                "desktop-entry" => {
                    parsed_hints.push(Hint::DesktopEntry(value.downcast_ref().map_err(
                        |err| zbus::fdo::Error::UnknownProperty(err.to_string()),
                    )?))
                }
                "image-data" => {
                    if let zvariant::Value::Structure(structure) = value {
                        let fields = structure.fields();

                        let width = match fields.get(0) {
                            Some(width) => match width {
                                zvariant::Value::I32(width) => *width,
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected an i32 for width".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing width parameter".into(),
                                ))
                            }
                        };

                        let height = match fields.get(1) {
                            Some(height) => match height {
                                zvariant::Value::I32(height) => *height,
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected an i32 for height".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing height parameter".into(),
                                ))
                            }
                        };

                        let row_side = match fields.get(2) {
                            Some(row_side) => match row_side {
                                zvariant::Value::I32(row_side) => *row_side,
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected an i32 for row_side".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing row_side parameter".into(),
                                ))
                            }
                        };

                        let has_alpha = match fields.get(3) {
                            Some(has_alpha) => match has_alpha {
                                zvariant::Value::Bool(has_alpha) => *has_alpha,
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected a boolean for has_alpha".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing has_alpha parameter".into(),
                                ))
                            }
                        };

                        let bits_per_sample = match fields.get(4) {
                            Some(bits_per_sample) => match bits_per_sample {
                                zvariant::Value::I32(bits_per_sample) => {
                                    *bits_per_sample
                                }
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected a i32 for bits_per_sample".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing bits_per_sample parameter".into(),
                                ))
                            }
                        };

                        let channels = match fields.get(5) {
                            Some(channels) => match channels {
                                zvariant::Value::I32(channels) => *channels,
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected a i32 for channels".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing channels parameter".into(),
                                ))
                            }
                        };

                        let data = match fields.get(6) {
                            Some(data) => match data {
                                zvariant::Value::Array(_) => {
                                    Vec::<u8>::try_from(data.clone()).expect("")
                                }
                                _ => {
                                    return Err(zbus::fdo::Error::Failed(
                                        "Expected an Array of bytes for image data".into(),
                                    ))
                                }
                            },
                            None => {
                                return Err(zbus::fdo::Error::Failed(
                                    "Missing image data parameter".into(),
                                ))
                            }
                        };

                        println!("ran 3");

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
                "image-path" => parsed_hints.push(Hint::ImagePath(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "resident" => parsed_hints.push(Hint::Resident(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "sound-file" => parsed_hints.push(Hint::SoundFile(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "sound-name" => parsed_hints.push(Hint::SoundName(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "suppress-sound" => parsed_hints.push(Hint::SuppressSound(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "transient" => parsed_hints.push(Hint::Transient(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "x" => parsed_hints.push(Hint::X(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
                "y" => parsed_hints.push(Hint::Y(
                    value
                        .downcast_ref()
                        .map_err(|err| zbus::fdo::Error::Failed(err.to_string()))?,
                )),
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

    /// Server info.
    fn get_server_information(&self) -> (&str, &str, &str, &str) {
        ("noda", "qxbthree", "0.0.1", "1.2")
    }
}

impl NotificationServer {
    /// Start D-Bus notification server.
    pub async fn start() -> NodaResult<()> {
        // Creates new dbus server.
        let dbus_server = NotificationServer { notifications: Vec::new() };

        // Register D-Bus session.
        let _conn = zbus::conn::Builder::session()?
            .name("org.freedesktop.Notifications")?
            .serve_at("/org/freedesktop/Notifications", dbus_server)?
            .build()
            .await?;

        log::info!("Notification server started.");

        pending::<()>().await;

        Ok(())
    }
}
