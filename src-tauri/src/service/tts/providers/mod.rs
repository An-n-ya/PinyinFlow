use std::fmt::Debug;
use std::time::Duration;
use std::{env, sync::Arc};

use async_trait::async_trait;
use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::{Message, WebSocket};
use tokio::sync::broadcast;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::service::tts::{
    providers::{kokoro::KokoroTTS, qwen::QWenTTS},
    service::{TTSEvent, TTSPlayRequest, TTSService, TTSServiceCommand},
};

mod kokoro;
mod qwen;

pub const RECONNECT_DELAY_SECS: u64 = 2;

pub(crate) fn broadcast_event(tx: &broadcast::Sender<TTSEvent>, event: TTSEvent) {
    if tx.send(event).is_err() {
        log::trace!("no subscribers, event not delivered");
    }
}

/// 统一管理 TTS 通信通道的 Hub
pub struct TTSChannelHub {
    /// 指令发送端 (Service -> Provider)
    command_tx: mpsc::UnboundedSender<Message>,
    /// 指令接收端 (Provider 消费)
    command_rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Message>>>,
    /// 事件广播端 (Provider -> Service/Frontend)
    event_tx: broadcast::Sender<TTSEvent>,
}

impl TTSChannelHub {
    pub fn new() -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let (event_tx, _) = broadcast::channel(100);
        Self {
            command_tx,
            command_rx: Arc::new(tokio::sync::Mutex::new(command_rx)),
            event_tx,
        }
    }

    /// 获取发往 Provider 的指令发送端
    pub fn command_tx(&self) -> mpsc::UnboundedSender<Message> {
        self.command_tx.clone()
    }

    /// 获取由 Provider 发出的事件广播端
    pub fn event_tx(&self) -> broadcast::Sender<TTSEvent> {
        self.event_tx.clone()
    }

    /// 订阅 Provider 发出的事件
    pub fn subscribe_events(&self) -> broadcast::Receiver<TTSEvent> {
        self.event_tx.subscribe()
    }

    /// 获取供 Provider 使用的指令接收端锁
    pub(crate) fn command_rx_lock(
        &self,
    ) -> Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Message>>> {
        self.command_rx.clone()
    }
}

#[async_trait]
pub(crate) trait Provider: Send + Sync + Debug + 'static {
    fn name(&self) -> String;
    async fn connect(&self, client: &reqwest::Client) -> anyhow::Result<WebSocket>;

    async fn on_connected(
        &self,
        _websocket: &mut WebSocket,
        event_tx: &broadcast::Sender<TTSEvent>,
    ) -> anyhow::Result<()> {
        broadcast_event(event_tx, TTSEvent::Connected);
        Ok(())
    }

    async fn handle_text(
        &self,
        _text: String,
        _event_tx: &broadcast::Sender<TTSEvent>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn handle_binary(
        &self,
        _bytes: Vec<u8>,
        _event_tx: &broadcast::Sender<TTSEvent>,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    async fn run_message_loop(
        &self,
        websocket: &mut WebSocket,
        event_tx: &broadcast::Sender<TTSEvent>,
        rx: &mut UnboundedReceiver<Message>,
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
                                    if let Err(e) = self.handle_text(text, event_tx).await {
                                        log::error!("Error handling text message: {}", e);
                                    }
                                }
                                Message::Binary(bytes) => {
                                    if let Err(e) = self.handle_binary(bytes.to_vec(), event_tx).await {
                                        log::error!("Error handling binary message: {}", e);
                                    }
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
                            log::info!("Closing websocket connection of {}...", self.name());
                            let _ = websocket.close().await;
                            return EventLoopRet::Close;
                        }
                    }
                    if let Err(e) = websocket.send(outbound_msg).await {
                        log::error!("failed to send message: {}", e);
                        return EventLoopRet::Disconnected;
                    }
                }
            }
        }
    }

    fn prepare_play_message(&self, req: TTSPlayRequest) -> Vec<Message>;
    fn is_ready(&self) -> bool;
    fn set_ready(&self, ready: bool);
}

pub fn spawn_event_loop(tts: Arc<dyn Provider>, hub: Arc<TTSChannelHub>) {
    let event_tx = hub.event_tx();
    let command_rx = hub.command_rx_lock();

    tauri::async_runtime::spawn(async move {
        let client = reqwest::Client::builder().http1_only().build().unwrap();

        loop {
            let mut websocket = match tts.connect(&client).await {
                Ok(ws) => ws,
                Err(e) => {
                    log::error!("WebSocket connection failed: {}", e);
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                    continue;
                }
            };

            if let Err(e) = tts.on_connected(&mut websocket, &event_tx).await {
                log::error!("Failed to handle connection start: {}", e);
                tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                continue;
            }

            tts.set_ready(true);
            log::info!("WebSocket connected");

            let mut receiver = command_rx.lock().await;
            let event_loop_ret = tts
                .run_message_loop(&mut websocket, &event_tx, &mut *receiver)
                .await;
            drop(receiver);

            match event_loop_ret {
                EventLoopRet::Disconnected => {
                    tts.set_ready(false);
                    broadcast_event(&event_tx, TTSEvent::Disconnected);
                    log::warn!(
                        "WebSocket disconnected, reconnecting in {}s...",
                        RECONNECT_DELAY_SECS
                    );
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                }
                EventLoopRet::Close => {
                    tts.set_ready(false);
                    return;
                }
            }
        }
    });
}

pub enum EventLoopRet {
    Disconnected,
    Close,
}

type TTSProvider = Arc<dyn Provider>;

#[derive(Clone)]
pub struct TTSProviderManager {
    providers: Vec<Option<TTSProvider>>,
    selected: Option<TTSProvider>,
    hub: Arc<TTSChannelHub>,
}

impl TTSProviderManager {
    pub fn init() -> Self {
        let hub = Arc::new(TTSChannelHub::new());
        let qwen_tts = QWenTTS::builder()
            .api_key(env::var("VITE_DASHSCOPE_API_KEY").unwrap_or_default())
            .build()
            .unwrap();

        Self {
            providers: vec![
                Some(Arc::new(KokoroTTS::default())),
                Some(Arc::new(qwen_tts)),
            ],
            selected: None,
            hub,
        }
    }

    pub fn hub(&self) -> Arc<TTSChannelHub> {
        self.hub.clone()
    }

    fn send_close_event(&mut self) {
        let sender = self.hub().command_tx();
        if let Err(e) = sender.send(Message::Text("Close".to_string())) {
            log::error!("failed to send close message: {}", e);
        }
    }

    pub fn close_selected(&mut self) {
        let s = self.selected.take();
        if let Some(s) = s {
            self.send_close_event();
            self.providers
                .iter_mut()
                .find(|p| p.is_none())
                .unwrap()
                .replace(s);
        }
    }
    pub fn select_by_name(&mut self, tts_name: &str) {
        self.close_selected();
        if let Some(provider) = self.providers.iter_mut().find(|p| {
            p.as_ref()
                .map_or(false, |provider| provider.name() == tts_name)
        }) {
            self.selected = provider.take();
        } else {
            // TODO: broadcast NOT_FOUND event to frontend
            log::warn!("TTS provider not found: {}", tts_name);
            return;
        }

        if let Some(provider) = self.selected.as_ref() {
            if !provider.is_ready() {
                spawn_event_loop(provider.clone(), self.hub());
            }
        }
    }
    pub fn selected(&mut self) -> &TTSProvider {
        if self.selected.is_none() {
            self.select_by_name("Kokoro");
        }
        self.selected.as_ref().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::{audio::AudioDevice, frontend::FClient};
    use std::{path::Path, time::Duration};

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
        dotenvy::from_path(Path::new("../.env.local")).unwrap();
        FClient::init(None);
        AudioDevice::init().unwrap();
    }

    #[tokio::test]
    async fn test_switch_tts() -> anyhow::Result<()> {
        init();
        let mut service = TTSService::init().unwrap();
        AudioDevice::listen(&service);
        service.execute(TTSServiceCommand::Switch {
            name: "QWen".into(),
        })?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        service.execute(TTSServiceCommand::Switch {
            name: "Kokoro".into(),
        })?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        service
            .execute(TTSServiceCommand::Play {
                id: "()".to_string(),
                input: "你好".to_string(),
            })
            .unwrap();
        tokio::time::sleep(Duration::from_secs(3)).await;
        Ok(())
    }
}
