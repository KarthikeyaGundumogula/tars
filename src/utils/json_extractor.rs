use axum::extract::{FromRequest, rejection::JsonRejection};
use serde::{Deserialize, de::DeserializeOwned};

use crate::errors::ApiError;
pub struct AppJson<T>(pub T);

impl<T, S> FromRequest<S> for AppJson<T>
where
    T: DeserializeOwned + Send + Sync,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let res = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|e: JsonRejection| ApiError::from(e))?;
        Ok(AppJson(res.0))
    }
}

impl<'a, T: Deserialize<'a>> AppJson<T>
{
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self, ApiError> {
        let res = serde_json::from_slice::<T>(bytes).map_err(|e| ApiError::from(e))?;
        Ok(AppJson(res))
    }
}
