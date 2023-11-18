use crate::buisiness_logic::get_top_20_craftsmen;
use crate::controller::{echo, getCraftsmen, hello};
use crate::model::service_provider_profile::ServiceProviderProfile;
use diesel::pg::PgConnection; // Import controller functions
                              //use crate::schema::quality_factor_score::dsl::quality_factor_score;
use crate::model::NewQualityFactorScore::NewQualityFactorScore;
use crate::schema::quality_factor_score::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use serde::{Deserialize, Serialize};

type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(hello)))
        .service(web::resource("/echo").route(web::post().to(echo)))
        .service(web::resource("/craftsmen").route(web::get().to(getCraftsmen)))
        .service(web::resource("/craftman/{craftman_id}").route(web::patch().to(index)))
        .service(web::resource("/craftsman").route(web::get().to(get_top20_craftmen)));
}

#[derive(Debug, Deserialize)]
pub struct PatchRequest {
    // At least one of the attributes should be defined
    maxDrivingDistance: i32,
    profilePictureScore: f64,
    profileDescriptionScore: f64,
}

async fn index(
    pool: web::Data<DbPool>,
    craftman_id: web::Path<i32>,
    req_body: web::Json<PatchRequest>,
) -> actix_web::Result<impl Responder> {
    let updatedScore = web::block(move || {
        // So, it should be called within the `web::block` closure, as well.
        let mut conn = pool.get().expect("couldn't get db connection from pool");

        updateCraftman(&mut conn, craftman_id, req_body)
    })
    .await?;
    //.map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(updatedScore))
}

pub fn updateCraftman(
    conn: &mut PgConnection,
    craftman_id: web::Path<i32>,
    req_body: web::Json<PatchRequest>,
) -> NewQualityFactorScore {
    // TODO
    // implement

    let PatchRequest {
        maxDrivingDistance,
        profilePictureScore,
        profileDescriptionScore,
    } = req_body.0;

    let craftman_id = craftman_id.into_inner();
    // if maxDrivingDistance.is_none() && profilePictureScore.is_none() && profileDescriptionScore.is_none() {
    //    HttpResponse::InternalServerError().body("At least one updated value must be specified")
    let updatedScore = diesel::update(quality_factor_score.filter(profile_id.eq(1)))
        .set(profile_picture_score.eq(1.0))
        .get_result(conn)
        .expect("Error updating score");

    updatedScore
}

#[derive(Deserialize)]
pub struct PostalCodeQuery {
    postalcode: String,
}

pub async fn get_top20_craftmen(
    pool: web::Data<DbPool>,
    postal_code_parameter: web::Query<PostalCodeQuery>,
) -> actix_web::Result<impl Responder> {
    // So, it should be called within the `web::block` closure, as well.
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let postal_code = &postal_code_parameter.postalcode;

    let toptwenty = get_top_20_craftsmen(postal_code, &mut conn);
    Ok(web::Json(toptwenty))
}
