use std::sync::Arc;

use axum::{extract::State, Router, routing::post};
use chrono::Utc;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::originals::{insert_new_original, insert_new_role},
    errors::ApiError,
    types::{
        db::{
            original::Original,
            profile::{Role, RoleType},
        },
        requests::originals::CreateOriginalReq,
        response::ApiResponse,
    },
    shared::{auth::password::get_password_hash, json_extractor::AppJson},
};
#[instrument(name = "create_new_original", skip(app, data), err, fields(title = %data.title))]
pub async fn create_new_original_handler(
    State(app): State<Arc<AppState>>,
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
        genres: data.genres.into_iter().map(|g| Some(g.to_string())).collect(),
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

async fn update_original_details() -> Result<ApiResponse, ApiError> {
    todo!()
}

async fn new_release_handler() -> Result<ApiResponse, ApiError> {
    todo!()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(create_new_original_handler))
        .route("/{resource_id}/update", post(update_original_details))
        .route("/new_release", post(new_release_handler))
}
