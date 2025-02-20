use std::collections::HashMap;

#[derive(Debug)]
pub struct Notification<'a> {
    app_name: String,
    replace_id: u32,
    app_icon: String,
    summary: String,
    body: String,
    actions: Vec<String>,
    hints: HashMap<String, zbus::zvariant::Value<'a>>,
    expire_timeout: i32
}
