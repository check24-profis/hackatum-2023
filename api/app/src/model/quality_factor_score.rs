use diesel::prelude::{Insertable, Queryable};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::schema::quality_factor_score)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewQualityFactorScore {
    pub profile_id: i32,
    pub profile_picture_score: f64,
    pub profile_description_score: f64,
}
