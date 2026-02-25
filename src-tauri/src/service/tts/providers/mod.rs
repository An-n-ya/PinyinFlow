use reqwest_websocket::Message;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::service::tts::{
    providers::kokoro::KokoroTTS,
    service::{TTSEvent, TTSPlayRequest},
};

mod kokoro;
mod qwen;

#[derive(Clone, Debug)]
pub enum TTSProvider {
    QWEN,
    KOKORO(KokoroTTS),
}

impl Default for TTSProvider {
    fn default() -> Self {
        TTSProvider::KOKORO(KokoroTTS::default())
    }
}

impl TTSProvider {
    pub fn prepare_play_message(&self, req: TTSPlayRequest) -> Message {
        match self {
            TTSProvider::QWEN => todo!(),
            TTSProvider::KOKORO(tts) => tts.prepare_play_message(req),
        }
    }

    pub fn event_loop(
        self,
        event_tx: tokio::sync::broadcast::Sender<TTSEvent>,
        ws_msg_rx: UnboundedReceiver<Message>,
    ) {
        match self {
            TTSProvider::QWEN => todo!(),
            TTSProvider::KOKORO(tts) => tts.event_loop(event_tx, ws_msg_rx),
        }
    }
}

trait Provider {
    fn prepare_play_message(&self, req: TTSPlayRequest) -> Message;
    fn event_loop(
        &self,
        event_tx: tokio::sync::broadcast::Sender<TTSEvent>,
        ws_msg_rx: UnboundedReceiver<Message>,
    );
}
