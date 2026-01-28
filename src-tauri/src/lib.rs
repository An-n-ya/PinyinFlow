use serde::{Deserialize, Serialize};
use thiserror::Error;

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
    eprintln!("split {input}");
    dollop::split(input)
}
#[tauri::command]
async fn tone(input: &str) -> Result<PinyinRespond, ()> {
    eprintln!("tone {input}");
    let client = reqwest::Client::new();
    
    let req_body = PinyinRequest {
        pinyin: input.to_string(),
    };

    let res = client.post("http://localhost:8000/tone")
        .json(&req_body)
        .send()
        .await.expect("result").text().await.unwrap();

    eprintln!("tone {res}");
    let v: PinyinRespond = serde_json::from_str(&res).unwrap();
    
    Ok(v)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, split, tone])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
