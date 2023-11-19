// @generated automatically by Diesel CLI.

diesel::table! {
    postcode (postal_code) {
        #[max_length = 5]
        postal_code -> Varchar,
        lon -> Float8,
        lat -> Float8,
        #[max_length = 255]
        postcode_extension_distance_group -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    quality_factor_score (profile_id) {
        profile_id -> Int4,
        profile_picture_score -> Float8,
        profile_description_score -> Float8,
        profile_score -> Nullable<Float8>,
    }
}

diesel::table! {
    service_provider_profile (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Nullable<Varchar>,
        #[max_length = 255]
        last_name -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
        #[max_length = 255]
        street -> Nullable<Varchar>,
        #[max_length = 255]
        house_number -> Nullable<Varchar>,
        lon -> Nullable<Float8>,
        lat -> Nullable<Float8>,
        max_driving_distance -> Nullable<Int4>,
        max_lon_a -> Nullable<Float8>,
        max_lon_b -> Nullable<Float8>,
        max_lon_c -> Nullable<Float8>,
        min_lon_a -> Nullable<Float8>,
        min_lon_b -> Nullable<Float8>,
        min_lon_c -> Nullable<Float8>,
        max_lat_a -> Nullable<Float8>,
        max_lat_b -> Nullable<Float8>,
        max_lat_c -> Nullable<Float8>,
        min_lat_a -> Nullable<Float8>,
        min_lat_b -> Nullable<Float8>,
        min_lat_c -> Nullable<Float8>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    postcode,
    quality_factor_score,
    service_provider_profile,
);
