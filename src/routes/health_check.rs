use crate::errors::ApiError;

pub async fn health_check_handler() -> Result<(), ApiError> {
    Ok(())
}
