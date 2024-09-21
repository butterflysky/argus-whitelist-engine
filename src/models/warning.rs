use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Warning {
    pub id: i32,
    pub player_id: i32,
    pub warned_by: i64,
    pub warning_text: String,
    pub ticket_id: Option<String>,
    pub created_at: NaiveDateTime,
}
