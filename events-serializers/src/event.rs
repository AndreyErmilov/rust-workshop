use serde::{Deserialize, Serialize};
use chrono::{Utc, DateTime};
use std::fmt;

/// Supported event types.
#[derive(Deserialize, Serialize, Debug)]
pub enum EventType {
    ViewItem,
    AddToFavourite,
    ViewPhone,
    SendMessage,
    ViewGallery,
    Unknown(u8),
}

/// Serializer for Event type.
#[derive(Deserialize, Serialize, Debug)]
pub struct Event {
    pub user_id: u64,
    pub item_id: u64,
    pub geo_id: u64,
    pub event_type: EventType,
    pub category_id: u16,
    pub create_time: DateTime<Utc>,
}

impl Event {
    /// Check if event type is supported.
    pub fn correct_type(&self) -> bool {
        match &self.event_type {
            EventType::Unknown(_) => false,
            _ => true,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(self).unwrap_or_default())
    }
}
