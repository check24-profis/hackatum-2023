use std::env;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;

use crate::routes::configure_routes;
use actix_web::{web, App, HttpServer};

mod buisiness_logic;
mod controller;
mod model;
mod routes;
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
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
