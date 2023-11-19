use actix_web::{web, Responder};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};

use crate::buisiness_logic::{get_top_20_craftsmen, CraftmanResponse};

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct PostalCodeQuery {
    postalcode: String,
    page: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Check24Response {
    data: Vec<CraftmanResponse>,
}

pub async fn get_top20_craftmen(
    pool: web::Data<DbPool>,
    postal_code_parameter: web::Query<PostalCodeQuery>,
) -> actix_web::Result<impl Responder> {
    // So, it should be called within the `web::block` closure, as well.
    let mut conn = pool.get().expect("couldn't get db connection from pool");
    let postal_code = &postal_code_parameter.postalcode;

    let toptwenty = get_top_20_craftsmen(postal_code, &mut conn);
    let response = Check24Response { data: toptwenty };
    Ok(web::Json(response))
}
