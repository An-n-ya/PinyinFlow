use std::sync::OnceLock;
use std::time::Duration;

use anyhow::Result;
use futures_lite::stream::StreamExt;
use futures_util::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use tokio::sync::mpsc;

use crate::play_pcm_from_ws;

static WS_SENDER: OnceLock<mpsc::UnboundedSender<Message>> = OnceLock::new();

/// 重连前等待的秒数
const RECONNECT_DELAY_SECS: u64 = 2;

pub struct WsClient {
    // 用于从外部向服务器发送消息的通道
}

impl WsClient {
    pub fn init(url: &str) -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
        WS_SENDER.set(tx).expect("set sender failed");

        let url = url.to_string();

        // 启动后台任务：连接 -> 消息循环 -> 断开则重连
        tauri::async_runtime::spawn(async move {
            let client = Client::default();

            loop {
                // 建立连接
                let mut websocket = match connect(&client, &url).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        log::error!("WebSocket 连接失败: {}", e);
                        tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                        continue;
                    }
                };

                log::info!("WebSocket 已连接");

                // 消息循环，直到连接断开
                let disconnected = run_message_loop(&mut websocket, &mut rx).await;

                if disconnected {
                    log::warn!("WebSocket 连接已断开，{} 秒后重连...", RECONNECT_DELAY_SECS);
                    tokio::time::sleep(Duration::from_secs(RECONNECT_DELAY_SECS)).await;
                }
            }
        });

        Ok(())
    }

    /// 向服务器发送文本消息
    pub fn send_text(text: String) -> Result<()> {
        WS_SENDER.get().unwrap().send(Message::Text(text.into()))?;
        Ok(())
    }
}

/// 建立 WebSocket 连接
async fn connect(client: &Client, url: &str) -> Result<WebSocket> {
    let response = client.get(url).upgrade().send().await?;
    let websocket = response.into_websocket().await?;
    Ok(websocket)
}

/// 运行消息循环，直到连接断开。返回 true 表示因断开而退出（需要重连）。
async fn run_message_loop(
    websocket: &mut WebSocket,
    rx: &mut mpsc::UnboundedReceiver<Message>,
) -> bool {
    loop {
        tokio::select! {
            msg = websocket.next() => {
                match msg {
                    Some(Ok(message)) => {
                        match message {
                            Message::Ping(payload) => {
                                if let Err(e) = websocket.send(Message::Pong(payload)).await {
                                    log::error!("发送 Pong 失败: {}", e);
                                    return true;
                                }
                            }
                            Message::Text(text) => log::info!("收到消息: {}", text),
                            msg @ Message::Binary(_) => {
                                if let Err(e) = play_pcm_from_ws(msg) {
                                    log::error!("播放 PCM 失败: {}", e);
                                }
                            }
                            Message::Close { code, reason } => {
                                log::info!("服务器关闭连接 {} - {}", code, reason);
                                return true;
                            }
                            _ => {}
                        }
                    }
                    Some(Err(e)) => {
                        log::error!("WebSocket 错误: {}", e);
                        return true;
                    }
                    None => return true,
                }
            }
            Some(outbound_msg) = rx.recv() => {
                if let Err(e) = websocket.send(outbound_msg).await {
                    log::error!("消息发送失败: {}", e);
                    return true;
                }
            }
        }
    }
}

