use actix::Addr;
use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use events_backend::messages::AddToStream;
use events_backend::RedisBackend;

use crate::error::ServiceError;

/// Wrapper for numeric representation of event type.
#[derive(Deserialize)]
pub struct EventType(u8);

#[derive(Deserialize)]
pub struct Payload {
    pub user_id: u64,
    pub item_id: u64,
    pub geo_id: u64,
    pub event_type: EventType,
    pub category_id: u16,
    pub timestamp: Option<i64>,
}

/// Convert u8 to EventType enum.
/// We only use range 0 to 4.
/// Everything else is not supported.
impl From<EventType> for events_serializers::EventType {
    fn from(event_type: EventType) -> Self {
        match event_type.0 {
            100 => events_serializers::EventType::SendMessage,
            101 => events_serializers::EventType::AddToFavourite,
            102 => events_serializers::EventType::ViewGallery,
            103 => events_serializers::EventType::ViewItem,
            104 => events_serializers::EventType::ViewPhone,
            _ => events_serializers::EventType::Unknown(event_type.0),
        }
    }
}

/// Convert Payload structure to Event serializer.
impl From<Payload> for events_serializers::Event {
    fn from(payload: Payload) -> Self {
        Self {
            user_id: payload.user_id,
            item_id: payload.item_id,
            geo_id: payload.geo_id,
            event_type: events_serializers::EventType::from(payload.event_type),
            category_id: payload.category_id,
            create_time: payload.timestamp
                .map(|value| {
                    let naive_date = NaiveDateTime::from_timestamp(value, 0);
                    DateTime::<Utc>::from_utc(naive_date, Utc)
                })
                .unwrap_or_else(Utc::now),
        }
    }
}

pub async fn add_event(
    payload: web::Json<Payload>,
    backend: Data<Addr<RedisBackend>>,
) -> Result<HttpResponse, ServiceError> {
    let stream_name = payload.user_id.to_string();
    let event = events_serializers::Event::from(payload.into_inner());
    let message = AddToStream::new(stream_name, event.to_string());
    backend.send(message).await??;
    Ok(HttpResponse::Created().finish())
}
