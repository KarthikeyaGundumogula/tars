use sqlx::PgPool;
use uuid::Uuid;

use crate::{errors::ApiError, types::artist::Artist};

pub async fn register_new_artist(pool: &PgPool, data: Artist) -> Result<Option<Artist>, ApiError> {
    Ok(sqlx::query_as!(Artist,
        r#"
      INSERT INTO profiles(id,user_name,tag_line,is_claimed,youtube_profile,twitter_profile,instagram_profile,created_at,profile_picture,password_hash)
      VALUES ($1,$2,$3,false,$4,$5,$6,NOW (),$7,$8) RETURNING id,user_name,tag_line,is_claimed,youtube_profile,twitter_profile,instagram_profile,created_at,profile_picture,password_hash
      "#,
      Uuid::new_v4(),
      data.user_name,
      data.tag_line,
      data.youtube_profile,
      data.twitter_profile,
      data.instagram_profile,
      data.profile_picture,
      data.password_hash
    ).fetch_optional(pool).await?
  )
}
