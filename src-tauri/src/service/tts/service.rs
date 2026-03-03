use anyhow::Result;
use reqwest_websocket::Message;
use serde::Serialize;
use tokio::sync::broadcast;

use crate::{commands::PlayRequest, service::tts::providers::TTSProviderManager};

#[derive(Clone, Debug)]
pub enum TTSEvent {
    Disconnected,
    Connected,
    Play(PlayRequest),
    Finished { id: String },
    Binary(Vec<u8>),
    Close(u16, String),
}

pub enum TTSServiceCommand {
    Play { id: String, input: String },
    Switch { name: String },
    Close,
}

#[derive(Serialize)]
pub struct TTSPlayRequest {
    pub id: String,
    pub input: String,
}

#[derive(Clone)]
pub struct TTSService {
    provider_manager: TTSProviderManager,
}

impl TTSService {
    pub fn init() -> Result<Self> {
        Ok(Self {
            provider_manager: TTSProviderManager::init(),
        })
    }

    pub fn subscribe(&self) -> broadcast::Receiver<TTSEvent> {
        self.provider_manager.hub().subscribe_events()
    }

    pub fn execute(&mut self, cmd: TTSServiceCommand) -> Result<()> {
        match cmd {
            TTSServiceCommand::Play { id, input } => {
                let req = TTSPlayRequest { id, input };
                let hub = self.provider_manager.hub();
                let sender = hub.command_tx();

                self.provider_manager
                    .selected()
                    .prepare_play_message(req)
                    .into_iter()
                    .for_each(|msg| {
                        if let Err(e) = sender.send(msg) {
                            log::error!("failed to send play message: {}", e);
                        }
                    });
            }
            TTSServiceCommand::Switch { name } => {
                self.provider_manager.select_by_name(&name);
            }
            TTSServiceCommand::Close => {
                self.provider_manager.close_selected();
            }
        }
        Ok(())
    }
}
