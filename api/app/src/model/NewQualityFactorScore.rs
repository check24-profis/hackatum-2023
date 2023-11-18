use crate::Identifiable;
use diesel::prelude::{AsChangeset, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::quality_factor_score)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[derive(Serialize)]
pub struct NewQualityFactorScore {
    pub profile_id: i32,
    pub profile_picture_score: f64,
    pub profile_description_score: f64,
    pub profile_score: Option<f64>
}
