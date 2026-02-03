use futures_util::SinkExt;
use tokio::sync::mpsc;
use std::sync::OnceLock;
use futures_lite::stream::StreamExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::Upgrade;
use reqwest_websocket::WebSocket;
use anyhow::Result;

use crate::play_pcm_from_ws;

static WS_SENDER: OnceLock<mpsc::UnboundedSender<Message>> = OnceLock::new();


pub struct WsClient {
    // 用于从外部向服务器发送消息的通道
}

impl WsClient {
    pub fn init(url: &str) -> Result<()> {
        let client = Client::default();

        let mut websocket: Option<WebSocket> = None;

        tauri::async_runtime::block_on(async {
            let response = client.get(url).upgrade().send().await.unwrap();
            websocket = Some(response.into_websocket().await.unwrap());
        });
        let mut websocket = websocket.unwrap();
        // 创建一个通道，用于在结构体外部发送消息到 WebSocket 循环
        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

        // 启动后台任务处理连接
        tauri::async_runtime::spawn(async move {
            loop {
                tokio::select! {
                    // 1. 处理从服务器接收到的消息
                    msg = websocket.next() => {
                        match msg {
                            Some(Ok(message)) => {
                                match message {
                                    Message::Ping(payload) => {
                                        // 自动响应 Pong
                                        if let Err(e) = websocket.send(Message::Pong(payload)).await {
                                            eprintln!("发送 Pong 失败: {}", e);
                                            break;
                                        }
                                    }
                                    Message::Text(text) => println!("收到消息: {}", text),
                                    msg @ Message::Binary(_) => play_pcm_from_ws(msg).expect("play failed"),
                                    Message::Close{code, reason} => {
                                        println!("服务器关闭连接 {code} - {reason}");
                                        break;
                                    }
                                    _ => {}
                                }
                            }
                            Some(Err(e)) => {
                                eprintln!("WebSocket 错误: {}", e);
                                break;
                            }
                            None => break,
                        }
                    }
                    // 2. 处理从本地发送出去的消息
                    Some(outbound_msg) = rx.recv() => {
                        if let Err(e) = websocket.send(outbound_msg).await {
                            eprintln!("消息发送失败: {}", e);
                            break;
                        }
                    }
                }
            }
            println!("WebSocket 任务已退出");
        });

        WS_SENDER.set(tx).expect("set sender failed");

        Ok(())
    }

    /// 向服务器发送文本消息
    pub fn send_text(text: String) -> Result<()> {
        WS_SENDER.get().unwrap().send(Message::Text(text.into()))?;
        Ok(())
    }
}

