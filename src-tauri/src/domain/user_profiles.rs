use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct UserProfiles {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
}
