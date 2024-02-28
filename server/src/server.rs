use std::net::SocketAddr;

use bytes::Bytes;
use futures_util::{SinkExt, StreamExt};
use prost::Message as _;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use tracing::{debug, error, info};

use proto_gen::control::{Hello, HelloReply};

use crate::errors::AppErrors;

pub(crate) async fn echo_handler(
    mut stream: WebSocketStream<TcpStream>,
    client_addr: SocketAddr,
) -> Result<(), AppErrors> {
    let hello = Hello {
        from: "server".to_string(),
        to: client_addr.to_string(),
    }
    .encode_to_vec();

    stream
        .send(Message::binary(hello))
        .await
        .map_err(|e| format!("failed to send message, err: {e}"))?;
    let rpl = stream
        .next()
        .await
        .ok_or_else(|| "no hello reply message")?
        .map_err(|e| format!("failed to receive hello reply message, err: {e}"))?;
    let hello_reply = HelloReply::decode(Bytes::from(rpl.into_data()))
        .map_err(|e| format!("failed to decode hello reply message, err: {e}"))?;
    info!("hello reply: {:?}", hello_reply);

    while let Some(msg) = stream.next().await {
        debug!("received message: {:?}", msg);
        if let Ok(msg) = msg {
            if msg.is_close() || msg.is_empty() || msg.is_ping() || msg.is_pong() {
                break;
            }
        } else {
            error!("failed to echo message, err: {msg:?}");
            continue;
        }
    }

    Ok(())
}
