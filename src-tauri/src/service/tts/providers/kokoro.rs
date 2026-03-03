use std::str::from_utf8;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::commands::PlayRequest;
use crate::device::frontend::FClient;
use crate::device::frontend::FEvent;
use crate::service::tts::providers::{broadcast_event, EventLoopRet, Provider};
use crate::service::tts::service::TTSEvent;
use anyhow::bail;
use anyhow::ensure;
use anyhow::Result;
use chrono::Utc;
use futures_lite::stream::StreamExt;
use futures_util::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

#[derive(Clone, Debug)]
struct State {
    ready: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct KokoroTTS {
    state: Arc<Mutex<State>>,
    url: String,
}

impl Default for KokoroTTS {
    fn default() -> Self {
        Self {
            url: "ws://localhost:8000/play".into(),
            state: Arc::new(Mutex::new(State { ready: false })),
        }
    }
}

fn distribute_binary_data(data: &[u8], event_tx: &broadcast::Sender<TTSEvent>) -> Vec<TTSEvent> {
    if let Ok(events) = unpack_pcm_data(data) {
        return events;
    }
    log::error!("Failed to unpack binary data");
    vec![TTSEvent::Binary(data.to_vec())]
}

fn unpack_pcm_data(data: &[u8]) -> Result<Vec<TTSEvent>> {
    ensure!(data.len() > 36);
    let magic_bytes = &data[0..20];
    let binding = String::from_utf8(magic_bytes.to_vec()).unwrap();
    let magic = binding.trim_end_matches('\0');
    match magic {
        "play" => {
            let id_bytes = &data[20..56];
            let id = from_utf8(id_bytes).unwrap().to_owned();
            FClient::send_event(FEvent::TTSFinished {
                timestamp: Utc::now().timestamp_millis() as u64,
                id: id.clone(),
            });

            let pcm_data = data[24..].to_vec();
            return Ok(vec![
                TTSEvent::Play(PlayRequest {
                    data: pcm_data,
                    id: id.clone(),
                }),
                TTSEvent::Finished { id },
            ]);
        }
        _ => {
            bail!("unsupported magic {magic}")
        }
    }
}

use async_trait::async_trait;

#[async_trait]
impl Provider for KokoroTTS {
    async fn connect(&self, client: &Client) -> Result<WebSocket> {
        let response = client.get(&self.url).upgrade().send().await?;
        let websocket = response.into_websocket().await?;
        Ok(websocket)
    }

    async fn run_message_loop(
        &self,
        websocket: &mut WebSocket,
        event_tx: &broadcast::Sender<TTSEvent>,
        rx: &mut mpsc::UnboundedReceiver<Message>,
    ) -> EventLoopRet {
        loop {
            tokio::select! {
                msg = websocket.next() => {
                    match msg {
                        Some(Ok(message)) => {
                            match message {
                                Message::Ping(payload) => {
                                    if let Err(e) = websocket.send(Message::Pong(payload)).await {
                                        log::error!("failed to send Pong: {}", e);
                                        return EventLoopRet::Disconnected;
                                    }
                                }
                                Message::Text(text) => {
                                    log::warn!("unimplemented message type: {:?}", text)
                                }
                                Message::Binary(bytes) => {
                                    distribute_binary_data(&bytes, event_tx).into_iter().for_each(|event| {
                                        broadcast_event(event_tx, event);
                                    });
                                }
                                Message::Close { code, reason } => {
                                    log::info!("server closed connection {} - {}", code, reason);
                                    broadcast_event(event_tx, TTSEvent::Close(code.into(), reason));
                                    return EventLoopRet::Disconnected;
                                }
                                _ => {}
                            }
                        }
                        Some(Err(e)) => {
                            log::error!("WebSocket error: {}", e);
                            return EventLoopRet::Disconnected;
                        }
                        None => return EventLoopRet::Disconnected,
                    }
                }
                Some(outbound_msg) = rx.recv() => {
                    if let Message::Text(t)  = &outbound_msg {
                        if t.starts_with("Close") {
                            log::info!("Closing Kokoro websocket...");
                            let _ = websocket.close().await;
                            return EventLoopRet::Close;
                        }
                    }
                    if let Err(e) = websocket.send(outbound_msg).await {
                        log::error!("Failed to send message: {}", e);
                        return EventLoopRet::Disconnected;
                    }
                }
            }
        }
    }

    fn prepare_play_message(
        &self,
        req: crate::service::tts::service::TTSPlayRequest,
    ) -> Vec<reqwest_websocket::Message> {
        let msg = serde_json::to_string(&req).expect("failed to serialize play request");
        log::debug!("sending play request: {msg}");
        vec![Message::Text(msg.into())]
    }

    fn is_ready(&self) -> bool {
        let state = futures::executor::block_on(self.state.lock());
        state.ready
    }

    fn set_ready(&self, ready: bool) {
        futures::executor::block_on(self.state.lock()).ready = ready;
    }
}
