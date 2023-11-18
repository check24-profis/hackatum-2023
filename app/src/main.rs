use actix_cors::Cors;
use routing::routes;
use routing::routes::configure_routes;
use std::env;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;

use actix_web::{web, App, HttpServer};

mod buisiness_logic;
mod controller;
mod model;
mod routing;
mod schema;
mod updateController;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}