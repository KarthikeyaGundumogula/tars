use std::sync::Arc;

use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    routing::{delete, post},
};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::mutations::originals::{
        add_new_role_if_not_exists, delete_original, delete_role, insert_new_original,
        insert_new_role, update_original,
    },
    errors::ApiError,
    services::{
        auth_service::{extractor::AdminUser, password::get_password_hash},
        json_extractor::AppJson,
        upload_service::upload_work,
    },
    types::{
        db::{
            original::Original,
            profile::{Role, RoleType},
            work::WorkType,
        },
        requests::originals::{AddNewRoleReq, CreateOriginalReq, RemoveRoleReq, UpdateOrignalReq},
        response::{ApiResponse, OriginalResponse},
    },
};
#[instrument(name = "create_new_original", skip(app, data), err, fields(title = %data.title))]
pub async fn create_new_original_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    AppJson(data): AppJson<CreateOriginalReq>,
) -> Result<ApiResponse, ApiError> {
    let password_hash = get_password_hash(data.password.as_ref())?;
    let mut txn = app.db_pool.begin().await?;
    let original = Original {
        id: Uuid::new_v4(),
        title: data.title.to_string(),
        release_date: data.release_date,
        description: data.description.to_string(),
        cover_img: data.cover_img,
        presence: Some(100),
        password_hash,
        associated_with: Some(data.associated_with),
        genres: data
            .genres
            .into_iter()
            .map(|g| Some(g.to_string()))
            .collect(),
        created_at: Utc::now(),
        parent: None,
        category: crate::types::db::original::OriginalCategory::MOVIE,
        duration: None,
    };
    let original_id = insert_new_original(&mut txn, original).await?;
    for star in data.stars.iter() {
        let role = Role {
            profile_id: star.artist,
            category: RoleType::STAR,
            original_id,
            role_name: star.role.to_string(),
            created_at: Utc::now(),
        };
        insert_new_role(&mut txn, role).await?;
    }
    for maker in data.makers.iter() {
        let role = Role {
            profile_id: maker.artist,
            category: RoleType::MAKER,
            original_id,
            role_name: maker.role.to_string(),
            created_at: Utc::now(),
        };
        insert_new_role(&mut txn, role).await?;
    }
    txn.commit().await?;
    tracing::info!("Original created successfully: {}", original_id);
    Ok(ApiResponse::OK)
}

#[instrument(
    name = "update_original_details",
    skip(app, data),
    fields(original_id = %original_id)
)]
async fn update_original_details(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Path(original_id): Path<Uuid>,
    AppJson(data): AppJson<UpdateOrignalReq>,
) -> Result<OriginalResponse, ApiError> {
    let res = update_original(&app.db_pool, data, original_id).await?;
    Ok(OriginalResponse::OriginalUpdated(res))
}

#[instrument(name = "add_new_role_to_original",skip(app,data),fields(original_id = %original_id,profile_id=%data.profile_id))]
async fn add_new_role_handler(
    State(app): State<Arc<AppState>>,
    Path(original_id): Path<Uuid>,
    AdminUser(_): AdminUser,
    AppJson(data): AppJson<AddNewRoleReq>,
) -> Result<OriginalResponse, ApiError> {
    let role = Role {
        profile_id: data.profile_id,
        category: data.category,
        original_id,
        role_name: data.role_name.to_string(),
        created_at: Utc::now(),
    };
    let res = add_new_role_if_not_exists(&app.db_pool, role).await?;
    match res {
        true => Ok(OriginalResponse::RoleCreated(data.profile_id)),
        false => Ok(OriginalResponse::RoleExists(data.profile_id)),
    }
}

#[instrument(name = "delete_role_in_original", skip(app, data), fields(original_id = %original_id,profile=%data.profile_id))]
async fn delete_role_from_original_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Path(original_id): Path<Uuid>,
    AppJson(data): AppJson<RemoveRoleReq>,
) -> Result<OriginalResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    delete_role(
        &mut txn,
        data.role_name.to_string(),
        original_id,
        data.profile_id,
    )
    .await?;
    txn.commit().await?;
    Ok(OriginalResponse::RoleDeleted(data.profile_id))
}

#[instrument(name = "delete original", skip(app),fields(original_id = %original_id))]
async fn delete_original_handler(
    State(app): State<Arc<AppState>>,
    AdminUser(_): AdminUser,
    Path(original_id): Path<Uuid>,
) -> Result<OriginalResponse, ApiError> {
    delete_original(&app.db_pool, original_id).await?;
    Ok(OriginalResponse::OriginalDeleted(original_id))
}

#[instrument(name = "upload_release", skip(app, data), fields(resource_id = %resource_id))]
async fn new_release_handler(
    State(app): State<Arc<AppState>>,
    Path((resource_id, release_type)): Path<(Uuid, WorkType)>,
    AdminUser(_): AdminUser,
    data: Bytes,
) -> Result<OriginalResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let res = upload_work(data, &mut txn, resource_id, release_type).await?;
    txn.commit().await?;
    Ok(OriginalResponse::OrignalReleaseCreated(res))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(create_new_original_handler))
        .route("/{original_id}/update", post(update_original_details))
        .route("/{original_id}/new_role", post(add_new_role_handler))
        .route(
            "/{original_id}/delete_role",
            delete(delete_role_from_original_handler),
        )
        .route("/{original_id}/delete", delete(delete_original_handler))
        .route(
            "/{resource_id}/new_release/{release_type}",
            post(new_release_handler),
        )
}
