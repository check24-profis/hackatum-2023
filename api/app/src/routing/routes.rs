use actix_web::{web, HttpResponse, Responder};

use super::craftsmen::get_top20_craftmen;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(hello)))
        .service(web::resource("/echo").route(web::post().to(echo)))
        .service(
            web::resource("/craftmen/{craftman_id}").route(web::patch().to(crate::routes::index)),
        )
        .service(web::resource("/craftsmen").route(web::get().to(get_top20_craftmen)));
}

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
