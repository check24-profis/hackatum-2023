use crate::buisiness_logic::get_top_20_craftsmen;
use crate::controller::{echo, getCraftsmen, hello};
use crate::model::service_provider_profile::ServiceProviderProfile;
use diesel::pg::PgConnection;
use serde::{Deserialize, Serialize}; // Import controller functions
                                     //use crate::schema::quality_factor_score::dsl::quality_factor_score;
use crate::model::NewQualityFactorScore::NewQualityFactorScore;
use crate::schema::quality_factor_score::dsl::*;
use crate::schema::service_provider_profile::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;

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

pub async fn updateCraftsman(pool: web::Data<DbPool>, craftman_id: web::Path<i32>, req_body: web::Json<PatchRequest>) -> actix_web::Result<impl Responder> {

    let mut conn = pool.get().expect("couldn't get db connection from pool");

    
    let PatchRequest {
        maxDrivingDistance,
        profilePictureScore,
        profileDescriptionScore,
    } = req_body.0;

    let craftman_id = craftman_id.into_inner();

    
    let mut score: NewQualityFactorScore;
    let craftsman: ServiceProviderProfile; 

    if maxDrivingDistance.is_some() {
        craftsman = diesel::update(service_provider_profile.filter(id.eq(&craftman_id)))
        .set(max_driving_distance.eq(maxDrivingDistance.unwrap()))   
        .get_result(&mut conn)
        .expect("Error updating craftsman");
    } else {
        craftsman = service_provider_profile.filter(id.eq(&craftman_id)).first(&mut conn).expect("Error loading craftsman");
    }
    if profileDescriptionScore.is_none() && profilePictureScore.is_none() {
        score = quality_factor_score.filter(profile_id.eq(&craftman_id)).first(&mut conn).expect("Error loading score")
    }
    else if profilePictureScore.is_some() {
        score = diesel::update(quality_factor_score.filter(profile_id.eq(&craftman_id)))
        .set(profile_picture_score.eq(profilePictureScore.unwrap()))
        .get_result(&mut conn)
        .expect("Error updating score");
    }
    else { // profileDescriptionScore.is_some()
        score = diesel::update(quality_factor_score.filter(profile_id.eq(&craftman_id)))
        .set(profile_description_score.eq(profileDescriptionScore.unwrap()))   
        .get_result(&mut conn)
        .expect("Error updating score");
    }
     
    
    let updated = Updated {
        maxDrivingDistance: maxDrivingDistance.unwrap(),
        profilePictureScore: profilePictureScore.unwrap(),
        profileDescriptionScore: profileDescriptionScore.unwrap()
    };
    let response = PatchResponse {
        id: craftman_id,
        updated: updated
    };
    Ok(web::Json(response))

}