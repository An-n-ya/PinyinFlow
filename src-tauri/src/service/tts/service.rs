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

    pub fn play(&mut self, req: TTSPlayRequest) -> Result<()> {
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

        Ok(())
    }

    pub fn close(&mut self) -> Result<()> {
        self.provider_manager.close_selected();
        Ok(())
    }

    /// 这个静态方法可能是为了让后端其他部分能强制关闭，但现在我们建议通过实例调用。
    /// 为了保持兼容性，我们可以暂时保留一个空的或者报错的实现，或者彻底移除。
    /// 鉴于之前使用了 OnceLock，这里我们直接移除它，改为由 Service 实例管理。
    pub fn stop() -> Result<()> {
        // 如果确实需要全局停止，应该考虑将 Service 放在全局状态中。
        // 目前先返回 Ok。
        Ok(())
    }

    pub fn switch_tts(&mut self, tts: &str) -> Result<()> {
        self.provider_manager.select_by_name(tts);
        Ok(())
    }
}
