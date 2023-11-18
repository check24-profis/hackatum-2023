-- Your SQL goes here
Update service_provider_profile
Set
  max_lat_a = lat + max_driving_distance/6371,
  min_lat_a = lat - max_driving_distance/6371,
  max_lat_b = lat + (max_driving_distance + 2)/6371,
  min_lat_b = lat - (max_driving_distance + 2)/6371,
  max_lat_c = lat + (max_driving_distance + 5)/6371,
  min_lat_c = lat - (max_driving_distance + 5)/6371,

  max_lon_a = lon + degrees(asin( 
    ((sin(radians(max_driving_distance/6371)) / cos(radians(lat)))
  ))),
  
  min_lon_a = lon - degrees(asin( 
    ((sin(radians(max_driving_distance/6371)) / cos(radians(lat)))
  ))),
  
  max_lon_b = lon + degrees(asin( 
    ((sin(radians((max_driving_distance + 2)/6371)) / cos(radians(lat)))
  ))),
  
  min_lon_b = lon - degrees(asin( 
    ((sin(radians((max_driving_distance + 2)/6371)) / cos(radians(lat)))
  ))),
  
  max_lon_c = lon + degrees(asin( 
    ((sin(radians((max_driving_distance + 5)/6371)) / cos(radians(lat)))
  ))),
  
  min_lon_c = lon - degrees(asin( 
    ((sin(radians((max_driving_distance + 5)/6371)) / cos(radians(lat)))
  )));
