use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn getCraftsmen(req_body: String) -> impl Responder {

    // TODO
    // fetch from database
    let result = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("!")
    ];

    // Serialize the result to JSON
    match serde_json::to_string(&result) {
        Ok(json_result) => HttpResponse::Ok().content_type("application/json").body(json_result),
        Err(_) => HttpResponse::InternalServerError().finish(), // Handle serialization error
    }
}