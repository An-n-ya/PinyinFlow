mod commands;
mod device;

use crate::commands::{play, split, tone};
use crate::device::audio::AudioDevice;
use crate::device::frontend::FClient;
use crate::device::websocket::WsClient;
use chrono::Local;

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
        .setup(|app| {
            log::info!("setup started");
            let ws_client = WsClient::init("ws://localhost:8000/play")?;
            FClient::init(app.handle().clone());
            AudioDevice::init(app.handle().clone())?;
            AudioDevice::listen(&ws_client);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![split, tone, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
