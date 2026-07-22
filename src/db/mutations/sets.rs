use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::ApiError, models::{
        db::sets::{DiscussionComment, DiscussionPost, Set, SetRole}, requests::sets::{UpdateCommentReq, UpdateDiscussionPostReq, UpdateSetReq},
    },
};

pub async fn insert_new_set(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    set: Set,
) -> Result<Uuid, ApiError> {
    let set_id = sqlx::query_scalar!(
        "
        INSERT INTO sets (id, name, statement, description, color_theme, presence, curator, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id;
        ",
        set.id,
        set.name,
        set.statement,
        set.description,
        set.color_theme,
        set.presence,
        set.curator,
        set.created_at
    )
    .fetch_one(&mut **txn)
    .await?;
    Ok(set_id)
}

pub async fn update_set(pool: &PgPool, set: UpdateSetReq, id: Uuid) -> Result<Uuid, ApiError> {
    Ok(
 sqlx::query_scalar!(
        "
        UPDATE sets
        SET name = COALESCE($1, name), statement = COALESCE($2, statement), description = COALESCE($3, description), color_theme = COALESCE($4, color_theme)
        WHERE id = $5 RETURNING id;
        ",
        set.name.as_ref().map(|n| n.as_str()),
        set.statement.as_ref().map(|s| s.as_str()),
        set.description.as_ref().map(|d| d.as_str()),
        set.profile_picture.as_ref().map(|p| p.as_str()),
        id
    )
    .fetch_one(pool)
    .await?)
}

pub async fn insert_set_member(
    txn: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    profile_id: Uuid,
    set_id: Uuid,
    set_role: SetRole,
    created_at: chrono::DateTime<chrono::Utc>,
) -> Result<SetRole, ApiError> {
    Ok(sqlx::query_scalar!(
        r#"
        INSERT INTO set_members (profile_id, set_id, set_role, created_at)
        VALUES ($1, $2, $3, $4) RETURNING set_role as "set_role:SetRole";
        "#,
        profile_id,
        set_id,
        set_role as SetRole,
        created_at
    )
    .fetch_one(&mut **txn)
    .await?)
}

pub async fn delete_set_member(
    pool: &PgPool,
    profile_id: Uuid,
    set_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
        DELETE FROM set_members
        WHERE profile_id = $1 AND set_id = $2;
        ",
        profile_id,
        set_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn update_set_member_status(
    pool: &PgPool,
    profile_id: Uuid,
    role: SetRole,
    set_id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
        UPDATE set_members
        SET set_role = $1
        WHERE profile_id = $2 AND set_id = $3;
        ",
        role as SetRole,
        profile_id,
        set_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn insert_new_discussion_post(
    pool: &PgPool,
    discussion: DiscussionPost,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
        INSERT INTO discussion_post (id, set_id, author_id, title, content, total_reactions, work_id, created_at, updated_at, last_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id;
        ",
        discussion.id,
        discussion.set_id,
        discussion.author_id,
        discussion.title,
        discussion.content,
        discussion.total_reactions,
        discussion.work_id,
        discussion.created_at,
        discussion.updated_at,
        discussion.last_active
    )
    .fetch_one(pool)
    .await?)
}

pub async fn update_discussion_post(
    pool: &PgPool,
    id: Uuid,
    data: UpdateDiscussionPostReq,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
        UPDATE discussion_post
        SET title = COALESCE($1, title), content = COALESCE($2, content), work_id = COALESCE($3, work_id), updated_at = NOW()
        WHERE id = $4
        ",
        data.title.as_ref().map(|t| t.as_str()),
        data.content.as_ref().map(|c| c.as_str()),
        data.work_id,
        id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn insert_new_discussion_comment(
    pool: &PgPool,
    comment: DiscussionComment,
) -> Result<Uuid, ApiError> {
    Ok(sqlx::query_scalar!(
        "
        WITH discussion_record AS (
            UPDATE discussion_post SET last_active = NOW(),total_reactions = total_reactions + 1 WHERE id = $2
        )
        INSERT INTO discussion_comments (id, discussion_post_id, author_id, content, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id;
        ",
        comment.id,
        comment.discussion_post_id,
        comment.author_id,
        comment.content,
        comment.created_at,
        comment.updated_at
    )
    .fetch_one(pool)
    .await?)
}

pub async fn update_comment(
    pool: &PgPool,
    comment_id: Uuid,
    data: UpdateCommentReq,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
        UPDATE discussion_comments
        SET content = COALESCE($1, content), updated_at = NOW()
        WHERE id = $2;
        ",
        data.content.as_ref().map(|c| c.as_str()),
        comment_id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_discussion_post(
    pool: &PgPool,
    id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
        DELETE FROM discussion_post
        WHERE id = $1;
        ",
        id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}

pub async fn delete_comment(
    pool: &PgPool,
    id: Uuid,
) -> Result<bool, ApiError> {
    Ok(sqlx::query!(
        "
        DELETE FROM discussion_comments
        WHERE id = $1;
        ",
        id
    )
    .execute(pool)
    .await?
    .rows_affected()
        == 1)
}
