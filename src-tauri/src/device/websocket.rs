use anyhow::Context;
use futures_lite::stream::StreamExt;
use futures_util::sink::SinkExt;
use reqwest::Client;
use reqwest_websocket::Message;
use reqwest_websocket::RequestBuilderExt;
use reqwest_websocket::WebSocket;
use anyhow::Result;

#[derive(Default)]
pub struct WSDevice {
    connection: Option<WebSocket>
}
impl WSDevice {
    async fn ensure_connection(&mut self) -> Result<()> {
        if self.connection.is_none() {
            let response = Client::default()
                .get("ws://localhost:8000/play")
                .upgrade() // Prepares the WebSocket upgrade.
                .send()
                .await?;

            // Turns the response into a WebSocket stream.
            let websocket = response.into_websocket().await.context("websocket connection failed")?;
            self.connection = Some(websocket);
        }
        Ok(())
    }

    pub async fn pcm_bytes(&mut self, pinyin: &str) -> Result<Vec<u8>> {
        self.ensure_connection().await?;
        let websocket = self.connection.as_mut().unwrap();
        websocket.send(Message::Text(pinyin.into())).await.context("websocket send message failed")?;

        // The WebSocket is also a `TryStream` over `Message`s.
        while let Some(message) = websocket.try_next().await.context("websockect receive message failed")? {
            if let Message::Binary(text) = message {
                return Ok(text.to_vec());
            }
        }

        Ok(vec![])
    }
}