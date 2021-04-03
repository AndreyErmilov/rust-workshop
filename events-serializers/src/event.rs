use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use std::fmt;

#[derive(Deserialize, Serialize, Debug)]
pub enum EventType {
    ViewItem,
    AddToFavourite,
    ViewPhone,
    SendMessage,
    ViewGallery,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub user_id: u64,
    pub device_id: String,
    pub item_id: u64,
    pub geo_id: u64,
    pub event_type: EventType,
    pub category_id: u16,
    pub create_time: DateTime<Utc>,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}
