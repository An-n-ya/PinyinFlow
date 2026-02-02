mod device;

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use thiserror::Error;
use anyhow::Result;
use anyhow_tauri::TAResult;
use crate::device::audio::AudioDevice;

#[derive(Error, Debug)]
pub enum DataStoreError {
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("unknown data store error")]
    Unknown,
}

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

async fn pcm_bytes_from_ws(pinyin: &str) -> Result<Vec<u8>> {
    // Extends the `reqwest::RequestBuilder` to allow WebSocket upgrades.
    use futures_lite::stream::StreamExt;
    use futures_util::sink::SinkExt;
    use reqwest::Client;
    use reqwest_websocket::Message;
    use reqwest_websocket::RequestBuilderExt;

    // Creates a GET request, upgrades and sends it.
    let response = Client::default()
        .get("ws://localhost:8000/play")
        .upgrade() // Prepares the WebSocket upgrade.
        .send()
        .await?;

    // Turns the response into a WebSocket stream.
    let mut websocket = response.into_websocket().await?;

    // The WebSocket implements `Sink<Message>`.
    websocket.send(Message::Text(pinyin.into())).await?;

    // The WebSocket is also a `TryStream` over `Message`s.
    while let Some(message) = websocket.try_next().await? {
        if let Message::Binary(text) = message {
            log::info!("got pcm data");
            let _ = websocket.close(reqwest_websocket::CloseCode::Normal, None);
            return Ok(text.to_vec());
        }
    }

    Ok(vec![])
}
async fn play_pcm_from_ws(state: State<'_, AppData>, pinyin: &str) {
    log::info!("pcm from ws: pinyin: {}", pinyin);
    let pcm_bytes = pcm_bytes_from_ws(pinyin).await.unwrap();
    log::info!("pcm len: {}", pcm_bytes.len());
    
    state.audio_device.play_pcm_bytes(&pcm_bytes);

    return;
}
#[tauri::command]
async fn play(state: State<'_, AppData>,input: String) -> TAResult<String> {
    play_pcm_from_ws(state, &input).await;
    return Ok("OK".to_string());
}


#[derive(Default)]
struct AppData {
    audio_device: AudioDevice
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppData::default());
            Ok(())
        })
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![split, tone, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
