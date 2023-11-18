use crate::buisiness_logic::get_top_20_craftsmen;
use crate::controller::{echo, getCraftsmen, hello};
use crate::updateController::{update_craftsman};
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




pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/craftsmen")
        .route(web::get().to(getCraftsmen))
    )
    .service(
        web::resource("/craftman/{craftman_id}")
        .route(web::patch().to(update_craftsman))
    )
    ;
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