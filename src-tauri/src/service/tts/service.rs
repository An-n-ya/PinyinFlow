use reqwest_websocket::Message;
use serde::Serialize;
use std::sync::OnceLock;
use tokio::sync::mpsc;

use anyhow::Result;
use tokio::sync::broadcast;

use crate::{
    commands::PlayRequest,
    service::tts::providers::{TTSProvider, TTSProviderManager},
};

static WS_SENDER: OnceLock<mpsc::UnboundedSender<Message>> = OnceLock::new();

#[derive(Clone)]
pub enum TTSEvent {
    Disconnected,
    Connected,
    Play(PlayRequest),
    Finished { id: String },
    Binary(Vec<u8>),
    Close(u16, String),
}

#[derive(Serialize)]
pub struct TTSPlayRequest {
    pub id: String,
    pub input: String,
}

#[derive(Clone)]
pub struct TTSService {
    provider_manager: TTSProviderManager,
    event_tx: broadcast::Sender<TTSEvent>,
}

impl TTSService {
    pub fn init() -> Result<Self> {
        let (tx, rx) = mpsc::unbounded_channel::<Message>();
        WS_SENDER.set(tx).expect("set sender failed");

        let (event_tx, _) = broadcast::channel(100);

        let event_tx_inner = event_tx.clone();

        Ok(Self {
            provider_manager: TTSProviderManager::init(event_tx_inner, rx),
            event_tx: event_tx,
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<TTSEvent> {
        self.event_tx.subscribe()
    }

    pub fn play(&mut self, req: TTSPlayRequest) -> Result<()> {
        // TODO: move WS_SENDER to provider_manager
        let sender = WS_SENDER.get().unwrap();

        self.provider_manager
            .selected()
            .prepare_play_message(req)
            .into_iter()
            .for_each(|msg| {
                sender.send(msg).unwrap();
            });

        Ok(())
    }

    pub fn close() -> Result<()> {
        let sender = WS_SENDER.get().unwrap();
        sender.send(Message::Text("Close".to_string())).unwrap();
        Ok(())
    }
}
