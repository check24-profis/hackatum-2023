use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

// TODO
//change json req structure
pub async fn getCraftsmen(req_body: String) -> impl Responder {
    // TODO
    // fetch from database
    let result = vec![
        String::from("Hello"),
        String::from("World"),
        String::from("!"),
    ];

    // Serialize the result to JSON
    match serde_json::to_string(&result) {
        Ok(json_result) => HttpResponse::Ok()
            .content_type("application/json")
            .body(json_result),
        Err(_) => HttpResponse::InternalServerError().finish(), // Handle serialization error
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchRequest {
    // At least one of the attributes should be defined
    maxDrivingDistance: i32,
    profilePictureScore: f64,
    profileDescriptionScore: f64,
}

pub async fn updateCraftman(
    craftman_id: web::Path<String>,
    req_body: web::Json<PatchRequest>,
) -> impl Responder {
    // TODO
    // implement

    // For testing only
    // TODO: delete
    println!("called");
    let response = format!("You passed the id: {}", craftman_id);

    HttpResponse::Ok().body(response)
}
