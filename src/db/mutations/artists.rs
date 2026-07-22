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
        data.tag_line.as_ref().map(|s| s.as_str()),
        data.youtube_profile.as_ref().map(|s| s.as_str()),
        data.twitter_profile.as_ref().map(|s| s.as_str()),
        data.instagram_profile.as_ref().map(|s| s.as_str()),
        data.profile_picture.as_deref(),
        data.color_theme.as_ref().map(|s| s.as_str()),
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
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
    .execute(&mut **txn)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_favorite(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
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
    .execute(&mut **txn)
    .await?
    .rows_affected()
        == 1)
}

pub async fn insert_save_recommendation(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    profile_id: Uuid,
    recommendation_id: Uuid,
) -> Result<Uuid, ApiError> {
    let artist_id = sqlx::query_scalar!(
        r#"
        WITH inserted AS (
            INSERT INTO saved_recommendations (artist_id, recommendation_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT DO NOTHING
            RETURNING recommendation_id
        )
        UPDATE recommendations
            SET saves = saves+1
            WHERE id = $2 AND EXISTS (SELECT 1 FROM inserted)
            RETURNING artist_id
        "#,
        profile_id,
        recommendation_id
    )
    .fetch_optional(&mut **txn)
    .await?;
    match artist_id {
        Some(id) => Ok(id),
        None => Err(ApiError::BadRequest("Already saved".to_string())),
    }
}

pub async fn delete_save_recommendation(
    pool: &PgPool,
    profile_id: Uuid,
    recommendation_id: Uuid,
) -> Result<bool, ApiError> {
    let result = sqlx::query!(
        r#"
        WITH deleted AS (
            DELETE FROM saved_recommendations WHERE artist_id = $1 AND recommendation_id = $2 RETURNING recommendation_id
        )
        UPDATE recommendations
            SET saves = saves-1
            WHERE id = $2 AND EXISTS (SELECT 1 FROM deleted)
        "#,
        profile_id,
        recommendation_id
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

pub async fn insert_boost_recommendation(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    profile_id: Uuid,
    recommendation_id: Uuid,
) -> Result<Uuid, ApiError> {
    let artist_id = sqlx::query_scalar!(
        r#"
        WITH boost_records AS(
            INSERT INTO recommendation_boosts (recommendation_id, user_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT DO NOTHING
            RETURNING recommendation_id
        )
        UPDATE recommendations
            SET boost_number = recommendations.boost_number+1
            WHERE id = $1 AND EXISTS (SELECT 1 FROM boost_records)
            RETURNING artist_id
        "#,
        recommendation_id,
        profile_id
    )
    .fetch_optional(&mut **txn)
    .await?;
    match artist_id {
        Some(id) => Ok(id),
        None => Err(ApiError::BadRequest("Already boosted".to_string())),
    }
}

pub async fn increment_spirit_relation(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    profile_id: Uuid,
    user_id: Uuid,
) -> Result<Uuid, ApiError> {
    let spirit_id = sqlx::query_scalar!(
        r#"
        INSERT INTO spirit (artist, fan, token_count)
        VALUES ($1, $2, 0)
        ON CONFLICT (artist, fan)
        DO UPDATE SET token_count = spirit.token_count + 1
        RETURNING artist
        "#,
        profile_id,
        user_id
    )
    .fetch_one(&mut **txn)
    .await?;
    Ok(spirit_id)
}

pub async fn decrement_spirit_tokens(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    profile_id: Uuid,
    user_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
        WITH decremented AS(
         UPDATE spirit
        SET token_count = token_count - 1
        WHERE artist= $1 AND fan = $2
        RETURNING token_count
        )
        DELETE FROM spirit 
        WHERE (artist,fan ) IN (
            SELECT artist, fan FROM decremented WHERE token_count <= 0
        )
        "#,
        profile_id,
        user_id
    )
    .execute(&mut **txn)
    .await?
    .rows_affected()
        > 0)
}

pub async fn delete_boost_recommendation(
    pool: &PgPool,
    profile_id: Uuid,
    recommendation_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        r#"
        WITH boost_records AS(
            DELETE FROM recommendation_boosts 
            WHERE recommendation_id = $1 AND user_id = $2
            RETURNING recommendation_id
        )
        UPDATE recommendations
            SET boost_number = recommendations.boost_number-1
            WHERE id = $1 AND EXISTS (SELECT 1 FROM boost_records)
        "#,
        recommendation_id,
        profile_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        > 0)
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
