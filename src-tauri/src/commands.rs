use anyhow::Result;
use anyhow_tauri::TAResult;
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

use crate::{
    device::websocket::WsClient,
    service::llm::{
        domain::TaskType,
        service::LlmService,
        strategy::proofread::{ProofreadBuilder, ProofreadContext},
    },
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PlayResond {
    pub data: Vec<u8>,
    pub id: String,
}
#[derive(Serialize, Debug)]
pub struct PlayRequest {
    input: String,
    id: String,
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
        id: "1".to_string(),
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
pub async fn play(id: String, input: String) -> TAResult<()> {
    WsClient::handle_play(PlayRequest { id, input })?;
    return Ok(());
}

#[tauri::command]
pub async fn proofread(
    state: State<'_, Mutex<LlmService>>,
    id: String,
    input: String,
) -> TAResult<String> {
    let input_ = ProofreadContext {
        text: input.clone(),
    };
    let service = state.lock().await;
    let res: String = service
        .execute_task(TaskType::Proofread, ProofreadBuilder::default(), input_)
        .await
        .unwrap();
    log::info!("proofread: origin: {input}, revised: {:?}", res);
    Ok(res)
}
