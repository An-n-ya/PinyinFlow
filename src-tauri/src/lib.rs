mod commands;
mod database;
mod device;
mod domain;
mod service;
mod utils;

use std::path::Path;

use tokio::sync::Mutex;

use crate::commands::{
    complete_message, fetch_user_preferences, fetch_user_profiles, play, proofread, split, tone,
    update_user_preferences, update_user_profiles,
};
use crate::database::DataBase;
use crate::device::audio::AudioDevice;
use crate::device::frontend::FClient;
use crate::device::tts::WsClient;
use crate::service::llm::service::LlmService;
use chrono::Local;
use tauri::Manager;

fn trim_webview_target(target: &str) -> &str {
    if target.starts_with("webview:") {
        return target.split("@").nth(0).unwrap();
    }
    target
}

fn log_time() -> String {
    let now = Local::now();
    format!("{}", now.format("[%Y-%m-%d][%H:%M:%S]"))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        path: std::path::PathBuf::from("../logs"),
                        file_name: Some("tauri_log.log".to_owned()),
                    },
                ))
                .level(tauri_plugin_log::log::LevelFilter::Debug)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "{}[{}][flow][{}] {}",
                        log_time(),
                        record.level(),
                        trim_webview_target(record.target()),
                        message
                    ));
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::new()
                .add_migrations(
                    crate::database::DB_CONNECTION,
                    crate::database::migrations(),
                )
                .build(),
        )
        .setup(|app| {
            log::info!("setup started");
            // FIXME: ensure env file local path
            dotenvy::from_path(Path::new("../.env.local")).unwrap();
            let ws_client = WsClient::init("ws://localhost:8000/play")?;
            FClient::init(app.handle().clone());
            AudioDevice::init(app.handle().clone())?;
            AudioDevice::listen(&ws_client);
            app.manage(Mutex::new(LlmService::init()));
            let db = DataBase::init(app.handle())?;
            app.manage(db);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            split,
            tone,
            play,
            proofread,
            complete_message,
            fetch_user_preferences,
            fetch_user_profiles,
            update_user_preferences,
            update_user_profiles
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
