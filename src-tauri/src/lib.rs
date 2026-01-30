use serde::{Deserialize, Serialize};
use thiserror::Error;
use anyhow::Result;

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
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
use byteorder::{LittleEndian, ReadBytesExt};
use rodio::Source;
use std::io::Cursor;
fn pcm_bytes_to_source(pcm_bytes: &[u8]) -> impl Source<Item = f32> {
    // 1. 将字节流包装为 Cursor（可读取的缓冲区）
    let mut cursor = Cursor::new(pcm_bytes);
    // 2. 解析 16bit 小端 PCM 数据为 i16 采样值（根据实际格式调整 LittleEndian/BigEndian）
    let samples: Vec<f32> = std::iter::from_fn(move || {
        cursor
            .read_i16::<LittleEndian>()
            .ok()
            .map(|f| f as f32 / 32767.0)
    })
    .collect();

    // 3. 将采样值转换为 rodio 音频源，设置采样率（44100 Hz）
    rodio::buffer::SamplesBuffer::new(
        1,       // 声道数：1=单声道，2=立体声
        24000,   // 采样率
        samples, // 解析后的 PCM 采样数据
    )
}
async fn play_pcm_from_ws(pinyin: &str) {
    log::info!("pcm from ws: pinyin: {}", pinyin);
    let pcm_bytes = pcm_bytes_from_ws(pinyin).await.unwrap();
    log::info!("pcm len: {}", pcm_bytes.len());

    // 初始化音频输出设备
    let stream_handle =
        rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    // 将 PCM 字节转换为音频源并播放
    let source = pcm_bytes_to_source(&pcm_bytes);
    sink.append(source);

    // 等待播放完成
    sink.sleep_until_end();
}
#[tauri::command]
async fn play(input: String) -> String {
    play_pcm_from_ws(&input).await;
    return "OK".to_string();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, split, tone, play])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
}
