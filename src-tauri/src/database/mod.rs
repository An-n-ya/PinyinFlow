use std::{fs, path::PathBuf};

use anyhow::Result;
use log::info;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use tauri::Manager;

mod preferences_queries;

pub const DB_FILENAME: &str = "voicerelay.db";
pub const DB_CONNECTION: &str = "sqlite:voicerelay.db";
const USER_PREFERENCES: &str = include_str!("migrations/000_schema.sql");

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
        description: "create user preferences table",
        sql: USER_PREFERENCES,
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
        Ok(DataBase { pool })
    }
}
