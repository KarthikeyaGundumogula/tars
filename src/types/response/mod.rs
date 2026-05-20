// Entity-specific response modules
pub mod admin;
pub mod festival;
pub mod ledger;
pub mod original;
pub mod profile;
pub mod set;
pub mod work;

// Re-export entity-specific response types for convenience
pub use admin::AdminResponse;
pub use festival::FestivalResponse;
pub use ledger::LedgerResponse;
pub use original::OriginalResponse;
pub use profile::ProfileResponse;
pub use set::SetResponse;
pub use work::WorkResponse;

use axum::{http::StatusCode, response::IntoResponse};

/// Generic API response for simple success cases
#[derive(Debug)]
pub enum ApiResponse {
    OK,
}

impl IntoResponse for ApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
        }
    }
}

/// Legacy ApiResponse enum - DEPRECATED
/// Use entity-specific response types instead (ProfileResponse, WorkResponse, etc.)
#[deprecated(note = "Use entity-specific response types instead")]
#[derive(Debug)]
pub enum LegacyApiResponse {
    OK,
    WorkCreated(uuid::Uuid),
    OriginalCreated(uuid::Uuid),
    SetCreated(uuid::Uuid),
    UpdatedSet(uuid::Uuid),
    JoinedSet(crate::types::db::sets::SetRole),
    ProfileAuthenticated(axum_extra::extract::CookieJar),
    PasswordUpdated(uuid::Uuid),
    LedgerEntryLogged(uuid::Uuid),
    ProfileUpdated(uuid::Uuid),
    FollowedArtist(bool),
    UnfollowedArtist(bool),
    FavoritedArtist(bool),
    FavoriteArtistRemoved(bool),
    FestivalDetailsUpdated(uuid::Uuid),
    PanelistAdded(uuid::Uuid),
    PanelistDeleted(uuid::Uuid),
    LedgerEntryUpdated(uuid::Uuid),
    WorkTaggedTOLedgerEntry(uuid::Uuid, uuid::Uuid),
    LedgerEntryDeleted(uuid::Uuid),
    OriginalUpdated(uuid::Uuid),
    OriginalDeleted(uuid::Uuid),
    RoleDeleted(uuid::Uuid),
    RoleCreated(uuid::Uuid),
    RoleExists(uuid::Uuid),
    OrignalReleaseCreated(uuid::Uuid),
    SetMemberDeleted(uuid::Uuid, uuid::Uuid),
    WorkUpdated(uuid::Uuid),
    AddedWorkLike(bool),
    RemovedWorkLike(bool),
    WorkDeleted(uuid::Uuid),
    AdminCreated(uuid::Uuid),
    AdminAuthenticated(axum_extra::extract::CookieJar),
}

#[allow(deprecated)]
impl IntoResponse for LegacyApiResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::WorkCreated(id) => (
                StatusCode::ACCEPTED,
                axum::Json(serde_json::json!({"id": id})),
            )
                .into_response(),
            Self::OriginalCreated(id) => (
                StatusCode::ACCEPTED,
                axum::Json(serde_json::json!({"id": id})),
            )
                .into_response(),
            Self::SetCreated(id) => (
                StatusCode::ACCEPTED,
                axum::Json(serde_json::json!({"id": id})),
            )
                .into_response(),
            Self::JoinedSet(role) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"role": role})),
            )
                .into_response(),
            Self::ProfileAuthenticated(jar) => (
                StatusCode::OK,
                jar,
                axum::Json(serde_json::json!({"message":"logged_in"})),
            )
                .into_response(),
            Self::PasswordUpdated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::LedgerEntryLogged(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::ProfileUpdated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::FollowedArtist(status) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"status": status})),
            )
                .into_response(),
            Self::UnfollowedArtist(status) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"status": status})),
            )
                .into_response(),
            Self::FavoritedArtist(status) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"status": status})),
            )
                .into_response(),
            Self::FavoriteArtistRemoved(status) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"status": status})),
            )
                .into_response(),
            Self::FestivalDetailsUpdated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::PanelistAdded(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::PanelistDeleted(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::LedgerEntryUpdated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::WorkTaggedTOLedgerEntry(work_id, ledger_entry_id) => (
                StatusCode::OK,
                axum::Json(
                    serde_json::json!({"work_id": work_id, "ledger_entry_id": ledger_entry_id}),
                ),
            )
                .into_response(),
            Self::LedgerEntryDeleted(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalUpdated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OriginalDeleted(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::RoleDeleted(id) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"profile_id": id})),
            )
                .into_response(),
            Self::RoleCreated(id) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"profile_id": id})),
            )
                .into_response(),
            Self::RoleExists(id) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"profile_id": id})),
            )
                .into_response(),
            Self::UpdatedSet(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::OrignalReleaseCreated(id) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"release_id": id})),
            )
                .into_response(),
            Self::SetMemberDeleted(set_id, profile_id) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"set_id": set_id, "profile_id": profile_id})),
            )
                .into_response(),
            Self::WorkUpdated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AddedWorkLike(status) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"is_addded": status})),
            )
                .into_response(),
            Self::RemovedWorkLike(status) => (
                StatusCode::OK,
                axum::Json(serde_json::json!({"Is_removed": status})),
            )
                .into_response(),
            Self::WorkDeleted(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AdminCreated(id) => {
                (StatusCode::OK, axum::Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::AdminAuthenticated(jar) => (StatusCode::OK, jar).into_response(),
        }
    }
}
