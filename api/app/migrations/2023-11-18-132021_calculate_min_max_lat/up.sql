-- Your SQL goes here
Update service_provider_profile
Set
  max_lat_a = degrees(radians(lat) + (max_driving_distance::double precision)/6371000),
  min_lat_a = degrees(radians(lat) - (max_driving_distance::double precision)/6371000),
  max_lat_b = degrees(radians(lat) + (max_driving_distance::double precision + 2000)/6371000),
  min_lat_b = radians(lat) - degrees((max_driving_distance + 2000)/6371000),
  max_lat_c = radians(lat) + degrees((max_driving_distance + 5000)/6371000),
  min_lat_c = radians(lat) - degrees((max_driving_distance + 5000)/6371000),

  max_lon_a = lon + degrees(asin( 
    ((sin(max_driving_distance/6371000) / cos(radians(lat)))
  ))),
  
  min_lon_a = lon - degrees(asin( 
    ((sin(max_driving_distance/6371000) / cos(radians(lat)))
  ))),
  
  max_lon_b = lon + degrees(asin( 
    ((sin((max_driving_distance + 2000)/6371000) / cos(radians(lat)))
  ))),
  
  min_lon_b = lon - degrees(asin( 
    ((sin((max_driving_distance + 2000)/6371000) / cos(radians(lat)))
  ))),
  
  max_lon_c = lon + degrees(asin( 
    ((sin((max_driving_distance + 5000)/6371000) / cos(radians(lat)))
  ))),
  
  min_lon_c = lon - degrees(asin( 
    ((sin((max_driving_distance + 5000)/6371000) / cos(radians(lat)))
  )));
