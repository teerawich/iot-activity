pub mod errors;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod repositories;

use axum::{Router, middleware as axum_middleware, routing::post};
use repositories::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/activities",
            post(handlers::activity::record_batch_activities),
        )
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            middleware::auth_middleware,
        ))
        .with_state(state)
}
