use actix_web::web;

use crate::handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service({
            web::resource("/events/")
                .name("add_event")
                .route(web::post().to(handlers::add_event::add_event))
        })
        .service({
            web::resource("/users/{user_id}/events/")
                .name("events_list")
                .route(web::get().to(handlers::events_list::events_list))
        });
}
