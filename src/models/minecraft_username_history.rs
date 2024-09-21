use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct MinecraftUsernameHistory {
    pub id: i32,
    pub player_id: i32,
    pub minecraft_username: String,
    pub changed_at: NaiveDateTime,
}
