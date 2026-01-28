// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use rodio::Source;

fn main() {
    // initialize server
    pinyin_lib::run()
}

// 核心函数：将 PCM 字节流转换为 rodio 音频源（16bit 单声道 44.1kHz 示例）
fn pcm_bytes_to_source(pcm_bytes: &[u8]) -> impl Source<Item = f32> {
    // 1. 将字节流包装为 Cursor（可读取的缓冲区）
    let mut cursor = Cursor::new(pcm_bytes);
    // 2. 解析 16bit 小端 PCM 数据为 i16 采样值（根据实际格式调整 LittleEndian/BigEndian）
    let samples: Vec<f32> = std::iter::from_fn(move || {
        cursor.read_i16::<LittleEndian>().ok().map(|f| f as f32 / 32767.0)
    }).collect();

    // 3. 将采样值转换为 rodio 音频源，设置采样率（44100 Hz）
    rodio::buffer::SamplesBuffer::new(
        1,          // 声道数：1=单声道，2=立体声
        24000,      // 采样率
        samples     // 解析后的 PCM 采样数据
    )
}

#[cfg(test)]
mod test {
    use dollop::split;
    use rodio::{OutputStream, Sink};
    use super::*;
    
    #[test]
    fn test_dollop() {
        let res = split("nihaoya zheshiyigeceshi");
        print!("{res}");
    }
    
    async fn test_websocket() -> Result<String, String> {
        // Extends the `reqwest::RequestBuilder` to allow WebSocket upgrades.
        use reqwest_websocket::RequestBuilderExt;
        use reqwest::Client;
        use reqwest_websocket::Message;
        use futures_util::sink::SinkExt;
        use futures_lite::stream::StreamExt;
        
        // Creates a GET request, upgrades and sends it.
        let response = Client::default()
            .get("ws://localhost:8000/test")
            .upgrade() // Prepares the WebSocket upgrade.
            .send()
            .await.unwrap();

        // Turns the response into a WebSocket stream.
        let mut websocket = response.into_websocket().await.unwrap();

        // The WebSocket implements `Sink<Message>`.
        websocket.send(Message::Text("Hello, World".into())).await;

        // The WebSocket is also a `TryStream` over `Message`s.
        while let Some(message) = websocket.try_next().await.unwrap() {
            if let Message::Text(text) = message {
                println!("received: {text}");
                return Ok(text);
            }
        }
        
        Ok("".to_string())
    }
    
    use tokio::time::sleep;
    use std::time::Duration;

    async fn my_async_function() -> u32 {
        sleep(Duration::from_millis(10)).await;
        42
    }

    #[tokio::test]
    async fn test_async_operation() {
        let result = my_async_function().await;
        let res = test_websocket().await;
        assert_eq!(result, 42);
    }

    async fn pcm_bytes_from_ws() -> Result<Vec<u8>, String> {
        // Extends the `reqwest::RequestBuilder` to allow WebSocket upgrades.
        use reqwest_websocket::RequestBuilderExt;
        use reqwest::Client;
        use reqwest_websocket::Message;
        use futures_util::sink::SinkExt;
        use futures_lite::stream::StreamExt;
        
        // Creates a GET request, upgrades and sends it.
        let response = Client::default()
            .get("ws://localhost:8000/play")
            .upgrade() // Prepares the WebSocket upgrade.
            .send()
            .await.unwrap();

        // Turns the response into a WebSocket stream.
        let mut websocket = response.into_websocket().await.unwrap();

        // The WebSocket implements `Sink<Message>`.
        let _ = websocket.send(Message::Text("ni3 hao3 ya5 zhe4 shi4 yi2 ge4 ce4 shi4".into())).await;

        // The WebSocket is also a `TryStream` over `Message`s.
        while let Some(message) = websocket.try_next().await.unwrap() {
            if let Message::Binary(text) = message {
                return Ok(text.to_vec());
            }
        }
        
        Ok(vec![])
    }
    
    #[tokio::test]
    async fn play_pcm_from_ws() {
        // 生成测试用 PCM 数据：440Hz 正弦波（16bit 单声道 44.1kHz，持续 2 秒）
        let pcm_bytes = pcm_bytes_from_ws().await.unwrap();

        // 初始化音频输出设备
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        // 将 PCM 字节转换为音频源并播放
        let source = pcm_bytes_to_source(&pcm_bytes);
        sink.append(source);

        // 等待播放完成
        sink.sleep_until_end();
    }
    #[test]
    fn play_pcm_from_memory() {
        // 生成测试用 PCM 数据：440Hz 正弦波（16bit 单声道 44.1kHz，持续 2 秒）
        let sample_rate = 24000;
        let duration_sec = 2;
        let mut pcm_bytes = Vec::new();
        for i in 0..(sample_rate * duration_sec) {
            let t = i as f32 / sample_rate as f32;
            let value = (t * 440.0 * 2.0 * std::f32::consts::PI).sin() * i16::MAX as f32;
            let sample = value as i16;
            // 将 i16 转换为小端字节（写入 Vec）
            pcm_bytes.extend_from_slice(&sample.to_le_bytes());
        }

        // 初始化音频输出设备
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
        .expect("open default audio stream");
        let sink = rodio::Sink::connect_new(&stream_handle.mixer());

        // 将 PCM 字节转换为音频源并播放
        let source = pcm_bytes_to_source(&pcm_bytes);
        sink.append(source);

        // 等待播放完成
        sink.sleep_until_end();
    }
}
