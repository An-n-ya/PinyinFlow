use crate::{
    commands::PlayResond,
    device::{
        frontend::{FClient, FEvent},
        websocket::{WsClient, WsEvent},
    },
};
use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use rodio::Source;
use std::{
    io::Cursor,
    sync::{mpsc::Sender, OnceLock},
};
use tauri::{AppHandle, Emitter};
pub struct AudioDevice {}

static AUDIO_SINK: OnceLock<Sender<AudioRequest>> = OnceLock::new();
enum AudioRequest {
    Play { data: Vec<u8>, id: u32 },
}

#[derive(serde::Serialize, Clone)]
#[serde(rename_all = "camelCase")]
struct AudioPlayed {
    id: u32,
}

impl AudioDevice {
    pub fn init(app: AppHandle) -> Result<()> {
        let (tx, rx) = std::sync::mpsc::channel::<AudioRequest>();
        AUDIO_SINK
            .set(tx)
            .map_err(|_| anyhow::anyhow!("already initialized"))?;
        std::thread::spawn(move || {
            for request in rx {
                let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
                    .expect("open default audio stream");
                match request {
                    AudioRequest::Play {
                        data: pcm_bytes,
                        id,
                    } => {
                        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
                        let source = Self::pcm_bytes_to_source(&pcm_bytes);
                        sink.append(source);
                        // 等待播放完成
                        sink.sleep_until_end();
                        FClient::send_event(FEvent::AudioPlayed { id });
                    }
                }
            }
        });
        Ok(())
    }
    pub fn listen(ws_client: &WsClient) {
        let ws_client_clone = ws_client.clone();
        tauri::async_runtime::spawn(async move {
            let mut receiver = ws_client_clone.subscribe();
            while let Ok(event) = receiver.recv().await {
                match event {
                    WsEvent::Play(res) => {
                        Self::play_pcm_bytes(&res);
                    }
                    _ => {}
                }
            }
        });
    }
    pub fn play_pcm_bytes(res: &PlayResond) {
        AUDIO_SINK
            .get()
            .unwrap()
            .send(AudioRequest::Play {
                data: res.data.to_vec(),
                id: res.id,
            })
            .expect("audio sink channel");
    }
    fn pcm_bytes_to_source(pcm_bytes: &[u8]) -> impl Source<Item = f32> {
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
