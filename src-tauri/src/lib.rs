mod device;

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



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Debug)
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
