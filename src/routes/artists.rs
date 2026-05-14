use axum::{Router, routing::post};

use crate::{errors::ApiError, types::response::ApiResponse};


async fn update_stage_details_handler() -> Result<ApiResponse,ApiError>{
    todo!()
}

async fn follow_artist_handler() -> Result<ApiResponse,ApiError>{
    todo!()
}

async fn unfollow_artist_handler() -> Result<ApiResponse,ApiError>{
    todo!()
}

async fn add_to_favorite_profiles_handler() -> Result<ApiResponse,ApiError>{
    todo!()
}

async fn remove_from_favorite_profiles_handler() -> Result<ApiResponse,ApiError>{
    todo!()
}

async fn update_profile_presence_handler() -> Result<ApiResponse,ApiError>{
    todo!()
}

pub fn router() -> Router {
    Router::new().route("/update_stage", post(update_stage_details_handler))
        .route("follow/{artitst_id}", post(follow_artist_handler))
        .route("unfollow/{artitst_id}", post(unfollow_artist_handler))
        .route("/favorite/{profile_id}", post(add_to_favorite_profiles_handler))
        .route("/unfavorite/{profile_id}", post(remove_from_favorite_profiles_handler))
        .route("/update_profile_presence", post(update_profile_presence_handler))
}
