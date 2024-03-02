use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use prost::Message as _;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use tracing::{error, info};

use proto_gen::control::{Hello, hello_service::Service, HelloService};

use crate::errors::AppErrors;

pub(crate) async fn hello_service_handler(
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

    while let Some(msg) = stream.next().await {
        if let Ok(msg) = msg {
            if msg.is_close() || msg.is_empty() || msg.is_ping() || msg.is_pong() {
                break;
            }
            let hello_msg: HelloService = HelloService::decode(msg.into_data().as_slice())
                .map_err(|e| format!("failed to decode msg as HelloService, decode err: {e:?}"))?;
            let Some(e) = hello_msg.service else {
                Err("no msg found")?
            };
            match e {
                Service::HelloMsg(e) => {
                    info!("hello msg recv: {e:?}");
                }
                Service::HelloReplyMsg(e) => {
                    info!("hello reply recv: {e:?}");
                }
            }
        } else {
            error!("failed to echo message, err: {msg:?}");
            continue;
        }
    }

    Ok(())
}
