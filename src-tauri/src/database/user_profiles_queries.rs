use crate::database::DataBase;
use crate::domain::user_profiles::UserProfiles;
use anyhow::Result;

impl DataBase {
    pub async fn fetch_user_profiles(&self, user_id: &str) -> Result<Option<UserProfiles>> {
        let row = sqlx::query("SELECT * FROM user_profiles WHERE user_id = $1 LIMIT 1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(UserProfiles::from(row))),
            None => Ok(None),
        }
    }

    pub async fn insert_user_profiles(&self, profile: &UserProfiles) -> Result<()> {
        sqlx::query(
            "INSERT OR REPLACE INTO user_profiles (user_id, user_name, email) VALUES ($1, $2, $3)",
        )
        .bind(&profile.user_id)
        .bind(&profile.user_name)
        .bind(&profile.email)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn update_user_profiles(&self, profile: &UserProfiles) -> Result<()> {
        sqlx::query("UPDATE user_profiles SET user_name = $2, email = $3 WHERE user_id = $1")
            .bind(&profile.user_id)
            .bind(&profile.user_name)
            .bind(&profile.email)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_user_profiles(&self, user_id: &str) -> Result<()> {
        sqlx::query("DELETE FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
