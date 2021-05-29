use actix_web::{middleware, web, App, HttpServer, HttpResponse, error};
use events::routes::v1;
use events_backend::RedisBackend;
use tracing::Level;
use actix::prelude::*;
use events::response_error::ErrorPayload;
use hitbox_actix::prelude::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let redis = RedisBackend::builder()
        .build()
        .await
        .unwrap()
        .start();
    let cache = Cache::new()
        .await
        .unwrap()
        .start();

    HttpServer::new(move || {
         let json_config = web::JsonConfig::default()
            .limit(64 * 1024)
            .error_handler(|err, _req| {
                let err_payload = ErrorPayload {
                    error: String::from("Error while serializing payload"),
                    message: Some(err.to_string()),
                };
                let response = HttpResponse::BadRequest().json(err_payload);
                error::InternalError::from_response(err, response).into()
            });

        App::new()
            .wrap(middleware::Logger::default())
            .data(redis.clone())
            .data(json_config)
            .data(cache.clone())
            .service(web::scope("/v1").configure(v1::routes))
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
}
