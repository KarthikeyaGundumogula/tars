use axum::{Json, http::StatusCode, response::IntoResponse};
use axum_extra::extract::CookieJar;
use uuid::Uuid;

use crate::types::db::sets::SetRole;

pub enum ApiResponse {
    OK,
    WorkCreated(Uuid),
    OriginalCreated(Uuid),
    SetCreated(Uuid),
    UpdatedSet(Uuid),
    JoinedSet(SetRole),
    ProfileAuthenticated(CookieJar),
    PasswordUpdated(Uuid),
    LedgerEntryLogged(Uuid),
    ProfileUpdated(Uuid),
    FollowedArtist(bool),
    UnfollowedArtist(bool),
    FavoritedArtist(bool),
    FavoriteArtistRemoved(bool),
    FestivalDetailsUpdated(Uuid),
    PanelistAdded(Uuid),
    PanelistDeleted(Uuid),
    LedgerEntryUpdated(Uuid),
    WorkTaggedTOLedgerEntry(Uuid,Uuid),
    LedgerEntryDeleted(Uuid),
    OriginalUpdated(Uuid),
    OriginalDeleted(Uuid),
    RoleDeleted(Uuid),
    RoleCreated(Uuid),
    RoleExists(Uuid), 
    OrignalReleaseCreated(Uuid),
    SetMemberDeleted(Uuid,Uuid),
    WorkUpdated(Uuid),
    AddedWorkLike(bool),
    RemovedWorkLike(bool),
    WorkDeleted(Uuid),
    AdminCreated(Uuid),
    AdminAuthenticated(CookieJar),
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::WorkCreated(id) => {
                (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalCreated(id) => {
                (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::SetCreated(id) => {
                (StatusCode::ACCEPTED, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::JoinedSet(role) => {
                (StatusCode::OK, Json(serde_json::json!({"role": role}))).into_response()
            }
            Self::ProfileAuthenticated(jar) => (
                StatusCode::OK,
                jar,
                Json(serde_json::json!({"message":"logged_in"})),
            )
                .into_response(),
            Self::PasswordUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::LedgerEntryLogged(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::ProfileUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::FollowedArtist(status) => {
                (StatusCode::OK, Json(serde_json::json!({"status": status}))).into_response()
            }
            Self::UnfollowedArtist(status) => {
                (StatusCode::OK, Json(serde_json::json!({"status": status}))).into_response()
            }
            Self::FavoritedArtist(status) => {
                (StatusCode::OK, Json(serde_json::json!({"status": status}))).into_response()
            }
            Self::FavoriteArtistRemoved(status) => {
                (StatusCode::OK, Json(serde_json::json!({"status": status}))).into_response()
            }
            Self::FestivalDetailsUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::PanelistAdded(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::PanelistDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::LedgerEntryUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::WorkTaggedTOLedgerEntry(work_id, ledger_entry_id) => {
                (StatusCode::OK, Json(serde_json::json!({"work_id": work_id, "ledger_entry_id": ledger_entry_id}))).into_response()
            }
            Self::LedgerEntryDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::RoleDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"profile_id": id}))).into_response()
            }
            Self::RoleCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"profile_id": id}))).into_response()
            }
            Self::RoleExists(id) => {
                (StatusCode::OK, Json(serde_json::json!({"profile_id": id}))).into_response()
            }
            Self::UpdatedSet(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OrignalReleaseCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"release_id": id}))).into_response()
            }
            Self::SetMemberDeleted(set_id, profile_id) => {
                (StatusCode::OK, Json(serde_json::json!({"set_id": set_id, "profile_id": profile_id}))).into_response()
            }
            Self::WorkUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AddedWorkLike(status) => {
                (StatusCode::OK, Json(serde_json::json!({"is_addded": status}))).into_response()
            }
            Self::RemovedWorkLike(status) => {
                (StatusCode::OK, Json(serde_json::json!({"Is_removed": status}))).into_response()
            }
            Self::WorkDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AdminCreated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AdminAuthenticated(jar) => {
                (StatusCode::OK, jar).into_response()
            }
        }
    }
}
