use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError,
    models::{
        db::profile::{Profile, ProfileType},
        requests::artist::UpdateProfileReq,
    },
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
          spirit,
          stage_name,
          color_theme,
          role_name,
          current_peak_recommendations,
          current_peak_library
        )
      VALUES ($1, $2, $3, false, $4, $5, $6, NOW (), $7, $8, $9, $10, $11, $12, $13, $14, $15)
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
        spirit,
        stage_name,
        color_theme,
        role_name,
        current_peak_recommendations,
        current_peak_library
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
        &data.spirit,
        data.stage_name,
        data.color_theme,
        data.role_name,
        data.current_peak_recommendations,
        data.current_peak_library
    )
    .fetch_optional(pool)
    .await?)
}

pub async fn update_profile_password(
    pool: &PgPool,
    profile_id: Uuid,
    password_hash: String,
) -> Result<Option<Uuid>, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"UPDATE profiles SET password_hash = $1 WHERE id = $2 RETURNING id"#,
        password_hash,
        profile_id
    )
    .fetch_optional(pool)
    .await?)
}

pub async fn update_profile_details(
    pool: &PgPool,
    data: UpdateProfileReq,
    id: Uuid,
) -> Result<Option<Uuid>, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
            UPDATE profiles
            SET stage_name = COALESCE($1, stage_name),
            tag_line = COALESCE($2, tag_line),
            youtube_profile = COALESCE($3, youtube_profile),
            twitter_profile = COALESCE($4, twitter_profile),
            instagram_profile = COALESCE($5, instagram_profile),
            profile_picture = COALESCE($6, profile_picture),
            color_theme = COALESCE($7, color_theme)
            WHERE id = $8
            RETURNING id
        "#,
        data.stage_name.as_ref().map(|s| s.as_str()),
        data.tag_line,
        data.youtube_profile,
        data.twitter_profile,
        data.instagram_profile,
        data.profile_picture,
        data.color_theme,
        id
    )
    .fetch_optional(pool)
    .await?)
}

pub async fn update_profile_role(
    pool: &PgPool,
    new_role: String,
    profile_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
            UPDATE profiles
            SET role_name = $1
            WHERE id = $2
            "#,
        new_role,
        profile_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn insert_new_favorite(
    pool: &PgPool,
    profile_id: Uuid,
    favoriting_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
        INSERT INTO favorite_profiles (profile_id, favorited_id,created_at)
        VALUES ($1, $2,NOW())
        "#,
        profile_id,
        favoriting_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_favorite(
    pool: &PgPool,
    profile_id: Uuid,
    favoriting_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
        DELETE FROM favorite_profiles
        WHERE profile_id = $1 AND favorited_id = $2
        "#,
        profile_id,
        favoriting_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn update_profile_presence(
    pool: &PgPool,
    profile_id: Uuid,
    spirit: i64,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
        UPDATE profiles
        SET spirit = $1
        WHERE id = $2
        "#,
        &spirit,
        profile_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn get_profile_auth_details(
    pool: &PgPool,
    user_name: &String,
) -> Result<Option<Profile>, ApiError> {
    Ok(sqlx::query_as!(
        Profile,
        r#"
            SELECT id,
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
            spirit,
            stage_name,
            color_theme,
            role_name,
            current_peak_recommendations,
            current_peak_library
            FROM profiles
            WHERE user_name = $1
        "#,
        user_name
    )
    .fetch_optional(pool)
    .await?)
}
