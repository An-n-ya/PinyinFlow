use std::{fs, path::PathBuf};

use anyhow::Result;
use log::info;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::Manager;

use crate::{
    domain::{preferences::UserPreferences, user_profiles::UserProfiles},
    utils::is_dev,
};

pub struct DataBase {
    pub pool: SqlitePool,
}

macro_rules! impl_crud {
    (
        $struct_type:ty,
        $table_name:expr,
        $suffix:ident, // 例如：user_profiles, user_preferences
        { $($field:ident),* } // 用于 insert/update 的字段（不包含 user_id）
    ) => {
        use paste::paste;
        use crate::database::DataBase;
        use anyhow::Result;
        paste! {
            impl DataBase {
                pub async fn [<fetch_ $suffix>](&self, user_id: &str) -> Result<Option<$struct_type>> {
                    let row = sqlx::query_as::<_, $struct_type>(
                        &format!("SELECT * FROM {} WHERE user_id = $1 LIMIT 1", $table_name)
                    )
                    .bind(user_id)
                    .fetch_optional(&self.pool)
                    .await?;
                    Ok(row)
                }

                pub async fn [<insert_ $suffix>](&self, item: &$struct_type) -> Result<()> {
                    let fields = vec!["user_id", $(stringify!($field)),*].join(", ");
                    let placeholders = (1..=(1 + [$(stringify!($field)),*].len()))
                        .map(|i| format!("${}", i))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let sql = format!("INSERT OR REPLACE INTO {} ({}) VALUES ({})", $table_name, fields, placeholders);

                    sqlx::query(&sql)
                        .bind(&item.user_id)
                        $(.bind(&item.$field))*
                        .execute(&self.pool)
                        .await?;
                    Ok(())
                }

                pub async fn [<update_ $suffix>](&self, item: &$struct_type) -> Result<()> {
                    let sets = [$(stringify!($field)),*]
                        .iter()
                        .enumerate()
                        .map(|(i, field)| format!("{} = ${}", field, i + 2))
                        .collect::<Vec<_>>()
                        .join(", ");
                    let sql = format!("UPDATE {} SET {} WHERE user_id = $1", $table_name, sets);

                    sqlx::query(&sql)
                        .bind(&item.user_id)
                        $(.bind(&item.$field))*
                        .execute(&self.pool)
                        .await?;
                    Ok(())
                }

                pub async fn [<delete_ $suffix>](&self, user_id: &str) -> Result<()> {
                    sqlx::query(&format!("DELETE FROM {} WHERE user_id = $1", $table_name))
                        .bind(user_id)
                        .execute(&self.pool)
                        .await?;
                    Ok(())
                }
            }
        }
    };
}
pub(crate) use impl_crud;

pub const DB_FILENAME: &str = "voicerelay.db";
pub const DB_CONNECTION: &str = "sqlite:voicerelay.db";
const USER_SCHEMA: &str = include_str!("migrations/000_schema.sql");
const DEV_USER_ID: &str = "00000000-0000-0000-0000-000000000000";

fn database_path(app: &tauri::AppHandle) -> Result<PathBuf> {
    let mut path = app.path().app_config_dir()?;
    fs::create_dir_all(&path)?;
    path.push(DB_FILENAME);
    if !fs::exists(&path)? {
        fs::write(&path, "")?;
    }
    info!("Database path: {:?}", &path);
    Ok(path)
}
fn database_url(app: &tauri::AppHandle) -> Result<String> {
    let path = database_path(app)?;
    let path_str = path.to_str().unwrap();
    Ok(format!("sqlite:{}", path_str))
}

pub fn migrations() -> Vec<tauri_plugin_sql::Migration> {
    vec![tauri_plugin_sql::Migration {
        version: 1,
        description: "create user preferences and user profiles table",
        sql: USER_SCHEMA,
        kind: tauri_plugin_sql::MigrationKind::Up,
    }]
}
impl DataBase {
    pub fn init(app: &tauri::AppHandle) -> Result<DataBase> {
        let db_url = { crate::database::database_url(app)? };
        let pool = tauri::async_runtime::block_on(async {
            SqlitePoolOptions::new()
                .max_connections(5)
                .connect(&db_url)
                .await
        })?;
        info!("Database initialized");

        let ret = DataBase { pool };

        // create dev user
        if is_dev() {
            tauri::async_runtime::block_on(async {
                if let Some(_) = ret.fetch_user_profiles(DEV_USER_ID).await.unwrap() {
                    // dev user already exists
                    return;
                }
                ret.insert_user_profiles(&UserProfiles::dev(DEV_USER_ID))
                    .await
                    .unwrap_or_else(|e| log::warn!("Failed to create dev user: {}", e));
                ret.insert_user_preferences(&UserPreferences::dev(DEV_USER_ID))
                    .await
                    .unwrap_or_else(|e| log::warn!("Failed to create dev preferences: {}", e));
            });
        }

        Ok(ret)
    }
}
