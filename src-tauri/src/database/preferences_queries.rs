use crate::{database::DataBase, domain::preferences::UserPreferences};
use anyhow::Result;
use sqlx::Row;

impl DataBase {
    pub async fn fetch_user_preferences(&self, user_id: &str) -> Result<Option<UserPreferences>> {
        let row = sqlx::query("SELECT * FROM user_preferences WHERE user_id = $1 LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(UserPreferences::from(row))),
            None => Ok(None),
        }
    }

    pub async fn insert_user_preferences(&self, preferences: &UserPreferences) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO user_preferences (user_id, is_sidebar_open, enable_complete_input) VALUES ($1, $2, $3)",
        )
        .bind(&preferences.user_id)
        .bind(preferences.is_sidebar_open)
        .bind(preferences.enable_complete_input)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user_preferences(&self, preferences: &UserPreferences) -> Result<()> {
        sqlx::query("UPDATE user_preferences SET is_sidebar_open = $2, enable_complete_input = $3 WHERE user_id = $1")
            .bind(&preferences.user_id)
            .bind(preferences.is_sidebar_open)
            .bind(preferences.enable_complete_input)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_user_preferences(&self, user_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_preferences WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
