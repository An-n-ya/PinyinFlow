use std::str::from_utf8;
use std::time::Duration;

use crate::commands::PlayResond;
use crate::device::frontend::FClient;
use crate::device::frontend::FEvent;
use crate::service::tts::providers::Provider;
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
pub(crate) struct KokoroTTS {
    url: String,
}

impl Default for KokoroTTS {
    fn default() -> Self {
        Self {
            url: "ws://localhost:8000/play".into(),
        }
    }
}

const RECONNECT_DELAY_SECS: u64 = 2;

/// Broadcast event to subscribers; log at trace level when there are no subscribers
fn broadcast_event(tx: &broadcast::Sender<TTSEvent>, event: TTSEvent) {
    if tx.send(event).is_err() {
        log::trace!("no subscribers, event not delivered");
    }
}

/// Establish WebSocket connection
async fn connect(client: &Client, url: &str) -> Result<WebSocket> {
    let response = client.get(url).upgrade().send().await?;
    let websocket = response.into_websocket().await?;
    Ok(websocket)
}

/// Run message loop until disconnected. Returns true when exited due to disconnect (reconnect needed).
async fn run_message_loop(
    websocket: &mut WebSocket,
    event_tx: &broadcast::Sender<TTSEvent>,
    rx: &mut mpsc::UnboundedReceiver<Message>,
) -> bool {
    loop {
        tokio::select! {
            msg = websocket.next() => {
                match msg {
                    Some(Ok(message)) => {
                        match message {
                            Message::Ping(payload) => {
                                if let Err(e) = websocket.send(Message::Pong(payload)).await {
                                    log::error!("failed to send Pong: {}", e);
                                    return true;
                                }
                            }
                            Message::Text(text) => {
                                unimplemented!("unimplemented message type: {:?}", text)
                            }
                            Message::Binary(bytes) => {
                                broadcast_event(event_tx, distribute_binary_data(&bytes));
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
                if let Err(e) = websocket.send(outbound_msg).await {
                    log::error!("failed to send message: {}", e);
                    return true;
                }
            }
        }
    }
}

fn distribute_binary_data(data: &[u8]) -> TTSEvent {
    if let Ok(event) = unpack_pcm_data(data) {
        return event;
    }
    log::error!("Failed to unpack binary data");
    TTSEvent::Binary(data.to_vec())
}

fn unpack_pcm_data(data: &[u8]) -> Result<TTSEvent> {
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
            return Ok(TTSEvent::Play(PlayResond { data: pcm_data, id }));
        }
        _ => {
            bail!("unsupported magic {magic}")
        }
    }
}

impl Provider for KokoroTTS {
    fn prepare_play_message(
        &self,
        req: crate::service::tts::service::TTSPlayRequest,
    ) -> reqwest_websocket::Message {
        let msg = serde_json::to_string(&req).expect("failed to serialize play request");
        log::debug!("sending play request: {msg}");
        Message::Text(msg.into())
    }

    fn event_loop(
        &self,
        event_tx: tokio::sync::broadcast::Sender<crate::service::tts::service::TTSEvent>,
        mut ws_msg_rx: tokio::sync::mpsc::UnboundedReceiver<reqwest_websocket::Message>,
    ) {
        let url = self.url.clone();
        tauri::async_runtime::spawn(async move {
            let client = Client::default();

            loop {
                let mut websocket = match connect(&client, &url).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        log::error!("WebSocket connection failed: {}", e);
                        tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                        continue;
                    }
                };
                broadcast_event(&event_tx, TTSEvent::Connected);
                log::info!("WebSocket connected");

                let disconnected =
                    run_message_loop(&mut websocket, &event_tx, &mut ws_msg_rx).await;

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
