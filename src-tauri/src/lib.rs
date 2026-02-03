mod device;

use reqwest_websocket::Message;

use serde::{Deserialize, Serialize};
use anyhow::{Result, bail};
use anyhow_tauri::TAResult;
use crate::device::{audio::AudioDevice, websocket::WsClient };

#[derive(Serialize, Debug)]
struct PinyinRequest {
    pinyin: String,
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

    let req_body = PinyinRequest {
        pinyin: input.to_string(),
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

pub fn play_pcm_from_ws(msg: Message) -> Result<()> {
    if let Message::Binary(pcm_bytes) = msg {
        log::debug!("receiving message success");
        AudioDevice::play_pcm_bytes(&pcm_bytes);
        return Ok(());
    }

    bail!("failed to parse text message")
}
#[tauri::command]
async fn play(input: String) -> TAResult<()> {
    WsClient::send_text(input)?;
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
        .setup(|_app| {
            log::info!("setup started");
            AudioDevice::init()?;
            WsClient::init("ws://localhost:8000/play")?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![split, tone, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
