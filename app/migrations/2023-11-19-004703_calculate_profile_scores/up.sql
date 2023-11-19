UPDATE quality_factor_score 
SET profile_score = 0.4 * profile_picture_score + 0.6 * profile_description_score;