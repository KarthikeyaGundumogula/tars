use sqlx::PgPool;

use crate::{
    domain::Handle,
    errors::ApiError,
    types::db::profile::{Profile, ProfileType},
};

pub async fn insert_new_profile(pool: &PgPool, data: Profile) -> Result<Option<Profile>, ApiError> {
    Ok(sqlx::query_as!(
        Profile,
        r#"
      INSERT INTO profiles(
          id,
          user_name,
          tag_line,
          is_claimed,
          youtube_profile,
          twitter_profile,
          instagram_profile,
          created_at,
          profile_picture,
          password_hash,
          profile_type,
          presence
        )
      VALUES ($1, $2, $3, false, $4, $5, $6, NOW (), $7, $8, $9, $10)
      RETURNING id,
        user_name,
        tag_line,
        is_claimed,
        youtube_profile,
        twitter_profile,
        instagram_profile,
        created_at,
        profile_picture,
        password_hash,
        profile_type as "profile_type: ProfileType",
        presence 
        "#,
        data.id,
        data.user_name,
        data.tag_line,
        data.youtube_profile,
        data.twitter_profile,
        data.instagram_profile,
        data.profile_picture,
        data.password_hash,
        data.profile_type as ProfileType,
        data.presence
    )
    .fetch_optional(pool)
    .await?)
}

pub async fn get_profile_auth_details(
    pool: &PgPool,
    user_name: &Handle,
) -> Result<Option<Profile>, ApiError> {
    Ok(sqlx::query_as!(
        Profile,
        r#"SELECT id, user_name, tag_line, is_claimed, youtube_profile, twitter_profile, instagram_profile, created_at, profile_picture, password_hash, profile_type as "profile_type: ProfileType", presence FROM profiles WHERE user_name=$1"#,
        user_name.as_ref()
    )
    .fetch_optional(pool)
    .await?)
}
