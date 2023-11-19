use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::schema::postcode)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Postcode {
    pub postal_code: String,
    pub lon: f64,
    pub lat: f64,
    pub postcode_extension_distance_group: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
