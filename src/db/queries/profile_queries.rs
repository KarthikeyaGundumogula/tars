use sqlx::PgPool;


pub async fn get_profile_stage_by_username(pool:&PgPool,user_name:&str) -> Result<(), sqlx::Error> {
    sqlx::query!("SELECT user_name,tag_line,youtube_profile,twitter_profile,instagram_profile,profile_picture FROM profiles WHERE user_name = $1", user_name)
        .fetch_one(pool)
        .await?;
    Ok(())
}