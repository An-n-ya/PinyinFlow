mod device;

use tokio::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{Manager, State};
use anyhow::{Context, Result};
use anyhow_tauri::TAResult;
use crate::device::{audio::AudioDevice, websocket::WSDevice};

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

async fn play_pcm_from_ws(state: &State<'_, Mutex<AppData>>, pinyin: &str) -> TAResult<()> {
    let mut state = state.lock().await;
    let pcm_bytes = state.websocket.pcm_bytes(pinyin).await.with_context(|| format!("request pcm bytes of '{pinyin}' failed"))?;
    
    state.audio_device.play_pcm_bytes(&pcm_bytes);

    Ok(())
}
#[tauri::command]
async fn play(state: State<'_, Mutex<AppData>>,input: String) -> TAResult<()> {
    play_pcm_from_ws(&state, &input).await?;
    return Ok(());
}


#[derive(Default)]
struct AppData {
    audio_device: AudioDevice,
    websocket: WSDevice
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData::default()));
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
