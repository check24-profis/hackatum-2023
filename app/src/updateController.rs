use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::r2d2::{Pool, ConnectionManager};
use crate::schema::quality_factor_score::dsl::*;
use crate::schema::service_provider_profile::dsl::*;
use serde::{Deserialize, Serialize}; 
use actix_web::{web, Responder};


type DbPool = Pool<ConnectionManager<PgConnection>>;


#[derive(Debug, Deserialize)]
pub struct PatchRequest {
  // At least one of the attributes should be defined
  maxDrivingDistance: Option<i32>,
  profilePictureScore: Option<f64>,
  profileDescriptionScore: Option<f64>
}

#[derive(Debug, Serialize)]
pub struct Updated {
  // At least one of the attributes should be defined
  maxDrivingDistance: i32,
  profilePictureScore: f64,
  profileDescriptionScore: f64
}

#[derive(Debug, Serialize)]
struct PatchResponse {
    id: i32,
    updated: Updated
}

pub async fn update_craftsman(pool: web::Data<DbPool>, craftman_id: web::Path<i32>, req_body: web::Json<PatchRequest>) -> actix_web::Result<impl Responder> {

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let PatchRequest {
        maxDrivingDistance,
        profilePictureScore,
        profileDescriptionScore,
    } = req_body.0;

    let craftman_id = craftman_id.into_inner();

    if profileDescriptionScore.is_some() {
        diesel::update(quality_factor_score.filter(profile_id.eq(&craftman_id)))
        .set(profile_description_score.eq(profileDescriptionScore.unwrap()))   
        .execute(&mut conn)
        .expect("Error updating score");

    } 
    if profilePictureScore.is_some() {
        diesel::update(quality_factor_score.filter(profile_id.eq(&craftman_id)))
        .set(profile_picture_score.eq(profilePictureScore.unwrap()))
        .execute(&mut conn)
        .expect("Error updating score");
    }
    if profileDescriptionScore.is_some() || profilePictureScore.is_some() {
        let update_query = format!("UPDATE quality_factor_score SET profile_score = 0.4 * profile_picture_score + 0.6 * profile_description_score WHERE profile_id = {}", &craftman_id);
        diesel::sql_query(update_query).execute(&mut conn).expect("Error updating score");
    }
    if maxDrivingDistance.is_some() { 
        diesel::update(service_provider_profile.filter(id.eq(&craftman_id)))
        .set(max_driving_distance.eq(maxDrivingDistance.unwrap()))   
        .execute(&mut conn)
        .expect("Error updating craftsman");
    }
     
    // Will return -1 if no update value has been given
    let updated = Updated {
        maxDrivingDistance: maxDrivingDistance.unwrap_or(-1),
        profilePictureScore: profilePictureScore.unwrap_or(-1.0),
        profileDescriptionScore: profileDescriptionScore.unwrap_or(-1.0)
    };
    let response = PatchResponse {
        id: craftman_id,
        updated: updated
    };
    Ok(web::Json(response))

}