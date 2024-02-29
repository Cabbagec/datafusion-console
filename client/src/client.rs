use futures::select;
use futures_util::stream::SplitStream;
use futures_util::{FutureExt, SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use log::{error, info};

pub struct Client {
    url: String,
}

impl Client {
    pub(crate) fn new(url: String) -> Self {
        Self { url }
    }

    pub(crate) async fn connect(&self) -> Result<(), String> {
        let ws = WebSocket::open(&self.url).map_err(|e| {
            let msg = format!("failed to connect to {}: {e}", self.url);
            error!("{msg}");
            msg
        })?;
        let (mut tx, mut rx) = ws.split();

        // ws.close()
        let _ = select! {
            r = self.handle(&mut rx).fuse() => r,
            r = self.test_cancel().fuse() => r
        };
        // select! {}
        Ok(())
    }
    async fn test_cancel(&self) -> Result<(), String> {
        Ok(())
    }

    async fn handle(&self, rx: &mut SplitStream<WebSocket>) -> Result<(), String> {
        info!("handling...");
        while let Some(msg) = rx.next().await {
            let m = match msg {
                Ok(m) => m,
                Err(e) => {
                    error!("failed to receive message, err: {e}");
                    break;
                }
            };
            match m {
                Message::Text(s) => {
                    info!("received message, len: {s}");
                }
                Message::Bytes(b) => {
                    info!("received bytes, len: {}", b.len());
                }
            }
        }
        Ok(())
    }
}
