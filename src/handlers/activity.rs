use axum::{
    extract::State, 
    http::StatusCode, 
    response::IntoResponse, 
    Json
};
use crate::errors::AppError;
use crate::models::{Activity, CreateActivity};
use crate::repositories::activity_repo;
use crate::AppState;
use validator::Validate;

pub async fn record_batch_activities (
    State(state): State<AppState>,
    Json(payload): Json<Vec<CreateActivity>>,
) -> Result<impl IntoResponse, AppError> {

    if payload.len() > 100 {
        return Err(AppError::BadRequest("Batch size exceeds limit (max 100)".into()))
    }

    for activity in &payload {
        activity.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    }

    let save_data = activity_repo::insert_batch_activity(
        state.db.writer(),
        payload
    ).await?;

    Ok((StatusCode::CREATED, Json(save_data)))
}