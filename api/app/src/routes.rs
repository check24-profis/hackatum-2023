use actix_web::web;
use crate::controller::{hello, echo, getCraftsmen}; // Import controller functions

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(hello))
    )
    .service(
        web::resource("/echo")
            .route(web::post().to(echo))
    )
    .service(
        web::resource("/get")
        .route(web::get().to(getCraftsmen))
    )
    ;
}
