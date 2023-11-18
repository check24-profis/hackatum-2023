use diesel::prelude::{Insertable, Queryable, AsChangeset};
use crate::Identifiable;
use serde::{Serialize, Deserialize};

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::quality_factor_score)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct NewQualityFactorScore {
    pub profile_id: i32,
    pub profile_picture_score: f64,
    pub profile_description_score: f64,
    pub profile_score: f64
}
