use crate::{
    AppState,
    db::mutations::festivals::{
        delete_panelist, get_festival_details, insert_new_festival_work, insert_new_panelist, update_festival_details, update_panelist_work
    },
    errors::ApiError,
    services::{
        auth_service::extractor::{EntityMemberOrAdmin, OwnedResourceOrAdmin},
        json_extractor::AppJson,
        upload_service::upload_work,
    },
    models::{
        db::{
            festivals::{Festival, Panelist},
            sets::FestivalMember,
            work::WorkTypeParam,
        },
        requests::festivals::{UpdateFestivalPanlist, UpdateFestivalReq},
        response::{FestivalResponse, WorkResponse},
    },
};
use axum::{
    Router,
    body::Bytes,
    extract::{Path, State},
    routing::post,
};
use std::sync::Arc;
use tracing::instrument;

#[instrument(name = "update festival details",skip(app,data),fields(festival_id = %resource_id))]
async fn update_festival_details_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Festival>,
    AppJson(data): AppJson<UpdateFestivalReq>,
) -> Result<FestivalResponse, ApiError> {
    let res = update_festival_details(&app.db_pool, resource_id, data)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(FestivalResponse::FestivalDetailsUpdated(res))
}

#[instrument(name = "update festival panelists",skip(app,data),fields(festival_id = %resource_id, artist_id = %data.artist_id, is_insert= %data.insert))]
async fn update_panelists_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Festival>,
    AppJson(data): AppJson<UpdateFestivalPanlist>,
) -> Result<FestivalResponse, ApiError> {
    if data.insert {
        let panelist = Panelist {
            festival_id: resource_id,
            profile_id: data.artist_id,
            work_id: None,
            created_at: chrono::Utc::now(),
        };
        let mut txn = app.db_pool.begin().await?;
        let res = insert_new_panelist(&mut txn, panelist).await?;
        txn.commit().await?;
        Ok(FestivalResponse::PanelistAdded(res))
    } else {
        let res = delete_panelist(&app.db_pool, resource_id, data.artist_id)
            .await?
            .ok_or(ApiError::NotFound)?;
        Ok(FestivalResponse::PanelistDeleted(res))
    }
}

#[instrument(name = "submit panelist work",skip(app,data),fields(festival_id = %entity.festival_id, profile_id = %user_id))]
async fn submit_panelist_work_handler(
    State(app): State<Arc<AppState>>,
    EntityMemberOrAdmin {
        user_id, entity, ..
    }: EntityMemberOrAdmin<Panelist>,
    Path(WorkTypeParam { work_type }): Path<WorkTypeParam>,
    data: Bytes,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let festival_details = get_festival_details(&mut txn, entity.festival_id).await?.ok_or(ApiError::NotFound)?;
    if !(festival_details.start_date <= chrono::Utc::now() && festival_details.end_date >= chrono::Utc::now()) {
        return Err(ApiError::BadRequest("Festival is not active".to_string()));
    }
    let work_id = upload_work(data, &mut txn, user_id, work_type).await?;
    let res = update_panelist_work(&mut txn, entity.festival_id, user_id, work_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    txn.commit().await?;
    Ok(WorkResponse::WorkCreated(res))
}

#[instrument(name = "submit member work",skip(app,data),fields(set_id = %entity_id, profile_id = %entity.0.profile_id))]
async fn submit_memeber_work_handler(
    State(app): State<Arc<AppState>>,
    EntityMemberOrAdmin {
        entity, entity_id, ..
    }: EntityMemberOrAdmin<FestivalMember>,
    Path(WorkTypeParam { work_type }): Path<WorkTypeParam>,
    data: Bytes,
) -> Result<WorkResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let festival_details = get_festival_details(&mut txn, entity_id).await?.ok_or(ApiError::NotFound)?;
    if !(festival_details.start_date <= chrono::Utc::now() && festival_details.end_date >= chrono::Utc::now()) {
        return Err(ApiError::BadRequest("Festival is not active".to_string()));
    }
    let work_id = upload_work(data, &mut txn, entity.0.profile_id, work_type).await?;
    let res = insert_new_festival_work(&mut txn, entity_id, work_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    txn.commit().await?;
    Ok(WorkResponse::WorkCreated(res))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/{resource_id}/update",
            post(update_festival_details_handler),
        )
        .route(
            "/{resource_id}/panelists/update",
            post(update_panelists_handler),
        )
        .route(
            "/{entity_id}/panelist/new/{work_type}",
            post(submit_panelist_work_handler),
        )
        .route(
            "/{entity_id}/member/new/{work_type}",
            post(submit_memeber_work_handler),
        )
}
