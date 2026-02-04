use std::sync::OnceLock;
use std::time::Duration;

use anyhow::Result;
use futures_lite::stream::StreamExt;
use futures_util::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use tokio::sync::mpsc;
use tokio::sync::broadcast;

static WS_SENDER: OnceLock<mpsc::UnboundedSender<Message>> = OnceLock::new();

/// Delay in seconds before reconnecting
const RECONNECT_DELAY_SECS: u64 = 2;

#[derive(Clone)]
pub enum WsEvent {
    Disconnected,
    Connected,
    Text(String),
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
    pub fn send_text(text: String) -> Result<()> {
        WS_SENDER.get().unwrap().send(Message::Text(text.into()))?;
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
                                broadcast_event(event_tx, WsEvent::Binary(bytes.to_vec()));
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

