mod device;

use std::time::Instant;

use chrono::{DateTime, Local, Utc};
use reqwest_websocket::Message;
use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};
use anyhow_tauri::TAResult;
use tauri::AppHandle;
use crate::device::{audio::AudioDevice, websocket::WsClient };

#[derive(Deserialize, Serialize, Debug, Clone)]
struct PlayResond {
    data: Vec<u8>,
    id: u32
}
#[derive(Serialize, Debug)]
struct PlayRequest {
    input: String,
    id: usize
}
#[derive(Deserialize, Serialize, Debug)]
struct PinyinRespond {
    pinyin: String,
    py_styled: String,
    tone: String,
}
#[tauri::command]
fn split(input: &str) -> String {
    log::info!("split {input}");
    dollop::split(input)
}
#[tauri::command]
async fn tone(input: &str) -> Result<PinyinRespond, String> {
    log::info!("tone {input}");
    let client = reqwest::Client::new();

    let req_body = PlayRequest {
        input: input.to_string(),
        id: 1
    };

    let res = client
        .post("http://localhost:8000/tone")
        .json(&req_body)
        .send()
        .await
        .expect("result")
        .text()
        .await
        .unwrap();

    log::info!("tone {res}");
    let v: PinyinRespond = serde_json::from_str(&res).unwrap();

    Ok(v)
}

#[tauri::command]
async fn play(id: usize, input: String) -> TAResult<()> {
    WsClient::handle_play(PlayRequest{id, input})?;
    return Ok(());
}

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
                    tauri_plugin_log::TargetKind::Folder { path: std::path::PathBuf::from("../logs"), file_name: Some("tauri_log.log".to_owned()) }
                ))
                .level(tauri_plugin_log::log::LevelFilter::Debug)
                .timezone_strategy(tauri_plugin_log::TimezoneStrategy::UseLocal)
                .format(|out, message, record| {
                    out.finish(format_args!("{}[{}][flow][{}] {}",
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
            AudioDevice::init(app.handle().clone())?;
            AudioDevice::listen(&ws_client);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![split, tone, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
