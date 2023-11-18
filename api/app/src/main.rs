extern crate diesel;

use crate::routes::configure_routes;
use actix_web::{web, App, HttpServer};

mod controller;
mod model;
mod routes;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configure_routes))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
