use diesel::prelude::*;
use diesel::{
    prelude::{Insertable, Queryable},
    ExpressionMethods, PgConnection, QueryDsl,
};

use super::NewQualityFactorScore::NewQualityFactorScore;
use crate::schema::quality_factor_score::dsl::*;

#[derive(Debug, Queryable, Insertable)]
#[diesel(table_name = crate::schema::service_provider_profile)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ServiceProviderProfile {
    pub id: i32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub city: Option<String>,
    pub street: Option<String>,
    pub house_number: Option<String>,
    pub lon: Option<f64>,
    pub lat: Option<f64>,
    pub max_driving_distance: Option<i32>,
    pub max_lon_a: Option<f64>,
    pub max_lon_b: Option<f64>,
    pub max_lon_c: Option<f64>,
    pub min_lon_a: Option<f64>,
    pub min_lon_b: Option<f64>,
    pub min_lon_c: Option<f64>,
    pub max_lat_a: Option<f64>,
    pub max_lat_b: Option<f64>,
    pub max_lat_c: Option<f64>,
    pub min_lat_a: Option<f64>,
    pub min_lat_b: Option<f64>,
    pub min_lat_c: Option<f64>,
}

impl ServiceProviderProfile {
    pub fn calculated_score(&self, lon: f64, lat: f64, connection: &mut PgConnection) -> f64 {
        let quality_factor: NewQualityFactorScore = quality_factor_score
            .filter(
                profile_id
                    .eq::<i32>(TryInto::try_into(self.id).expect("Service provider id to big")),
            )
            .first(connection)
            .expect("Failed to calculate score from profile");
        let profile_score_calculated = quality_factor.profile_description_score * 0.6
            + quality_factor.profile_picture_score * 0.4;

        let distance = self.calculate_distance(lon, lat);
        let distance_score = 1.0 - distance / 80.0;

        let distance_weight = match distance > 80.0 {
            true => 0.01,
            false => 0.15,
        };

        distance_weight * distance_score + (1.0 - distance_weight) * profile_score_calculated
    }

    pub fn calculate_distance(&self, another_lon: f64, another_lat: f64) -> f64 {
        let lat = match self.lat {
            None => panic!("Cant calculate distance of non initialized craftsman"),
            Some(lat) => lat,
        };

        let lon = match self.lon {
            None => panic!("Cant calculate distance of non initialized craftsman"),
            Some(lon) => lon,
        };

        f64::acos(
            f64::sin(lat) * f64::sin(another_lat)
                + f64::cos(lat) * f64::cos(another_lat) * f64::cos(lon - another_lon),
        ) * 6371 as f64
    }
}
