use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::commands::PlayRequest;
use crate::device::frontend::FClient;
use crate::device::frontend::FEvent;
use crate::service::tts::providers::{broadcast_event, Provider};
use crate::service::tts::service::TTSEvent;
use anyhow::Result;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Local;
use chrono::Utc;
use futures_util::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::broadcast;

#[derive(Clone, Debug, PartialEq)]
enum SessionMode {
    ServerCommit,
}
impl ToString for SessionMode {
    fn to_string(&self) -> String {
        match self {
            SessionMode::ServerCommit => "server_commit".into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct State {
    ready: bool,
    cur_req_id: Option<String>,
    req_res_map: HashMap<String, String>,
}

#[derive(Clone, Debug, derive_builder::Builder)]
pub(crate) struct QWenTTS {
    #[builder(default = "Self::default_url()")]
    url: String,
    api_key: String,
    #[builder(default)]
    state: Arc<Mutex<State>>,
    #[builder(default)]
    session: SessionInfo,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
enum WsRequest {
    #[serde(rename = "session.update")]
    SessionUpdate {
        event_id: String,
        session: SessionInfo,
    },
    #[serde(rename = "input_text_buffer.append")]
    AppendText { event_id: String, text: String },
    #[serde(rename = "input_text_buffer.commit")]
    CommitText { event_id: String },
}

impl WsRequest {
    async fn send(&self, websocket: &mut WebSocket) {
        let event = serde_json::to_string(self).unwrap();
        if let Err(e) = websocket.send(Message::Text(event.into())).await {
            log::error!("failed to send websocket event: {}", e);
        }
    }
    pub fn commit_text() -> Self {
        Self::CommitText {
            event_id: format!("event_{}", Local::now().timestamp_millis()),
        }
    }
    pub fn session_update(session: SessionInfo) -> Self {
        Self::SessionUpdate {
            event_id: format!("event_{}", Local::now().timestamp_millis()),
            session,
        }
    }
    pub fn append_text(text: String) -> Self {
        Self::AppendText {
            event_id: format!("event_{}", Local::now().timestamp_millis()),
            text,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum WsResponse {
    #[serde(rename = "session.created")]
    SessionCreated {
        event_id: String,
        session: SessionInfo,
    },
    #[serde(rename = "session.updated")]
    SessionUpdated {
        event_id: String,
        session: SessionInfo,
    },
    #[serde(rename = "response.created")]
    ResponseCreated {
        event_id: String,
        response: ResponseCreatedInfo,
    },
    #[serde(rename = "response.audio.delta")]
    AudioDelta {
        event_id: String,
        delta: String,
        response_id: String,
    },
    #[serde(rename = "response.done")]
    ResponseDone {
        event_id: String,
        response: ResponseDoneInfo,
    },
    #[serde(rename = "error")]
    ResponseError {
        event_id: String,
        error: ResponseErrorInfo,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseDoneInfo {
    id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseCreatedInfo {
    id: String,
    status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseErrorInfo {
    code: String,
    message: String,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInfo {
    id: Option<String>,
    model: String,
    voice: String,
    mode: String,
    response_format: String,
    sample_rate: usize,
}
impl Default for SessionInfo {
    fn default() -> Self {
        Self {
            id: None,
            model: "qwen3-tts-flash-realtime".into(),
            voice: "Cherry".into(),
            mode: SessionMode::ServerCommit.to_string(),
            response_format: "pcm".into(),
            sample_rate: 24000,
        }
    }
}

impl QWenTTS {
    pub fn builder() -> QWenTTSBuilder {
        QWenTTSBuilder::default()
    }
}
impl QWenTTSBuilder {
    fn default_url() -> String {
        "wss://dashscope.aliyuncs.com/api-ws/v1/realtime?model=qwen3-tts-flash-realtime".into()
    }
}

#[async_trait]
impl Provider for QWenTTS {
    fn name(&self) -> String {
        "QWen".into()
    }
    async fn connect(&self, client: &Client) -> Result<WebSocket> {
        let response = client
            .get(self.url.clone())
            .version(reqwest::Version::HTTP_11)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json; charset=utf-8")
            .upgrade()
            .send()
            .await?;
        let websocket = response.into_websocket().await?;
        Ok(websocket)
    }

    async fn on_connected(
        &self,
        websocket: &mut WebSocket,
        _event_tx: &broadcast::Sender<TTSEvent>,
    ) -> Result<()> {
        WsRequest::session_update(self.session.clone())
            .send(websocket)
            .await;
        Ok(())
    }

    async fn handle_text(
        &self,
        text: String,
        event_tx: &broadcast::Sender<TTSEvent>,
    ) -> Result<()> {
        use WsResponse::*;
        match serde_json::from_str::<WsResponse>(&text) {
            Ok(response) => match response {
                SessionCreated { event_id, session } => {
                    broadcast_event(event_tx, TTSEvent::Connected);
                    log::info!(
                        "[{}]session created, ID:{}",
                        event_id,
                        session.id.unwrap_or_default()
                    );
                }
                SessionUpdated {
                    event_id, session, ..
                } => {
                    log::debug!(
                        "[{}]session updated, ID:{}",
                        event_id,
                        session.id.unwrap_or_default()
                    );
                }
                ResponseCreated { event_id, response } => {
                    let mut state = self.state.lock().await;
                    let req_id = state.cur_req_id.take();
                    state
                        .req_res_map
                        .insert(response.id.clone(), req_id.unwrap_or_default());
                    log::info!("[{}]Response Created", event_id);
                }
                AudioDelta {
                    event_id, delta, ..
                } => {
                    log::debug!(
                        "[{}]received audio data, len: {} bytes",
                        event_id,
                        delta.len()
                    );
                    let binary_data = BASE64_STANDARD.decode(delta).unwrap();
                    broadcast_event(
                        event_tx,
                        TTSEvent::Play(PlayRequest {
                            data: binary_data,
                            id: "".into(),
                        }),
                    );
                }
                ResponseDone { event_id, response } => {
                    let state = self.state.lock().await;
                    state.req_res_map.get(&response.id).map(|req_id| {
                        FClient::send_event(FEvent::TTSFinished {
                            timestamp: Utc::now().timestamp_millis() as u64,
                            id: req_id.clone(),
                        });
                        broadcast_event(event_tx, TTSEvent::Finished { id: req_id.clone() });
                    });
                    log::info!("[{}]Response Done", event_id);
                }
                ResponseError { event_id, error } => {
                    log::error!("[{}]QWenTTS ERROR: {error:?}", event_id)
                }
                Unknown => {}
            },
            Err(e) => log::error!("JSON parse failed: {}, raw data: {}", e, text),
        }
        Ok(())
    }

    fn prepare_play_message(
        &self,
        req: crate::service::tts::service::TTSPlayRequest,
    ) -> Vec<reqwest_websocket::Message> {
        let append_event = serde_json::to_string(&WsRequest::append_text(req.input)).unwrap();
        let commit_event = serde_json::to_string(&WsRequest::commit_text()).unwrap();

        let mut state = futures::executor::block_on(self.state.lock());
        state.cur_req_id = Some(req.id);

        vec![
            reqwest_websocket::Message::Text(append_event.into()),
            reqwest_websocket::Message::Text(commit_event.into()),
        ]
    }

    fn is_ready(&self) -> bool {
        let state = futures::executor::block_on(self.state.lock());
        state.ready
    }

    fn set_ready(&self, ready: bool) {
        futures::executor::block_on(self.state.lock()).ready = ready;
    }
}
