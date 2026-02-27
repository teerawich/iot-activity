use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateActivity {
    pub device_id: Uuid,

    #[validate(length(min = 1, max = 50))]
    pub activity_type: String,

    pub payload: serde_json::Value,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Activity {
    pub id: Uuid,
    pub device_id: Uuid,
    pub activity_type: String,
    pub payload: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

pub struct BatchActivityResponse {
    pub processed: usize,
    pub ids: Vec<uuid::Uuid>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_activity_alidation() {
        let activity = CreateActivity {
            device_id: Uuid::new_v4(),
            activity_type: "".to_string(),
            payload: serde_json::json!({}),
        };

        assert!(activity.validate().is_err());
    }

    #[test]
    fn test_activity_type_too_long() {
        let long_string = "a".repeat(51);
        let activity = CreateActivity {
            device_id: Uuid::new_v4(),
            activity_type: long_string,
            payload: serde_json::json!({}),
        };

        assert!(
            activity.validate().is_err(),
            "Should be error because activity_type exceed lenght (50 Characters)"
        );
    }
}
