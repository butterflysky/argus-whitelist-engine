use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct WhitelistChange {
    pub id: i32,
    pub player_id: i32,
    pub changed_by: i64,
    pub old_status: String, // Handle as enum in code
    pub new_status: String, // Handle as enum in code
    pub changed_at: NaiveDateTime,
    pub reason: Option<String>,
}
