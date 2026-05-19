use crate::{
    AppState,
    db::festivals::{
        delete_panelist, insert_new_festival, insert_new_festival_work, insert_new_panelist,
        update_festival_details, update_panelist_work,
    },
    errors::ApiError,
    shared::{
        auth::extractor::{Artist, EntityMemberOrAdmin, OwnedResourceOrAdmin},
        json_extractor::AppJson,
        works::upload_work,
    },
    types::{
        db::{
            festivals::{Festival, Panelist},
            sets::SetMember,
            work::WorkType,
        },
        requests::festivals::{CreateFestivalReq, UpdateFestivalPanlist, UpdateFestivalReq},
        response::ApiResponse,
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
use uuid::Uuid;

#[instrument(name="create_new_set", skip(state, user, data), fields(user_id = %user.profile_id, festival_name = %data.name))]
pub async fn create_new_set(
    State(state): State<Arc<AppState>>,
    Artist(user): Artist,
    AppJson(data): AppJson<CreateFestivalReq>,
) -> Result<ApiResponse, ApiError> {
    let festival = Festival {
        id: uuid::Uuid::new_v4(),
        name: data.name.to_string(),
        description: data.description.to_string(),
        set_id: data.set_id,
        organizer: user.profile_id,
        start_date: data.start_date,
        end_date: data.end_date,
        rules: data.rules.map(|r| r.to_string()),
        created_at: chrono::Utc::now(),
    };
    let mut txn = state.db_pool.begin().await?;
    let set_id = insert_new_festival(&mut txn, festival).await?;
    for panelist in data.panelists {
        let panelist = Panelist {
            festival_id: set_id,
            profile_id: panelist,
            work_id: None,
            created_at: chrono::Utc::now(),
        };
        tracing::info!("Inserting panelist: {}", panelist.profile_id);
        insert_new_panelist(&mut txn, panelist).await?;
    }
    txn.commit().await?;
    Ok(ApiResponse::OK)
}

#[instrument(name = "update festival details",skip(app,data),fields(festival_id = %resource_id))]
async fn update_festival_details_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Festival>,
    AppJson(data): AppJson<UpdateFestivalReq>,
) -> Result<ApiResponse, ApiError> {
    let res = update_festival_details(&app.db_pool, resource_id, data)
        .await?
        .ok_or(ApiError::NotFound)?;
    Ok(ApiResponse::FestivalDetailsUpdated(res))
}

#[instrument(name = "update festival panelists",skip(app,data),fields(festival_id = %resource_id, artist_id = %data.artist_id, is_insert= %data.insert))]
async fn update_panelists_handler(
    State(app): State<Arc<AppState>>,
    OwnedResourceOrAdmin { resource_id, .. }: OwnedResourceOrAdmin<Festival>,
    AppJson(data): AppJson<UpdateFestivalPanlist>,
) -> Result<ApiResponse, ApiError> {
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
        Ok(ApiResponse::PanelistAdded(res))
    } else {
        let res = delete_panelist(&app.db_pool, resource_id, data.artist_id)
            .await?
            .ok_or(ApiError::NotFound)?;
        Ok(ApiResponse::PanelistDeleted(res))
    }
}

#[instrument(name = "submit panelist work",skip(app,data),fields(festival_id = %festival_id, profile_id = %user_id))]
async fn submit_panelist_work_handler(
    State(app): State<Arc<AppState>>,
    Path((festival_id, work_type)): Path<(Uuid, WorkType)>,
    EntityMemberOrAdmin {
        user_id, entity, ..
    }: EntityMemberOrAdmin<Panelist>,
    data: Bytes,
) -> Result<ApiResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let work_id = upload_work(data, &mut txn, user_id, work_type).await?;
    let res = update_panelist_work(&mut txn, entity.festival_id, user_id, work_id)
        .await?
        .ok_or(ApiError::NotFound)?;

    txn.commit().await?;
    Ok(ApiResponse::WorkCreated(res))
}

#[instrument(name = "submit member work",skip(app,data),fields(set_id = %entity_id, profile_id = %entity.profile_id))]
async fn submit_memeber_work_handler(
    State(app): State<Arc<AppState>>,
    EntityMemberOrAdmin {
        entity, entity_id, ..
    }: EntityMemberOrAdmin<SetMember>,
    Path((_entity_id, work_type)): Path<(Uuid, WorkType)>,
    data: Bytes,
) -> Result<ApiResponse, ApiError> {
    let mut txn = app.db_pool.begin().await?;
    let work_id = upload_work(data, &mut txn, entity.profile_id, work_type).await?;
    let res = insert_new_festival_work(&mut txn, entity_id, work_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    txn.commit().await?;
    Ok(ApiResponse::WorkCreated(res))
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/new", post(create_new_set))
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
