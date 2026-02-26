use crate::{
    commands::PlayRequest,
    device::frontend::{FClient, FEvent},
    service::tts::service::TTSService,
};
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use rodio::Source;
use std::{
    io::Cursor,
    sync::{mpsc::Sender, Arc, OnceLock},
};
use tauri::AppHandle;
pub struct AudioDevice {}

static AUDIO_SINK: OnceLock<Sender<AudioRequest>> = OnceLock::new();
enum AudioRequest {
    Play {
        data: Vec<u8>,
        id: String,
        finished: bool,
    },
}

impl AudioDevice {
    fn check_play_finished(sink: Arc<rodio::Sink>, id: String) {
        tauri::async_runtime::spawn(async move {
            loop {
                // FIXME: maybe there will be more than one checking threads simultaneously.
                if sink.empty() {
                    FClient::send_event(FEvent::AudioPlayed { id });
                    break;
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        });
    }
    pub fn init(_app: AppHandle) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<AudioRequest>();
        AUDIO_SINK
            .set(tx)
            .map_err(|_| anyhow::anyhow!("already initialized"))?;
        std::thread::spawn(move || {
            let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio stream");
            let sink = Arc::new(rodio::Sink::connect_new(&stream_handle.mixer()));
            loop {
                let request = rx.recv().unwrap();
                match request {
                    AudioRequest::Play {
                        data: pcm_bytes,
                        id,
                        finished,
                    } => {
                        if finished {
                            Self::check_play_finished(sink.clone(), id);
                        }
                        let source = Self::pcm_bytes_to_source(&pcm_bytes);
                        sink.append(source);
                    }
                }
            }
        });
        Ok(())
    }
    pub fn listen(tts_service: &TTSService) {
        let tts_service_clone = tts_service.clone();
        tauri::async_runtime::spawn(async move {
            let mut receiver = tts_service_clone.subscribe();
            while let Ok(event) = receiver.recv().await {
                match event {
                    crate::service::tts::service::TTSEvent::Play(res) => {
                        Self::play_pcm_bytes(&res);
                    }
                    crate::service::tts::service::TTSEvent::Finished { id } => {
                        AUDIO_SINK
                            .get()
                            .unwrap()
                            .send(AudioRequest::Play {
                                data: vec![],
                                id,
                                finished: true,
                            })
                            .expect("audio sink channel");
                    }
                    _ => {}
                }
            }
        });
    }
    pub fn play_pcm_bytes(res: &PlayRequest) {
        AUDIO_SINK
            .get()
            .unwrap()
            .send(AudioRequest::Play {
                data: res.data.to_vec(),
                id: res.id.clone(),
                finished: false,
            })
            .expect("audio sink channel");
    }
    pub fn pcm_bytes_to_source(pcm_bytes: &[u8]) -> impl Source<Item = f32> {
        // 1. 将字节流包装为 Cursor（可读取的缓冲区）
        let mut cursor = Cursor::new(pcm_bytes);
        // 2. 解析 16bit 小端 PCM 数据为 i16 采样值（根据实际格式调整 LittleEndian/BigEndian）
        let samples: Vec<f32> = std::iter::from_fn(move || {
            cursor
                .read_i16::<LittleEndian>()
                .ok()
                .map(|f| f as f32 / 32767.0)
        })
        .collect();

        // 3. 将采样值转换为 rodio 音频源，设置采样率（44100 Hz）
        rodio::buffer::SamplesBuffer::new(
            1,       // 声道数：1=单声道，2=立体声
            24000,   // 采样率
            samples, // 解析后的 PCM 采样数据
        )
    }
}
