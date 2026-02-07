use std::sync::OnceLock;
use std::time::Duration;
use std::time::Instant;

use anyhow::Result;
use anyhow::bail;
use anyhow::ensure;
use byteorder::ByteOrder;
use byteorder::LittleEndian;
use chrono::Local;
use chrono::Utc;
use futures_lite::stream::StreamExt;
use futures_util::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use tokio::sync::mpsc;
use tokio::sync::broadcast;

use crate::commands::PlayRequest;
use crate::commands::PlayResond;
use crate::device::frontend::FClient;
use crate::device::frontend::FEvent;

static WS_SENDER: OnceLock<mpsc::UnboundedSender<Message>> = OnceLock::new();

/// Delay in seconds before reconnecting
const RECONNECT_DELAY_SECS: u64 = 2;

#[derive(Clone)]
pub enum WsEvent {
    Disconnected,
    Connected,
    Text(String),
    Play(PlayResond),
    Binary(Vec<u8>),
    Close(u16, String),
}


#[derive(Clone)]
pub struct WsClient {
    event_tx: broadcast::Sender<WsEvent>,
}

impl WsClient {
    pub fn init(url: &str) -> Result<Self> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
        WS_SENDER.set(tx).expect("set sender failed");

        let url = url.to_string();
        let (event_tx, _) = broadcast::channel(100);
        let event_tx_inner = event_tx.clone();
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
                broadcast_event(&event_tx_inner, WsEvent::Connected);
                log::info!("WebSocket connected");

                let disconnected = run_message_loop(&mut websocket, &event_tx_inner, &mut rx).await;

                if disconnected {
                    broadcast_event(&event_tx_inner, WsEvent::Disconnected);
                    log::warn!("WebSocket disconnected, reconnecting in {}s...", RECONNECT_DELAY_SECS);
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                }
            }
        });

        let client = Self { event_tx };
        Ok(client)
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WsEvent> {
        self.event_tx.subscribe()
    }
    /// Send text message to the server
    pub fn handle_play(req: PlayRequest) -> Result<()> {
        let msg = serde_json::to_string(&req)?;
        WS_SENDER.get().unwrap().send(Message::Text(msg.into()))?;
        Ok(())
    }
}

/// Broadcast event to subscribers; log at trace level when there are no subscribers
fn broadcast_event(tx: &broadcast::Sender<WsEvent>, event: WsEvent) {
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
    event_tx: &broadcast::Sender<WsEvent>,
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
                                broadcast_event(event_tx, WsEvent::Text(text));
                            }
                            Message::Binary(bytes) => {
                                broadcast_event(event_tx, distribute_binary_data(&bytes));
                            }
                            Message::Close { code, reason } => {
                                log::info!("server closed connection {} - {}", code, reason);
                                broadcast_event(event_tx, WsEvent::Close(code.into(), reason));
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


fn distribute_binary_data(data: &[u8]) -> WsEvent {
    if let Ok(event) = unpack_pcm_data(data) {
        return event;
    }
    log::error!("Failed to unpack binary data");
    WsEvent::Binary(data.to_vec())
}

fn unpack_pcm_data(data: &[u8]) -> Result<WsEvent> {
    ensure!(data.len() > 24);
    let magic_bytes = &data[0..20];
    let binding = String::from_utf8(magic_bytes.to_vec()).unwrap();
    let magic = binding.trim_end_matches('\0');
    match magic {
        "play" => {

            let id_bytes = &data[20..24];
            let id = LittleEndian::read_u32(id_bytes) ;
            FClient::send_event(FEvent::TTSFinished { timestamp: Utc::now().timestamp_millis() as u64, id: id });
            
            let pcm_data = data[24..].to_vec();
            return Ok(WsEvent::Play(PlayResond { data: pcm_data, id }))
        },
        _ => {
            bail!("unsupported magic {magic}")
        }
    }
    
}