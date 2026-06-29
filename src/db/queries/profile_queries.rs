use crate::{
    errors::ApiError,
    types::{
        db::work::WorkCategory,
        response::artist::{ArtistStage, WorkPreview},
    },
};
use sqlx::PgPool;

pub async fn get_profile_details_by_username(
    pool: &PgPool,
    user_name: &str,
) -> Result<ArtistStage, ApiError> {
    let rows= sqlx::query!(
        r#"SELECT pf.user_name,pf.tag_line,pf.stage_name,pf.background_color,pf.text_color, pf.youtube_profile,pf.twitter_profile,pf.instagram_profile,pf.profile_picture,works.title work_title,works.category AS "category: WorkCategory" FROM profiles AS pf INNER JOIN works ON works.artist_id = pf.id WHERE pf.user_name = $1 ORDER BY works.created_at DESC LIMIT 6"#,
        user_name
    )
    .fetch_all(pool)
    .await?;
    let mut works = Vec::<WorkPreview>::new();
    for row in rows.iter() {
        works.push(WorkPreview {
            title: row.work_title.clone(),
            work_type: row.category.as_ref().to_string(),
        });
    }
    if rows.is_empty() {
        return Err(ApiError::NotFound);
    }
    Ok(ArtistStage {
        stage_name: rows[0].stage_name.clone(),
        user_name: rows[0].user_name.clone(),
        tag_line: rows[0].tag_line.clone(),
        youtube_profile: rows[0].youtube_profile.clone(),
        twitter_profile: rows[0].twitter_profile.clone(),
        instagram_profile: rows[0].instagram_profile.clone(),
        presence: 0,
        text_color: rows[0].text_color.clone(),
        background_color: rows[0].background_color.clone(),
        profile_picture: rows[0].profile_picture.clone(),
        works,
    })
}
