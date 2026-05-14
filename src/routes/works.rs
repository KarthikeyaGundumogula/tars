use std::sync::Arc;

use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    routing::post,
};
use chrono::Utc;
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    AppState,
    db::works::{insert_new_edit, insert_new_poster, insert_new_script, insert_new_work},
    errors::ApiError,
    types::{
        db::work::{Edit, Poster, Script, Work, WorkType},
        requests::works::{UploadEditReq, UploadPosterReq, UploadScriptReq},
        response::ApiResponse,
    },
    utils::{
        auth::extractor::{Artist, AuthUser},
        json_extractor::AppJson,
    },
};

#[instrument(name = "create_new_work", skip(app, body), err)]
pub async fn create_new_work_handler(
    Path(work_type): Path<WorkType>,
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
    body: Bytes,
) -> Result<ApiResponse, ApiError> {
    let res = match work_type {
        WorkType::EDIT => {
            let AppJson(data) = AppJson::<UploadEditReq>::from_bytes(&body)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading edit"),
                None => tracing::info!("Uploading edit"),
            }
            upload_edit(data, &app.db_pool, user).await
        }
        WorkType::POSTER => {
            let AppJson(data) = AppJson::<UploadPosterReq>::from_bytes(&body)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading poster"),
                None => tracing::info!("Uploading poster"),
            }
            upload_poster(data, user, &app.db_pool).await
        }
        WorkType::SCRIPT => {
            let AppJson(data) = AppJson::<UploadScriptReq>::from_bytes(&body)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading script"),
                None => tracing::info!("Uploading script"),
            }
            upload_script(data, user, &app.db_pool).await
        }
    };
    Ok(ApiResponse::WorkCreated(res?))
}

async fn upload_edit(data: UploadEditReq, pool: &PgPool, user: AuthUser) -> Result<Uuid, ApiError> {
    let new_work = Work {
        id: Uuid::new_v4(),
        artist_id: user.profile_id,
        title: data.title.map(|t| t.to_string()),
        credits: Some(0),
        created_at: Utc::now(),
        category: WorkType::EDIT,
    };
    let edit = Edit {
        work_id: new_work.id,
        src_id: data.src_id,
        platform: data.platform,
        format: data.format,
    };
    let mut txn = pool.begin().await?;
    let new_work_id = insert_new_work(&mut txn, new_work).await?;
    insert_new_edit(&mut txn, edit).await?;
    txn.commit().await?;
    Ok(new_work_id)
}

async fn upload_poster(
    data: UploadPosterReq,
    user: AuthUser,
    pool: &PgPool,
) -> Result<Uuid, ApiError> {
    let new_work = Work {
        id: Uuid::new_v4(),
        artist_id: user.profile_id,
        title: data.title.map(|t| t.to_string()),
        credits: Some(0),
        created_at: Utc::now(),
        category: WorkType::POSTER,
    };
    let poster = Poster {
        work_id: new_work.id,
        src_id: data.src_id,
        format: data.format,
    };
    let mut txn = pool.begin().await?;
    let new_work_id = insert_new_work(&mut txn, new_work).await?;
    insert_new_poster(&mut txn, poster).await?;
    txn.commit().await?;
    Ok(new_work_id)
}

async fn upload_script(
    data: UploadScriptReq,
    user: AuthUser,
    pool: &PgPool,
) -> Result<Uuid, ApiError> {
    let new_work = Work {
        id: Uuid::new_v4(),
        artist_id: user.profile_id,
        title: data.title.map(|t| t.to_string()),
        credits: Some(0),
        created_at: Utc::now(),
        category: WorkType::SCRIPT,
    };
    let script = Script {
        work_id: new_work.id,
        img_src_ids: data.src_ids,
        thoughts: data.thoughts.iter().map(|t| t.to_string()).collect(),
    };
    let mut txn = pool.begin().await?;
    let new_work_id = insert_new_work(&mut txn, new_work).await?;
    insert_new_script(&mut txn, script).await?;
    txn.commit().await?;
    Ok(new_work_id)
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/new/{work_type}", post(create_new_work_handler))
}
