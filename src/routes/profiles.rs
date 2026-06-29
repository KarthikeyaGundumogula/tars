use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, State},
    routing::get,
};

use crate::{
    AppState, db::queries::profile_queries::get_profile_details_by_username, errors::ApiError,
    types::response::ArtistResponse,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/get_profile_details/{user_name}", get(get_profile_details))
}

async fn get_profile_details(
    State(app): State<Arc<AppState>>,
    Path(user_name): Path<String>,
) -> Result<ArtistResponse, ApiError> {
    Ok(ArtistResponse::ArtistStage(
        get_profile_details_by_username(&app.db_pool, &user_name).await?,
    ))
}
