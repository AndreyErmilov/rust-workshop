use actix::Addr;
use actix_web::{HttpResponse, web};
use actix_web::web::Data;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

use events_backend::messages::Add;
use events_backend::RedisBackend;

use crate::error::ServiceError;

#[derive(Deserialize)]
pub enum EventType {
    ViewItem,
    AddToFavourite,
    ViewPhone,
    SendMessage,
    ViewGallery,
}

#[derive(Deserialize)]
pub struct Payload {
    pub user_id: u64,
    pub device_id: String,
    pub item_id: u64,
    pub geo_id: u64,
    pub event_type: EventType,
    pub category_id: u16,
    pub timestamp: Option<i64>,
}

impl From<EventType> for events_serializers::EventType {
    fn from(event_type: EventType) -> Self {
        match event_type {
            EventType::SendMessage => events_serializers::EventType::SendMessage,
            EventType::AddToFavourite => events_serializers::EventType::AddToFavourite,
            EventType::ViewGallery => events_serializers::EventType::ViewGallery,
            EventType::ViewItem => events_serializers::EventType::ViewItem,
            EventType::ViewPhone => events_serializers::EventType::ViewPhone,
        }
    }
}

impl From<Payload> for events_serializers::Event {
    fn from(payload: Payload) -> Self {
        Self {
            user_id: payload.user_id,
            device_id: payload.device_id,
            item_id: payload.item_id,
            geo_id: payload.geo_id,
            event_type: events_serializers::EventType::from(payload.event_type),
            category_id: payload.category_id,
            create_time: payload.timestamp
                .map(|value| {
                    DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(value, 0), Utc)
                })
                .unwrap_or_else(Utc::now),
        }
    }
}

pub async fn add_event(
    payload: web::Json<Payload>,
    backend: Data<Addr<RedisBackend>>,
) -> Result<HttpResponse, ServiceError> {
    let stream = payload.user_id.to_string();
    let event = events_serializers::Event::from(payload.into_inner());
    let message = Add::new(stream, String::from("key"), event.to_string());
    backend.send(message).await??;
    Ok(HttpResponse::Created().finish())
}
