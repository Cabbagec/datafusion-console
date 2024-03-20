use std::net::SocketAddr;
use std::path::PathBuf;
use std::process::ExitCode;

use datafusion::execution::{
    memory_pool::{FairSpillPool, GreedyMemoryPool},
    runtime_env::{RuntimeConfig, RuntimeEnv},
};
use futures_util::stream::IntoAsyncRead;
use futures_util::TryStreamExt;
use regex;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};
use tokio_tungstenite::{accept_async, WebSocketStream};
use tracing::{debug, error, info};
use tracing_subscriber::util::SubscriberInitExt;

use errors::AppErrors;
use server::hello_service_handler;

use crate::assets::{GeneratedAssets, StaticAssets};

mod assets;
mod config;
mod errors;
mod messages;
mod server;

pub async fn serve() -> Result<(), AppErrors> {
    let bind = "0.0.0.0:8081";
    let bind_socket = TcpListener::bind(bind)
        .await
        .map_err(|e| format!("failed to bind to {bind}"))?;
    for f in GeneratedAssets::iter() {
        info!("serving {f}");
    }

    for r in StaticAssets::iter() {
        info!("serving {r}");
    }

    debug!("starting dashboard server on ws://{bind}");
    'outer: while let Ok((mut stream, _)) = bind_socket.accept().await {
        let mut peek_buf: [u8; 512] = [0; 512];
        let Ok(peek_size) = stream
            .peek(&mut peek_buf)
            .await else {
            error!("failed to peek"); 
            continue;
        };
        let peek = String::from_utf8_lossy(&peek_buf[..peek_size]);
        debug!("peek: {peek}");
        if peek.contains("Upgrade: websocket") {
            let Ok((ws, endpoint)) = handshake(stream).await else {
                error!("failed to handshake");
                continue;
            };
            tokio::spawn(hello_service_handler(ws, endpoint));
        } else {
            // if peek.contains("\nUpgrade: ") {
            //     error!("unsupported upgrade: {peek}");
            //     continue;
            // }

            info!("proceed to serve static files");
            let buf_reader = BufReader::new(&mut stream);
            let mut req_lines = buf_reader.lines();
            let mut lines = vec![];
            loop {
                let Ok(line) = req_lines.next_line().await else {
                    error!("failed to read next req line");
                    continue 'outer;
                };
                if let Some(line) = line {
                    if line.is_empty() {
                        break;
                    }
                    lines.push(line)
                } else {
                    break;
                }
            }

            let Some(method) = lines.first() else {
                error!("failed to read req method: request is empty");
                continue;
            };

            // serve static files
            let path_regex = regex::Regex::new(r"GET (.*?) HTTP").expect("failed to compile regex");
            let Some(path) = path_regex
                .captures(&method)
                .map(|m| m.get(1).unwrap().as_str()) else {
                error!("no request method and path found, request: {lines:?}");
                continue
            };
            debug!("path str: {path}");
            let path_buf = PathBuf::from(path);
            if path_buf == PathBuf::from("/") {
                // serve index.html
                if let Err(e) = serve_file("index.html", &mut stream).await {
                    error!("{e:?}");
                    continue;
                };
            } else {
                let file_path = path_buf
                    .strip_prefix("/")
                    .unwrap_or(path_buf.as_path())
                    .to_str()
                    .unwrap_or("index.html");
                if let Err(e) = serve_file(file_path, &mut stream).await {
                    error!("{e:?}");
                    continue;
                };
            }
        }
        debug!("finished serving");
    }
    Ok(())
}

pub async fn serve_file(
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

pub async fn handshake(
    stream: TcpStream,
) -> Result<(WebSocketStream<TcpStream>, SocketAddr), AppErrors> {
    let peer_addr = if let Ok(peer) = stream.peer_addr() {
        info!("new client from {}", peer);
        peer
    } else {
        Err("failed to get peer addr from stream")?
    };
    let ws_stream = accept_async(stream)
        .await
        .map_err(|e| format!("failed to create ws stream, err: {e}"))?;
    Ok((ws_stream, peer_addr))
}

#[tokio::main]
async fn main() -> ExitCode {
    let c = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .finish();
    c.try_init().expect("failed to install tracing");

    if let Err(e) = serve().await {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
