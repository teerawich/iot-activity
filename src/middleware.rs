use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
    http::HeaderMap
};
use crate::errors::AppError;
use crate::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let api_key = headers
                .get("x-api-key")
                .and_then(|value| value.to_str().ok())
                .ok_or(AppError::Unauthorized)?;
    
    let device_exists = sqlx::query!(
        "SELECT id FROM devices WHERE api_key_hash = $1 LIMIT 1",
        api_key
    )
    .fetch_optional(state.db.reader())
    .await?;

    if device_exists.is_some() {
        Ok(next.run(request).await)
    } else {
        Err(AppError::Unauthorized)
    }
}