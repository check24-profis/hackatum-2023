use actix_web::web;
use crate::controller::{hello, echo, getCraftsmen, updateCraftman}; // Import controller functions

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
        web::resource("/craftsmen")
        .route(web::get().to(getCraftsmen))
    )
    .service(
        web::resource("/craftman/{craftman_id}")
        .route(web::patch().to(updateCraftman))
    )
    ;
}
