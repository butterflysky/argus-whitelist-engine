use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlRow;
use sqlx::{FromRow, Row};
use strum::{Display, EnumString};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum PlayerStatus {
    Applied,
    Whitelisted,
    Tempbanned,
    Permabanned,
    Removed,
}

impl Default for PlayerStatus {
    fn default() -> Self {
        PlayerStatus::Applied
    }
}

impl PlayerStatus {
    pub fn from_str(status: &str) -> Option<PlayerStatus> {
        match status {
            "applied" => Some(PlayerStatus::Applied),
            "whitelisted" => Some(PlayerStatus::Whitelisted),
            "tempbanned" => Some(PlayerStatus::Tempbanned),
            "permabanned" => Some(PlayerStatus::Permabanned),
            "removed" => Some(PlayerStatus::Removed),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            PlayerStatus::Applied => "applied",
            PlayerStatus::Whitelisted => "whitelisted",
            PlayerStatus::Tempbanned => "tempbanned",
            PlayerStatus::Permabanned => "permabanned",
            PlayerStatus::Removed => "removed",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: i32,
    pub discord_user_id: i64,
    pub discord_username: String,
    pub discord_nickname: Option<String>,
    pub minecraft_uuid: String,
    pub minecraft_username: String,
    pub is_patron: bool,
    #[serde(skip_serializing)]
    pub status: String, // Internal field for database interaction
    #[serde(skip)]
    pub status_enum: PlayerStatus, // Enum used in code
    pub application_timestamp: Option<NaiveDateTime>,
    pub status_changed_at: Option<NaiveDateTime>,
    pub last_modified_timestamp: NaiveDateTime,
}

impl<'r> FromRow<'r, MySqlRow> for Player {
    fn from_row(row: &MySqlRow) -> Result<Self, sqlx::Error> {
        let status_str: String = row.try_get("status")?;
        let status_enum =
            PlayerStatus::from_str(&status_str).ok_or_else(|| sqlx::Error::ColumnDecode {
                index: "status".into(),
                source: Box::new(std::fmt::Error),
            })?;

        Ok(Player {
            id: row.try_get("id")?,
            discord_user_id: row.try_get("discord_user_id")?,
            discord_username: row.try_get("discord_username")?,
            discord_nickname: row.try_get("discord_nickname")?,
            minecraft_uuid: row.try_get("minecraft_uuid")?,
            minecraft_username: row.try_get("minecraft_username")?,
            is_patron: row.try_get("is_patron")?,
            status: status_str,
            status_enum,
            application_timestamp: row.try_get("application_timestamp")?,
            status_changed_at: row.try_get("status_changed_at")?,
            last_modified_timestamp: row.try_get("last_modified_timestamp")?,
        })
    }
}
