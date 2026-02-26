use crate::models::{Activity, CreateActivity};
use crate::errors::AppError;

pub async fn insert_batch_activity(
    pool: &sqlx::PgPool,
    activities: Vec<CreateActivity>
) -> Result<Vec<Activity>, AppError> {
    
    let mut device_ids = Vec::new();
    let mut types = Vec::new();
    let mut payloads = Vec::new();

    for act in activities {
        device_ids.push(act.device_id);
        types.push(act.activity_type);
        payloads.push(act.payload); 
    }

    let rows = sqlx::query_as::<_, Activity> (
        r#"
        INSERT INTO activities (device_id, activity_type, payload)
        SELECT * FROM UNNEST($1::uuid[], $2::text[], $3::jsonb[]) AS t(d_id, a_type, p_load)
        RETURNING id, device_id, activity_type, payload, created_at 
        "#
    )
    .bind(&device_ids)
    .bind(&types)
    .bind(&payloads)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}