use std::{env, sync::Arc};

use futures::future::poll_fn;
use reqwest_websocket::Message;
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::service::tts::{
    providers::{kokoro::KokoroTTS, qwen::QWenTTS},
    service::{TTSEvent, TTSPlayRequest, TTSService},
};

mod kokoro;
mod qwen;

#[derive(Clone, Debug)]
pub enum TTSProvider {
    QWEN { name: String, tts: QWenTTS },
    KOKORO { name: String, tts: KokoroTTS },
}

pub enum EventLoopRet {
    Disconnected,
    Close,
}

impl TTSProvider {
    pub fn prepare_play_message(&self, req: TTSPlayRequest) -> Vec<Message> {
        match self {
            TTSProvider::QWEN { tts, .. } => tts.prepare_play_message(req),
            TTSProvider::KOKORO { tts, .. } => tts.prepare_play_message(req),
        }
    }

    pub fn event_loop(
        &self,
        event_tx: tokio::sync::broadcast::Sender<TTSEvent>,
        ws_msg_rx: Arc<tokio::sync::Mutex<UnboundedReceiver<Message>>>,
    ) {
        match self {
            TTSProvider::QWEN { tts, .. } => tts.event_loop(event_tx, ws_msg_rx),
            TTSProvider::KOKORO { tts, .. } => tts.event_loop(event_tx, ws_msg_rx),
        }
    }
    pub fn is_ready(&self) -> bool {
        match self {
            TTSProvider::QWEN { tts, .. } => tts.is_ready(),
            TTSProvider::KOKORO { tts, .. } => tts.is_ready(),
        }
    }
    pub fn close(&self) {
        match self {
            TTSProvider::QWEN { tts, .. } => tts.close(),
            TTSProvider::KOKORO { tts, .. } => tts.close(),
        }
    }
}

trait Provider {
    fn prepare_play_message(&self, req: TTSPlayRequest) -> Vec<Message>;
    fn is_ready(&self) -> bool;
    fn close(&self) {
        TTSService::close().unwrap();
    }
    fn event_loop(
        &self,
        event_tx: tokio::sync::broadcast::Sender<TTSEvent>,
        ws_msg_rx: Arc<tokio::sync::Mutex<UnboundedReceiver<Message>>>,
    );
}

#[derive(Clone)]
pub struct TTSProviderManager {
    providers: Vec<Option<TTSProvider>>,
    selected: Option<TTSProvider>,
    tts_event_sender: tokio::sync::broadcast::Sender<TTSEvent>,
    ws_reciver: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<Message>>>,
}

impl TTSProviderManager {
    pub fn init(
        tts_event_sender: tokio::sync::broadcast::Sender<TTSEvent>,
        ws_reciver: mpsc::UnboundedReceiver<Message>,
    ) -> Self {
        let tts = QWenTTS::builder()
            .api_key(env::var("VITE_DASHSCOPE_API_KEY").unwrap())
            .build()
            .unwrap();
        Self {
            providers: vec![
                TTSProvider::KOKORO {
                    name: "Kokoro".into(),
                    tts: KokoroTTS::default(),
                },
                TTSProvider::QWEN {
                    name: "QWen".into(),
                    tts,
                },
            ]
            .into_iter()
            .map(|t| Some(t))
            .collect(),
            selected: None,
            tts_event_sender,
            ws_reciver: Arc::new(tokio::sync::Mutex::new(ws_reciver)),
        }
    }
    fn close_selected(&mut self) {
        let s = self.selected.take();
        if let Some(s) = s {
            s.close();
            self.providers
                .iter_mut()
                .find(|p| p.is_none())
                .unwrap()
                .replace(s);
        }
    }
    pub fn select_by_name(&mut self, tts_name: &str) {
        // self.selected =
        self.close_selected();
        self.selected = self
            .providers
            .iter_mut()
            .find(|p| {
                p.as_ref().map_or(false, |provider| match provider {
                    TTSProvider::QWEN { name, .. } => name == tts_name,
                    TTSProvider::KOKORO { name, .. } => name == tts_name,
                })
            })
            .unwrap()
            .take();
        self.selected.as_ref().map(|provider| {
            if !provider.is_ready() {
                provider.event_loop(self.tts_event_sender.clone(), self.ws_reciver.clone());
            }
        });
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

    use std::{path::Path, time::Duration};

    use crate::device::{audio::AudioDevice, frontend::FClient};

    use super::*;
    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
        dotenvy::from_path(Path::new("../.env.local")).unwrap();
        FClient::init(None);
    }

    #[tokio::test]
    async fn test_switch_tts() -> anyhow::Result<()> {
        init();
        let mut servise = TTSService::init().unwrap();
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = Arc::new(rodio::Sink::connect_new(&stream_handle.mixer()));
        let mut receiver = servise.subscribe();
        servise.switch_tts("Kokoro")?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        servise.switch_tts("QWen")?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        servise
            .play(TTSPlayRequest {
                id: "()".to_string(),
                input: "你好".to_string(),
            })
            .unwrap();
        while let Ok(event) = receiver.recv().await {
            match event {
                crate::service::tts::service::TTSEvent::Play(res) => {
                    log::debug!("received event TTSEvent::Play");
                    let source = AudioDevice::pcm_bytes_to_source(&res.data);
                    sink.append(source);
                }
                _ => {}
            }
        }
        Ok(())
    }
}
