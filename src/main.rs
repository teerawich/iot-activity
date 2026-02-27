use iot_activity::{AppState, create_router, repositories::Database};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let write_db_url = std::env::var("WRITE_DB_URL").expect("WRITE_DB_URL must be set");
    let reader_db_url = std::env::var("READ_DB_URL").expect("READ_DB_URL must be set");

    let write_pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&write_db_url)
        .await?;

    let read_pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .connect(&reader_db_url)
        .await?;

    let db = Database::new(write_pool, read_pool);
    let state = AppState { db };
    let app = create_router(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server start at http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
