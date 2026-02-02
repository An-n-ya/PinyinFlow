use byteorder::{LittleEndian, ReadBytesExt};
use rodio::{OutputStream, Sink, Source};
use std::io::Cursor;
pub struct AudioDevice {
    stream: OutputStream,
    sink: Sink
}
impl Default for AudioDevice {
    fn default() -> Self {
        let stream_handle =
            rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());
        Self {
            stream: stream_handle,
            sink
        }
    }
}
impl AudioDevice {
    pub fn play_pcm_bytes(&self, pcm_bytes: &[u8]) {
        let source = Self::pcm_bytes_to_source(pcm_bytes);
        self.sink.append(source);

        // 等待播放完成
        self.sink.sleep_until_end();

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