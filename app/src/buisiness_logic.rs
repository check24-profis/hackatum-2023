use crate::model::postcode::Postcode;
use crate::routing::craftsmen::PostalCodeQuery;
use crate::schema::postcode::dsl::*;
use crate::schema::service_provider_profile::dsl::*;
use crate::{
    model::service_provider_profile::ServiceProviderProfile, schema::postcode::postal_code,
};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};
use crate::routing::craftsmen::SortBy;

#[derive(Serialize, Deserialize)]
pub struct CraftmanResponse {
    id: i32,
    name: String,
    rankingScore: f64,
    distance: f64,
    profile_score: f64,
}

// has to be in radians
// pub fn calculate_distance(some_lon: f64, some_lat: f64, another_lon: f64, another_lat: f64) -> f64 {
//     f64::acos(
//         f64::sin(some_lat) * f64::sin(another_lat)
//             + f64::cos(some_lat) * f64::cos(another_lat) * f64::cos(some_lon - another_lon),
//     ) * 6371 as f64
// }

pub fn get_top_20_craftsmen(
    page_query: PostalCodeQuery,
    connection: &mut PgConnection,
) -> Vec<CraftmanResponse> {
    //TOOO: What to do in no existent postcode
    let postcode_struct: Postcode = postcode
        .filter(postal_code.eq(page_query.postalcode))
        .first(connection)
        .expect("Failed to load top craftsmen");

    let available_craftsmen: Vec<ServiceProviderProfile> =
        match postcode_struct.postcode_extension_distance_group.as_str() {
            "group_a" => service_provider_profile
                .filter(max_lat_a.ge(postcode_struct.lat))
                .filter(min_lat_a.le(postcode_struct.lat))
                .filter(max_lon_a.ge(postcode_struct.lon))
                .filter(min_lon_a.le(postcode_struct.lon))
                .load(connection)
                .expect("Failed to laod craftworkers by postcode"),

            "group_b" => service_provider_profile
                .filter(max_lat_b.ge(postcode_struct.lat))
                .filter(min_lat_b.le(postcode_struct.lat))
                .filter(max_lon_b.ge(postcode_struct.lon))
                .filter(min_lon_b.le(postcode_struct.lon))
                .load(connection)
                .expect("Failed to laod craftworkers by postcode"),

            "group_c" => service_provider_profile
                .filter(max_lat_c.ge(postcode_struct.lat))
                .filter(min_lat_c.le(postcode_struct.lat))
                .filter(max_lon_c.ge(postcode_struct.lon))
                .filter(min_lon_c.le(postcode_struct.lon))
                .load(connection)
                .expect("Failed to laod craftworkers by postcode"),

            _ => panic!("Returned a group from the database that does not exist"),
        };

    let mut craftmen_response: Vec<CraftmanResponse> = available_craftsmen
        .into_iter()
        .map(|c: ServiceProviderProfile| 
            {let profilescore_distance_rank = c.calculate_profilescore_distance_rank(postcode_struct.lon, postcode_struct.lat, connection);
            CraftmanResponse {
            id: c.id,
            name: format!(
                "{} {}",
                c.first_name.as_ref().unwrap(),
                c.last_name.as_ref().unwrap()
            ),
            profile_score: profilescore_distance_rank.0,
            distance: profilescore_distance_rank.1,
            rankingScore: profilescore_distance_rank.2,
            }})
        .collect();
    
    match page_query.sort_by {
        SortBy::Distance => {
            craftmen_response.sort_unstable_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
        },
        SortBy::Profile => {
            craftmen_response.sort_unstable_by(|a, b| b.profile_score.partial_cmp(&a.profile_score).unwrap().reverse());
        },
        SortBy::Default => {
            craftmen_response.sort_unstable_by(|a, b| {
                a.rankingScore
                    .partial_cmp(&b.rankingScore)
                    .unwrap()
                    .reverse()
            });
        },
    }    

    craftmen_response.into_iter().take(20).collect()
}
