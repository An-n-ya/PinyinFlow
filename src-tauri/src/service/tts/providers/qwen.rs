use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Duration;

use anyhow::Result;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use chrono::Local;
use chrono::Utc;
use futures_lite::stream::StreamExt;
use futures_util::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use serde::Deserialize;
use serde::Serialize;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::commands::PlayRequest;
use crate::device::frontend::FClient;
use crate::device::frontend::FEvent;
use crate::service::tts::providers::Provider;
use crate::service::tts::service::TTSEvent;

#[derive(Clone, Debug, PartialEq)]
enum SessionMode {
    ServerCommit,
    Commit,
}
impl ToString for SessionMode {
    fn to_string(&self) -> String {
        match self {
            SessionMode::ServerCommit => "server_commit".into(),
            SessionMode::Commit => "commit".into(),
        }
    }
}

#[derive(Clone, Debug, Default)]
struct State {
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
    #[serde(rename = "session.finish")]
    SessionFinish { event_id: String },
    #[serde(rename = "input_text_buffer.append")]
    AppendText { event_id: String, text: String },
    #[serde(rename = "input_text_buffer.commit")]
    CommitText { event_id: String },
}

impl WsRequest {
    async fn send(&self, websocket: &mut WebSocket) {
        let event = serde_json::to_string(self).unwrap();
        if let Err(e) = websocket.send(Message::Text(event)).await {
            log::error!("failed to send websocket event: {}", e);
        }
    }
    pub fn commit_text() -> Self {
        Self::CommitText {
            event_id: format!("event_{}", Local::now().timestamp_millis()),
        }
    }
    pub fn session_finish() -> Self {
        Self::SessionFinish {
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
    #[serde(rename = "session.finished")]
    SessionFinished { event_id: String },
    #[serde(rename = "session.updated")]
    SessionUpdated {
        event_id: String,
        session: SessionInfo,
    },
    #[serde(rename = "error")]
    ResponseError {
        event_id: String,
        error: ResponseErrorInfo,
    },
    #[serde(rename = "input_text_buffer.committed")]
    TextBufferCommitted { event_id: String, item_id: String },
    #[serde(rename = "response.created")]
    ResponseCreated {
        event_id: String,
        response: ResponseCreatedInfo,
    },
    #[serde(rename = "response.output_item.added")]
    ResponseOutputItemAdded { event_id: String },
    #[serde(rename = "response.output_item.done")]
    ResponseOutputItemDone { event_id: String },
    #[serde(rename = "response.content_part.added")]
    ResponseContentPartAdded { event_id: String },
    #[serde(rename = "response.content_part.done")]
    ResponseContentPartDone { event_id: String },
    #[serde(rename = "response.audio.delta")]
    AudioDelta {
        event_id: String,
        delta: String,
        response_id: String,
        item_id: String,
        output_index: usize,
        content_index: usize,
    },
    #[serde(rename = "response.audio.done")]
    AudioDone {
        event_id: String,
        response_id: String,
    },
    #[serde(rename = "response.done")]
    ResponseDone {
        event_id: String,
        response: ResponseDoneInfo,
    },
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseDoneUsage {
    characters: usize,
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ResponseDoneInfo {
    id: String,
    usage: ResponseDoneUsage,
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
    language_type: Option<String>,
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
            language_type: Some("auto".into()),
            response_format: "pcm".into(),
            sample_rate: 24000,
        }
    }
}
fn broadcast_event(tx: &broadcast::Sender<TTSEvent>, event: TTSEvent) {
    if tx.send(event).is_err() {
        log::trace!("no subscribers, event not delivered");
    }
}

impl QWenTTS {
    pub fn builder() -> QWenTTSBuilder {
        QWenTTSBuilder::default()
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

    async fn initialize_connection(&self, websocket: &mut WebSocket) {
        WsRequest::session_update(self.session.clone())
            .send(websocket)
            .await;
    }

    async fn run_message_loop(
        &self,
        websocket: &mut WebSocket,
        event_tx: &broadcast::Sender<TTSEvent>,
        rx: &mut mpsc::UnboundedReceiver<Message>,
    ) -> bool {
        self.initialize_connection(websocket).await;
        loop {
            tokio::select! {
                msg = websocket.next() => {
                    match msg {
                        Some(Ok(message)) => {
                            match message {
                                Message::Ping(payload) => {
                                    log::debug!("收到 Ping, 发送 Pong");
                                    if let Err(e) = websocket.send(Message::Pong(payload)).await {
                                        log::error!("failed to send Pong: {}", e);
                                        return true;
                                    }
                                }
                                Message::Text(text) => {
                                    // log::info!("receive message: {text}");
                                    use WsResponse::*;
                                    match serde_json::from_str::<WsResponse>(&text) {
                                        Ok(response) => match response {
                                            SessionCreated{event_id,session}=>{
                                                broadcast_event(&event_tx,TTSEvent::Connected);
                                                log::info!("[{}]会话创建成功, ID:{}",event_id, session.id.unwrap());
                                            }
                                            SessionUpdated{event_id, session, ..}=>{log::debug!("[{}]配置已更新, ID:{}",event_id, session.id.unwrap());}
                                            SessionFinished{event_id}=>log::info!("[{}]session finished", event_id),
                                            TextBufferCommitted{event_id, item_id}=>log::info!("[{}]Text Buffer Committed ID:{}", event_id,item_id),
                                            ResponseCreated{event_id, response}=>{
                                                let mut state = self.state.lock().unwrap();
                                                let req_id = state.cur_req_id.take();
                                                state.req_res_map.insert(response.id.clone(), req_id.unwrap());
                                                log::info!("[{}]Response Created", event_id);
                                            },
                                            ResponseOutputItemAdded{event_id}=>log::info!("[{}]Response Output Item Added", event_id),
                                            ResponseOutputItemDone{event_id}=>log::info!("[{}]Response Output Item Done", event_id),
                                            ResponseContentPartAdded{event_id}=>log::info!("[{}]Response Content Part Added", event_id),
                                            ResponseContentPartDone{event_id}=>log::info!("[{}]Response Content Part Done", event_id),
                                            AudioDelta{event_id, delta, ..}=>{
                                                log::info!("[{}]收到音频数据, 长度: {} bytes",event_id, delta.len());
                                                let binary_data=BASE64_STANDARD.decode(delta).unwrap();
                                                broadcast_event(&event_tx,TTSEvent::Play(PlayRequest{data:binary_data,id:"".into()}));
                                            }
                                            AudioDone{event_id, ..}=>log::info!("[{}]Audio Done", event_id),
                                            ResponseDone{event_id, response}=>{
                                                let state = self.state.lock().unwrap();
                                                state.req_res_map.get(&response.id).map(|req_id|{
                                                    FClient::send_event(FEvent::TTSFinished {
                                                        timestamp: Utc::now().timestamp_millis() as u64,
                                                        id: req_id.clone(),
                                                    });
                                                    broadcast_event(&event_tx,TTSEvent::Finished{id: req_id.clone()});
                                                });

                                                log::info!("[{}]Response Done. Cost:{:?}",event_id, response);
                                            },
                                            ResponseError { event_id, error } => log::error!("[{}]QWenTTS ERROR: {error:?}", event_id),
                                            Unknown=>{log::warn!("收到了一个暂时没处理的事件类型: {}",text);}
                                                                                    },
                                        Err(e) => log::error!("JSON 解析失败: {}, 原数据: {}", e, text),
                                    }
                                }
                                Message::Binary(bytes) => {
                                    unimplemented!("unimplemented message type: {:?}", bytes.len())
                                }
                                Message::Close { code, reason } => {
                                    log::info!("server closed connection {} - {}", code, reason);
                                    broadcast_event(event_tx, TTSEvent::Close(code.into(), reason));
                                    return true;
                                }
                                _ => {}
                            }
                        }
                        Some(Err(e)) => {
                            log::error!("WebSocket error: {}", e);
                            return true;
                        }
                        None => return true,
                    }
                }
                Some(outbound_msg) = rx.recv() => {
                    log::debug!("sending message: {:?}", outbound_msg);
                    if let Err(e) = websocket.send(outbound_msg).await {
                        log::error!("failed to send message: {}", e);
                        return true;
                    }
                }
            }
        }
    }
}
impl QWenTTSBuilder {
    fn default_url() -> String {
        "wss://dashscope.aliyuncs.com/api-ws/v1/realtime?model=qwen3-tts-flash-realtime".into()
    }
}

const RECONNECT_DELAY_SECS: u64 = 2;

impl Provider for QWenTTS {
    fn prepare_play_message(
        &self,
        req: crate::service::tts::service::TTSPlayRequest,
    ) -> Vec<reqwest_websocket::Message> {
        let append_event = serde_json::to_string(&WsRequest::append_text(req.input)).unwrap();
        let commit_event = serde_json::to_string(&WsRequest::commit_text()).unwrap();

        let mut state = self.state.lock().unwrap();
        // FIXME: for now we assume that the last request is finished
        assert!(
            state.cur_req_id.is_none(),
            "last request is not finished yet"
        );
        state.cur_req_id = Some(req.id);

        log::debug!("sending play message: {append_event}");
        vec![
            reqwest_websocket::Message::Text(append_event),
            reqwest_websocket::Message::Text(commit_event),
        ]
    }

    fn event_loop(
        &self,
        event_tx: tokio::sync::broadcast::Sender<crate::service::tts::service::TTSEvent>,
        mut ws_msg_rx: tokio::sync::mpsc::UnboundedReceiver<reqwest_websocket::Message>,
    ) {
        let tts = self.clone();
        tauri::async_runtime::spawn(async move {
            // NOTE: http1_only is required for websocket upgrade
            let client = Client::builder().http1_only().build().unwrap();

            loop {
                let mut websocket = match tts.connect(&client).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        log::error!("WebSocket connection failed: {}", e);
                        tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                        continue;
                    }
                };

                let disconnected = tts
                    .run_message_loop(&mut websocket, &event_tx, &mut ws_msg_rx)
                    .await;

                if disconnected {
                    broadcast_event(&event_tx, TTSEvent::Disconnected);
                    log::warn!(
                        "WebSocket disconnected, reconnecting in {}s...",
                        RECONNECT_DELAY_SECS
                    );
                    log::warn!(
                        "WebSocket disconnected, reconnecting in {}s...",
                        RECONNECT_DELAY_SECS
                    );
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{env, path::Path};

    use crate::device::audio::AudioDevice;

    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
        dotenvy::from_path(Path::new("../.env.local")).unwrap();
    }

    #[tokio::test]
    async fn test_qwentts_initialize_connection() -> anyhow::Result<()> {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream().unwrap();
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        init();
        let (tx, rx) = mpsc::unbounded_channel::<Message>();
        let (event_tx, _) = broadcast::channel(100);
        let tts = QWenTTS::builder()
            .api_key(env::var("VITE_DASHSCOPE_API_KEY").unwrap())
            .build()?;
        tts.event_loop(event_tx.clone(), rx);

        tokio::time::sleep(Duration::from_secs_f32(0.2)).await;
        let event =
            serde_json::to_string(&WsRequest::append_text("为啥会有滋啦滋啦的声音？".into()))
                .unwrap();
        log::debug!("sending play message: {event}");
        tx.send(Message::Text(event)).unwrap();

        let event = serde_json::to_string(&WsRequest::commit_text()).unwrap();
        log::debug!("sending text commit: {event}");
        tx.send(Message::Text(event)).unwrap();

        while let Ok(event) = event_tx.subscribe().recv().await {
            match event {
                crate::service::tts::service::TTSEvent::Play(res) => {
                    log::debug!("received event TTSEvent::Play");
                    let source = AudioDevice::pcm_bytes_to_source(&res.data);
                    sink.append(source);
                }
                _ => {}
            }
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
        Ok(())
    }
}
