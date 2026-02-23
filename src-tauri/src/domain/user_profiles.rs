use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserProfiles {
    pub user_id: String,
    pub user_name: String,
    pub email: String,
}

impl From<SqliteRow> for UserProfiles {
    fn from(row: SqliteRow) -> Self {
        UserProfiles {
            user_id: row.get("user_id"),
            user_name: row.get("user_name"),
            email: row.get("email"),
        }
    }
}
