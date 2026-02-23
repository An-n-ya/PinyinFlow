use std::{fs, path::PathBuf};

use anyhow::Result;
use log::info;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::Manager;

use crate::{
    domain::{preferences::UserPreferences, user_profiles::UserProfiles},
    utils::is_dev,
};

mod preferences_queries;
mod user_profiles_queries;

pub const DB_FILENAME: &str = "voicerelay.db";
pub const DB_CONNECTION: &str = "sqlite:voicerelay.db";
const USER_SCHEMA: &str = include_str!("migrations/000_schema.sql");
const DEV_USER_ID: &str = "00000000-0000-0000-0000-000000000000";
const DEV_USER_NAME: &str = "dev";
const DEV_USER_EMAIL: &str = "dev@example.com";

pub struct DataBase {
    pool: SqlitePool,
}

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
                ret.insert_user_profiles(&UserProfiles {
                    user_id: DEV_USER_ID.into(),
                    user_name: DEV_USER_NAME.into(),
                    email: DEV_USER_EMAIL.into(),
                })
                .await
                .unwrap_or_else(|e| log::warn!("Failed to create dev user: {}", e));
                ret.insert_user_preferences(&UserPreferences {
                    user_id: DEV_USER_ID.into(),
                    is_sidebar_open: true,
                    enable_complete_input: true,
                })
                .await
                .unwrap_or_else(|e| log::warn!("Failed to create dev preferences: {}", e));
            });
        }

        Ok(ret)
    }
}
