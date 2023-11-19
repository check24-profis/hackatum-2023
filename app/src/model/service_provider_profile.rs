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
    pub fn calculate_profilescore_distance_rank(&self, lon: f64, lat: f64, connection: &mut PgConnection) -> (f64, f64, f64) {
        // :returns tuple of (profile_score, distance, rank)
        let quality_factor: NewQualityFactorScore = quality_factor_score
            .filter(
                profile_id
                    .eq::<i32>(TryInto::try_into(self.id).expect("Service provider id to big")),
            )
            .first(connection)
            .expect("Failed to calculate score from profile");


        let distance = self.calculate_distance(lon, lat);
        let distance_score = 1.0 - distance / 80.0;

        let distance_weight = match distance > 80.0 {
            true => 0.01,
            false => 0.15,
        };

       (quality_factor.profile_score.unwrap(), distance, distance_weight * distance_score + (1.0 - distance_weight) * quality_factor.profile_score.unwrap())
    }

    fn calculate_distance(&self, another_lon: f64, another_lat: f64) -> f64 {
        let lat = match self.lat {
            None => panic!("Cant calculate distance of non initialized craftsman"),
            Some(lat) => f64::to_radians(lat),
        };

        let lon = match self.lon {
            None => panic!("Cant calculate distance of non initialized craftsman"),
            Some(lon) => f64::to_radians(lon),
        };

        f64::acos(
            f64::sin(lat) * f64::sin(f64::to_radians(another_lat))
                + f64::cos(lat)
                    * f64::cos(f64::to_radians(another_lat))
                    * f64::cos(lon - f64::to_radians(another_lon)),
        ) * 6371 as f64
    }
}
