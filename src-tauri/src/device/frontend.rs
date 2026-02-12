use std::sync::OnceLock;

use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc;

static EVENT_SENDER: OnceLock<mpsc::UnboundedSender<FEvent>> = OnceLock::new();

#[derive(Clone, Debug, Serialize)]
pub enum FEvent {
    TTSFinished{timestamp: u64, id: String},
    AudioPlayed{id: String}
}


impl FEvent {
    fn name(&self) -> String {
        match self {
            FEvent::TTSFinished { .. } => "tts-finished".to_owned(),
            FEvent::AudioPlayed { .. } => "audio-played".to_owned(),
        }
    }
}

pub struct FClient {

}

impl FClient {
    pub fn init(app: AppHandle) -> Self {
        let (tx, mut rx) = mpsc::unbounded_channel::<FEvent>();
        EVENT_SENDER.set(tx).expect("set front event sender failed");
        tauri::async_runtime::spawn(async move {
            loop {
                if let Some(event) = rx.recv().await {
                    app.emit(&event.name(), event).expect("emit {event:?} failed");
                }
            }
        });
        Self{}
    }
    
    pub fn send_event(e: FEvent) {
        log::info!("got event {e:?}");
        EVENT_SENDER.get().unwrap().send(e).expect("send fevent")
    }
}