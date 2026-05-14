use crate::{
    AppState,
    db::festivals::{insert_new_festival, insert_new_panelist},
    errors::ApiError,
    types::{
        db::festivals::{Festival, Panelist}, requests::festivals::CreateFestivalReq, response::ApiResponse,
    },
    utils::{auth::extractor::Artist, json_extractor::AppJson},
};
use axum::{Router, extract::State, routing::post};
use tracing::instrument;
use std::sync::Arc;

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
        rules: data.rules.to_string(),
        created_at: chrono::Utc::now(),
    };
    let mut txn = state.db_pool.begin().await?;
    let set_id = insert_new_festival(&mut txn, festival).await?;
    for panelist in data.panelists {
      let panelist = Panelist{
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

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/new", post(create_new_set))
}
