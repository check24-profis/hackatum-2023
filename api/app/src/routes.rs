use diesel::pg::PgConnection;
use crate::controller::{hello, echo, getCraftsmen}; // Import controller functions
//use crate::schema::quality_factor_score::dsl::quality_factor_score;
use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use crate::schema::quality_factor_score::dsl::*;
use crate::schema::service_provider_profile::dsl::*;
use crate::model::NewQualityFactorScore::NewQualityFactorScore;
use diesel::prelude::*;

type DbPool = Pool<ConnectionManager<PgConnection>>;




pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(hello))
    )
    .service(
        web::resource("/echo")
            .route(web::post().to(echo))
    )
    .service(
        web::resource("/craftsmen")
        .route(web::get().to(getCraftsmen))
    )
    .service(
        web::resource("/craftman/{craftman_id}")
        .route(web::patch().to(index))
    )
    ;
}



#[derive(Debug, Deserialize)]
pub struct PatchRequest {
  // At least one of the attributes should be defined
  maxDrivingDistance: i32,
  profilePictureScore: f64,
  profileDescriptionScore: f64
}

async fn index(
    pool: web:: Data<DbPool>,
    craftman_id: web::Path<i32>,
    req_body: web::Json<PatchRequest>
) -> actix_web::Result<impl Responder> {

    let updatedScore = web::block(move || {
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");
        
        updateCraftman(&mut conn, craftman_id, req_body)
        
    }).await?;
    //.map_err(error::ErrorInternalServerError)?;

        
    Ok(HttpResponse::Ok().json(updatedScore))

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

pub fn updateCraftman(conn: &mut PgConnection, craftman_id: web::Path<i32>, req_body: web::Json<PatchRequest>) -> PatchResponse {
    // TODO
    // implement


        // if maxDrivingDistance.is_none() && profilePictureScore.is_none() && profileDescriptionScore.is_none() {
    //    HttpResponse::InternalServerError().body("At least one updated value must be specified")
    let PatchRequest {
        maxDrivingDistance,
        profilePictureScore,
        profileDescriptionScore,
    } = req_body.0;

    let craftman_id = craftman_id.into_inner();

    let updatedScore = diesel::update(quality_factor_score.filter(profile_id.eq(&craftman_id)))
    .set((profile_picture_score.eq(profilePictureScore), profile_description_score.eq(profileDescriptionScore), profile_score.eq(0.4 * profile_picture_score + 0.6 * profile_description_score)))
    .get_result(conn)
    .expect("Error updating score");



    let updatedCraftsman = diesel::update(service_provider_profile.filter(id.eq(&craftman_id)))
    .set(max_driving_distance.eq(maxDrivingDistance))
    .get_result(conn)
    .expect("Error updating craftsman");

    let updated = Updated {
        maxDrivingDistance: updatedCraftsman.max_driving_distance,
        profilePictureScore: updatedScore.profile_picture_score,
        profileDescriptionScore: updatedScore.profile_description_score
    };
    let patchResponse = PatchResponse {
        id: craftman_id,
        updated: updated
    };



    updatedScore
}
