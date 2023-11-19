use actix_cors::Cors;
use diesel::pg::Pg;
use routing::routes;
use routing::routes::configure_routes;
use std::env;
use std::error::Error;

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

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(
    connection: &mut impl MigrationHarness<Pg>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    // This will run the necessary migrations.
    //
    // See the documentation for `MigrationHarness` for
    // all available methods.
    connection.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // Load .env file

    // First connection to run migrations
    let mut conn = establish_connection();
    run_migrations(&mut conn).expect("Error running migrations at startup");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        let cors = Cors::permissive().allow_any_origin();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .configure(configure_routes)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
