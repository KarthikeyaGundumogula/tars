use std::sync::Arc;

use axum::{
    body::Bytes,
    extract::{Path, State},
};
use sqlx::PgPool;
use uuid::Uuid;
use tracing::instrument;

use crate::{
    AppState,
    errors::ApiError,
    types::{
        db::work::WorkType,
        requests::works::{UploadEditReq, UploadPosterReq, UploadScriptReq},
        response::ApiResponse,
    }, utils::json_extractor::AppJson,
};

#[instrument(name = "create_new_work", skip(app, body), err)]
pub async fn create_new_work_handler(
    Path(work_type): Path<WorkType>,
    State(app): State<Arc<AppState>>,
    body: Bytes,
) -> Result<ApiResponse, ApiError> {
    let res = match work_type {
        WorkType::EDIT => {
            let AppJson(data) = AppJson::<UploadEditReq>::from_bytes(&body)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading edit"),
                None => tracing::info!("Uploading edit"),
            }
            upload_edit_handler(data, &app.pool).await
        }
        WorkType::POSTER => {
            let AppJson(data) = AppJson::<UploadPosterReq>::from_bytes(&body)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading poster"),
                None => tracing::info!("Uploading poster"),
            }
            upload_poster_handler(data).await
        }
        WorkType::SCRIPT => {
            let AppJson(data) = AppJson::<UploadScriptReq>::from_bytes(&body)?;
            match &data.title {
                Some(title) => tracing::info!(work_title = %title, "Uploading script"),
                None => tracing::info!("Uploading script"),
            }
            upload_script_handler(data).await
        }
    };
    Ok(ApiResponse::WorkCreated(res?))
}

async fn upload_edit_handler(_data: UploadEditReq, _pool: &PgPool) -> Result<Uuid, ApiError> {
    // let new_work = types::db::work::Work {
    //     id: Uuid::new_v4(),
    //     artist_id:
    // }
    // insert_new_work(pool, data).await
    Ok(Uuid::new_v4())
}

async fn upload_poster_handler(_data: UploadPosterReq) -> Result<Uuid, ApiError> {
    Ok(Uuid::new_v4())
}

async fn upload_script_handler(_data: UploadScriptReq) -> Result<Uuid, ApiError> {
    Ok(Uuid::new_v4())
}
