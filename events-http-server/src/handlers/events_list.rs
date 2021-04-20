use actix::Addr;
use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::error::ServiceError;
use events_backend::RedisBackend;
use actix_web::web::Data;
use events_backend::messages::GetStream;
use events_serializers::Event;

#[derive(Deserialize, Debug)]
pub struct Path {
    pub user_id: String,
}

pub async fn events_list(
    path: web::Path<Path>,
    backend: Data<Addr<RedisBackend>>,
) -> Result<HttpResponse, ServiceError> {
    let message = GetStream::new(path.into_inner().user_id);
    let result: Vec<Event> = backend.send(message)
        .await??
        .into_iter()
        .filter_map(|value| serde_json::from_str(&value).ok())
        .filter(Event::correct_type)
        .collect();
    Ok(HttpResponse::Ok().json(result))
}
