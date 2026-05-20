use axum::{Json, http::StatusCode, response::IntoResponse};
use uuid::Uuid;

/// Ledger-related API responses
#[derive(Debug)]
pub enum LedgerResponse {
    LedgerEntryLogged(Uuid),
    LedgerEntryUpdated(Uuid),
    WorkTaggedToLedgerEntry(Uuid, Uuid),
    LedgerEntryDeleted(Uuid),
}

impl IntoResponse for LedgerResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::LedgerEntryLogged(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::LedgerEntryUpdated(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
            Self::WorkTaggedToLedgerEntry(work_id, ledger_entry_id) => (
                StatusCode::OK,
                Json(serde_json::json!({"work_id": work_id, "ledger_entry_id": ledger_entry_id})),
            )
                .into_response(),
            Self::LedgerEntryDeleted(id) => {
                (StatusCode::OK, Json(serde_json::json!({"id": id}))).into_response()
            }
        }
    }
}
