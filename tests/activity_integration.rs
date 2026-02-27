use axum::{
    body::Body,
    http::{self, StatusCode},
};
use iot_activity::{AppState, create_router, repositories::Database};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

#[sqlx::test]
async fn test_record_batch_activities_success(pool: PgPool) {
    let db = Database::new(pool.clone(), pool.clone());
    let state = AppState { db };
    let app = create_router(state);

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Fail to run migrations");

    sqlx::query!(
        "INSERT INTO devices (id, device_name, api_key_hash) VALUES ($1, $2, $3)",
        uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        "Test-Device",
        "valid-key"
    )
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .oneshot(
            http::Request::builder()
                .method(http::Method::POST)
                .uri("/activities")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header("x-api-key", "valid-key")
                .body(Body::from(
                    json!([
                        {
                            "device_id": "550e8400-e29b-41d4-a716-446655440000",
                            "activity_type": "test_type",
                            "payload": {"temp" : 25}
                        }
                    ])
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

#[sqlx::test]
async fn test_record_batch_activities_devices_not_found(pool: PgPool) {
    let db = Database::new(pool.clone(), pool.clone());
    let state = AppState { db };
    let app = create_router(state);

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Fail to run migrations");

    sqlx::query!(
        "INSERT INTO devices (id, device_name, api_key_hash) VALUES ($1, $2, $3)",
        uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        "Test-Device",
        "valid-key"
    )
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .oneshot(
            http::Request::builder()
                .method(http::Method::POST)
                .uri("/activities")
                .header(http::header::CONTENT_TYPE, "application/json")
                .header("x-api-key", "valid-key")
                .body(Body::from(
                    json!([
                        {
                            "device_id": "550e8400-e29b-41d4-a716-446655449993",
                            "activity_type": "test_type",
                            "payload": {"temp" : 25}
                        }
                    ])
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[sqlx::test]
async fn test_record_activities_unauthrized(pool: PgPool) {
    let db = Database::new(pool.clone(), pool.clone());
    let app = create_router(AppState { db });

    let response = app
        .oneshot(
            http::Request::builder()
                .method("POST")
                .uri("/activities")
                .header("x-api-key", "wrong-key")
                .header("content-type", "application/json")
                .body(Body::from(json!([]).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[sqlx::test]
async fn test_record_activities_validation_error(pool: PgPool) {
    let db = Database::new(pool.clone(), pool.clone());
    let app = create_router(AppState { db });

    sqlx::query!(
        "INSERT INTO devices (id, device_name, api_key_hash) VALUES ($1, $2, $3)",
        uuid::Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
        "Validator-Test-Device",
        "valid-key" // Key ตรงกับใน Header ด้านล่าง
    )
    .execute(&pool)
    .await
    .unwrap();

    let response = app
        .oneshot(
            http::Request::builder()
                .method("POST")
                .uri("/activities")
                .header("x-api-key", "valid-key")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!([
                        {
                            "device_id" : "550e8400-e29b-41d4-a716-446655440000",
                            "activity_type" : "",
                            "payload": {}
                        }
                    ])
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}
