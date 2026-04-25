use std::sync::Arc;

use axum::{Json, body::Bytes, extract::{Path, State}};
use sqlx::{PgPool};
use uuid::Uuid;

use crate::{
    db::works::create_edit_work, errors::ApiError, types::{
        requests::works::{UploadEditData, UploadPosterData, UploadScriptData, WorkType},
        response::ApiResponse,
    }
};

pub async fn create_new_work_handler(
    Path(work_type): Path<WorkType>,
    State(app):State<Arc<PgPool>>,
    body: Bytes,
) -> Result<ApiResponse, ApiError> {
    let res = match work_type {
        WorkType::Edit => {
            let Json(data) = Json::<UploadEditData>::from_bytes(&body)?;
            upload_edit_handler(data,&app).await
        }
        WorkType::Poster => {
            let Json(data) = Json::<UploadPosterData>::from_bytes(&body)?;
            upload_poster_handler(data).await
        }
        WorkType::Script => {
            let Json(data) = Json::<UploadScriptData>::from_bytes(&body)?;
            upload_script_handler(data).await
        }
    };
    Ok(ApiResponse::WorkCreated(res?))

}

async fn upload_edit_handler(data: UploadEditData,pool:&PgPool) -> Result<Uuid,ApiError>{
    create_edit_work(pool, data).await
}

async fn upload_poster_handler(data: UploadPosterData) -> Result<Uuid,ApiError>{
    Ok(Uuid::new_v4())
}

async fn upload_script_handler(data: UploadScriptData) -> Result<Uuid,ApiError>{
    Ok(Uuid::new_v4())
}
