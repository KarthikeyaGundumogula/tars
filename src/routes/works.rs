use std::sync::Arc;

use axum::{
    Json,
    body::Bytes,
    extract::{Path, State},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    AppState,
    errors::ApiError,
    types::{
        db::work::WorkType,
        requests::works::{UploadEditReq, UploadPosterReq, UploadScriptReq},
        response::ApiResponse,
    },
};

pub async fn create_new_work_handler(
    Path(work_type): Path<WorkType>,
    State(app): State<Arc<AppState>>,
    body: Bytes,
) -> Result<ApiResponse, ApiError> {
    let res = match work_type {
        WorkType::EDIT => {
            let Json(data) = Json::<UploadEditReq>::from_bytes(&body)?;
            upload_edit_handler(data, &app.pool).await
        }
        WorkType::POSTER => {
            let Json(data) = Json::<UploadPosterReq>::from_bytes(&body)?;
            upload_poster_handler(data).await
        }
        WorkType::SCRIPT => {
            let Json(data) = Json::<UploadScriptReq>::from_bytes(&body)?;
            upload_script_handler(data).await
        }
    };
    Ok(ApiResponse::WorkCreated(res?))
}

async fn upload_edit_handler(data: UploadEditReq, pool: &PgPool) -> Result<Uuid, ApiError> {
    // let new_work = types::db::work::Work {
    //     id: Uuid::new_v4(),
    //     artist_id:
    // }
    // insert_new_work(pool, data).await
    Ok(Uuid::new_v4())
}

async fn upload_poster_handler(data: UploadPosterReq) -> Result<Uuid, ApiError> {
    Ok(Uuid::new_v4())
}

async fn upload_script_handler(data: UploadScriptReq) -> Result<Uuid, ApiError> {
    Ok(Uuid::new_v4())
}
