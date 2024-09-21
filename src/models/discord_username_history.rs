use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct DiscordUsernameHistory {
    pub id: i32,
    pub player_id: i32,
    pub discord_username: String,
    pub discord_nickname: Option<String>,
    pub changed_at: NaiveDateTime,
}
