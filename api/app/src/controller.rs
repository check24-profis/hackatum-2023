use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
// use crate::model::quality_factor_score::*; 
use crate::schema::quality_factor_score;
//use crate::schema::quality_factor_score::dsl::quality_factor_score;



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
        String::from("!")
    ];

    // Serialize the result to JSON
    match serde_json::to_string(&result) {
        Ok(json_result) => HttpResponse::Ok().content_type("application/json").body(json_result),
        Err(_) => HttpResponse::InternalServerError().finish(), // Handle serialization error
    }
}

#[derive(Debug, Deserialize)]
pub struct PatchRequest {
  // At least one of the attributes should be defined
  maxDrivingDistance: i32,
  profilePictureScore: f64,
  profileDescriptionScore: f64
}




    
