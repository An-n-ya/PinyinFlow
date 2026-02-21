use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserPreferences {
    pub user_id: String,
    pub is_sidebar_open: bool,
}

impl From<SqliteRow> for UserPreferences {
    fn from(row: SqliteRow) -> Self {
        UserPreferences {
            user_id: row.get("user_id"),
            is_sidebar_open: row.get("is_sidebar_open"),
        }
    }
}
