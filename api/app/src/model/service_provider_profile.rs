use chrono::NaiveDateTime;
use diesel::prelude::{Insertable, Queryable};

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::schema::service_provider_profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceProviderProfile {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub street: String,
    pub house_number: String,
    pub lon: f64,
    pub lat: f64,
    pub max_driving_distance: i32,
}