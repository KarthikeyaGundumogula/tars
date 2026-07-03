use std::sync::Arc;

use axum::{
    Router,
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
        sets::{delete_set_member, insert_new_set, insert_set_member, update_set},
    },
    errors::ApiError,
    services::{
        auth_service::extractor::{Artist, EntityMemberOrAdmin, OwnedResourceOrAdmin, Resource},
        json_extractor::AppJson,
    },
    models::{
        db::{
            festivals::{Festival, Panelist},
            sets::{Set, SetMember, SetRole},
        },
        requests::{
            festivals::CreateFestivalReq,
            sets::{CreateSetReq, JoinSetRequest, UpdateSetReq},
        },
        response::{FestivalResponse, SetResponse},
    },
};

#[instrument(name = "create_new_set", skip(app, user, data), fields(curator= %user.handle, set_name = %data.name))]
pub async fn create_new_set_handler(
    State(app): State<Arc<AppState>>,
    Artist(user): Artist,
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

#[instrument(name="create_new_festival", skip(state, data), fields(user_id = %user.profile_id, festival_name = %data.name))]
pub async fn create_new_festival_handler(
    State(state): State<Arc<AppState>>,
    axum::extract::Path(resource_id): axum::extract::Path<Uuid>,
    Artist(user): Artist,
    AppJson(data): AppJson<CreateFestivalReq>,
) -> Result<FestivalResponse, ApiError> {
    let (owner_id, _) = Set::fetch_by_id(&state.db_pool, resource_id)
        .await?
        .ok_or(ApiError::NotFound)?;
    if owner_id != user.profile_id {
        return Err(ApiError::Unauthorized(
            "Only the set owner can create a festival".to_string(),
        ));
    }

    let festival = Festival {
        id: uuid::Uuid::new_v4(),
        name: data.name.to_string(),
        description: data.description.to_string(),
        set_id: resource_id,
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
}
