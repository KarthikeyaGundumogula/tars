use std::sync::Arc;

use axum::{
    Json, Router,
    extract::State,
    routing::{delete, post},
};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::mutations::{
        festivals::{insert_new_festival, insert_new_panelist},
        sets::{
            delete_comment, delete_discussion_post, delete_set_member,
            insert_new_discussion_comment, insert_new_discussion_post, insert_new_set,
            insert_set_member, update_comment, update_discussion_post, update_set,
        },
    },
    errors::ApiError,
    models::{
        db::{
            festivals::{Festival, Panelist},
            sets::{DiscussionComment, DiscussionPost, Set, SetMember, SetRole},
        },
        requests::{
            festivals::CreateFestivalReq,
            sets::{
                CreateSetReq, JoinSetRequest, NewDiscussionCommentReq, NewDiscussionPostReq,
                UpdateCommentReq, UpdateDiscussionPostReq, UpdateSetReq,
            },
        },
        response::{FestivalResponse, SetResponse},
    },
    services::{
        auth_service::extractor::{
            Artist, EntityMemberOrAdmin, OrganizerOrAdmin, OwnedResourceOrAdmin,
        },
        json_extractor::AppJson,
    },
};

#[instrument(name = "create_new_set", skip(app, user, data), fields(curator= %user.handle, set_name = %data.name))]
pub async fn create_new_set_handler(
    State(app): State<Arc<AppState>>,
    OrganizerOrAdmin(user): OrganizerOrAdmin,
    AppJson(data): AppJson<CreateSetReq>,
) -> Result<SetResponse, ApiError> {
    let set = Set {
        id: Uuid::new_v4(),
        name: data.name.to_string(),
        statement: data.statement.to_string(),
        description: data.description.to_string(),
        color_theme: data.color_theme,
        curator: user.profile_id,
        presence: 0,
        created_at: Utc::now(),
    };
    let mut txn = app.db_pool.begin().await?;
    let set_id = insert_new_set(&mut txn, set).await?;
    insert_set_member(
        &mut txn,
        user.profile_id,
        set_id,
        SetRole::CURATOR,
        Utc::now(),
    )
    .await?;
    txn.commit().await?;
    Ok(SetResponse::SetCreated(set_id))
}

#[instrument(name = "create_new_festival", skip(app, data), fields(user_id = %user_id, festival_name = %data.name))]
pub async fn create_new_festival_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin {
        resource_id,
        user_id,
        ..
    }: OwnedResourceOrAdmin<Set>,
    AppJson(data): AppJson<CreateFestivalReq>,
) -> Result<FestivalResponse, ApiError> {
    let festival = Festival {
        id: Uuid::new_v4(),
        name: data.name.to_string(),
        description: data.description.to_string(),
        set_id: resource_id,
        organizer: user_id,
        start_date: data.start_date,
        end_date: data.end_date,
        rules: data.rules.map(|r| r.to_string()),
        created_at: Utc::now(),
    };
    let mut txn = app.db_pool.begin().await?;
    let set_id = insert_new_festival(&mut txn, festival).await?;
    for panelist in data.panelists {
        let panelist = Panelist {
            festival_id: set_id,
            profile_id: panelist,
            work_id: None,
            created_at: Utc::now(),
        };
        tracing::info!("Inserting panelist: {}", panelist.profile_id);
        insert_new_panelist(&mut txn, panelist).await?;
    }
    txn.commit().await?;
    Ok(FestivalResponse::FestivalCreated(set_id))
}

async fn update_set_details_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Set>,
    AppJson(data): AppJson<UpdateSetReq>,
) -> Result<SetResponse, ApiError> {
    let res = update_set(&app.db_pool, data, resource_id).await?;
    Ok(SetResponse::UpdatedSet(res))
}
async fn join_set_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<JoinSetRequest>,
) -> Result<SetResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let role = insert_set_member(
        &mut txn,
        user.profile_id,
        data.set_id,
        SetRole::MEMBER,
        Utc::now(),
    )
    .await?;
    txn.commit().await?;
    Ok(SetResponse::JoinedSet(role))
}
async fn leave_set_handler(
    State(app): State<Arc<AppState>>,
    EntityMemberOrAdmin {
        entity_id, user_id, ..
    }: EntityMemberOrAdmin<SetMember>,
) -> Result<SetResponse, ApiError> {
    delete_set_member(&app.db_pool, user_id, entity_id).await?;
    Ok(SetResponse::SetMemberDeleted(entity_id, user_id))
}

async fn create_new_discussion_handler(
    State(app): State<Arc<AppState>>,
    EntityMemberOrAdmin {
        user_id, entity_id, ..
    }: EntityMemberOrAdmin<SetMember>,
    Json(data): Json<NewDiscussionPostReq>,
) -> Result<SetResponse, ApiError> {
    let discussion = DiscussionPost {
        id: Uuid::new_v4(),
        set_id: Some(entity_id),
        author_id: Some(user_id),
        title: data.title.to_string(),
        content: data.content.to_string(),
        total_reactions: 0,
        work_id: data.work_id,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        last_active: Utc::now(),
    };
    let res = insert_new_discussion_post(&app.db_pool, discussion).await?;
    Ok(SetResponse::CreatedDiscussion(res))
}

async fn create_new_comment_handler(
    State(app): State<Arc<AppState>>,
    EntityMemberOrAdmin { user_id, .. }: EntityMemberOrAdmin<SetMember>,
    Json(data): Json<NewDiscussionCommentReq>,
) -> Result<SetResponse, ApiError> {
    let comment = DiscussionComment {
        id: Uuid::new_v4(),
        discussion_post_id: data.discussion_id,
        author_id: Some(user_id),
        parent_id: data.parent_id,
        content: data.content.to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    let res = insert_new_discussion_comment(&app.db_pool, comment).await?;
    Ok(SetResponse::CreatedComment(res))
}

async fn update_discussion_post_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<DiscussionPost>,
    AppJson(data): AppJson<UpdateDiscussionPostReq>,
) -> Result<SetResponse, ApiError> {
    update_discussion_post(&app.db_pool, resource_id, data).await?;
    Ok(SetResponse::UpdatedDiscussionPost(resource_id))
}

async fn update_comment_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<DiscussionComment>,
    AppJson(data): AppJson<UpdateCommentReq>,
) -> Result<SetResponse, ApiError> {
    update_comment(&app.db_pool, resource_id, data).await?;
    Ok(SetResponse::UpdatedComment(resource_id))
}

async fn delete_discussion_post_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<DiscussionPost>,
) -> Result<SetResponse, ApiError> {
    delete_discussion_post(&app.db_pool, resource_id).await?;
    Ok(SetResponse::DeletedDiscussionPost(resource_id))
}

async fn delete_comment_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<DiscussionComment>,
) -> Result<SetResponse, ApiError> {
    delete_comment(&app.db_pool, resource_id).await?;
    Ok(SetResponse::DeletedComment(resource_id))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(create_new_set_handler))
        .route(
            "/{resource_id}/new_festival",
            post(create_new_festival_handler),
        )
        .route("/{resource_id}/update", post(update_set_details_handler))
        .route("/join", post(join_set_handler))
        .route("/{entity_id}/leave", delete(leave_set_handler))
        .route(
            "/{entity_id}/new/discussion",
            post(create_new_discussion_handler),
        )
        .route("/{entity_id}/new/comment", post(create_new_comment_handler))
        .route(
            "/update/comment/{resource_id}",
            post(update_comment_handler),
        )
        .route(
            "/update/discussion_post/{resource_id}",
            post(update_discussion_post_handler),
        )
        .route(
            "/delete/discussion_post/{resource_id}",
            delete(delete_discussion_post_handler),
        )
        .route(
            "/delete/comment/{resource_id}",
            delete(delete_comment_handler),
        )
}
