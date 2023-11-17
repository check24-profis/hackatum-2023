use actix_web::{web, App, HttpServer};
use crate::routes::configure_routes;

mod controller;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
