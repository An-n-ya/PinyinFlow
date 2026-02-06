use serde::{Deserialize, Serialize};
use anyhow::Result;
use anyhow_tauri::TAResult;

use crate::device::websocket::WsClient;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlayResond {
    pub data: Vec<u8>,
    pub id: u32
}
#[derive(Serialize, Debug)]
pub struct PlayRequest {
    input: String,
    id: usize
}
#[derive(Deserialize, Serialize, Debug)]
pub struct PinyinRespond {
    pinyin: String,
    py_styled: String,
    tone: String,
}
#[tauri::command]
pub fn split(input: &str) -> String {
    log::info!("split {input}");
    dollop::split(input)
}
#[tauri::command]
pub async fn tone(input: &str) -> Result<PinyinRespond, String> {
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
pub async fn play(id: usize, input: String) -> TAResult<()> {
    WsClient::handle_play(PlayRequest{id, input})?;
    return Ok(());
}