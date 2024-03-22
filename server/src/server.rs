use std::net::SocketAddr;

use futures_util::{SinkExt, StreamExt};
use prost::Message as _;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use tracing::{error, info};

use proto_gen::control::{Hello, hello_service::Service, HelloService};

use crate::assets::{GeneratedAssets, StaticAssets};
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

pub async fn http_serve_file(
    rel_path: impl AsRef<str>,
    stream: &mut TcpStream,
) -> Result<(), AppErrors> {
    let rel_path = rel_path.as_ref();
    let header;
    let file = if let Some(file) = GeneratedAssets::get(rel_path) {
        let content_type = file.metadata.mimetype();
        let content_length = file.data.len();
        header = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n",
            content_type, content_length
        );
        file
    } else {
        error!("failed to get asset: {rel_path}");
        let f = StaticAssets::get("404.html").expect("failed to get 404.html");
        header = format!(
            "HTTP/1.1 404 Not Found\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
            f.data.len()
        );
        f
    };

    stream
        .write(header.as_bytes())
        .await
        .map_err(|e| format!("failed to write header: {e}"))?;
    stream
        .write_all(&file.data)
        .await
        .map_err(|e| format!("failed to write body: {e}"))?;
    Ok(())
}
