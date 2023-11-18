-- This file should undo anything in `up.sql`
Update service_provider_profile
Set
  max_lat_a = NULL,
  min_lat_a = NULL,
  max_lat_b = NULL,
  min_lat_b = NULL,
  max_lat_c = NULL,
  min_lat_c = NULL,
  
  max_lon_a = NULL;
