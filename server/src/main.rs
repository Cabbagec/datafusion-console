use std::path::PathBuf;
use std::process::ExitCode;

// use datafusion::execution::{
//     memory_pool::{FairSpillPool, GreedyMemoryPool},
//     runtime_env::{RuntimeConfig, RuntimeEnv},
// };
use regex;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    net::TcpListener,
};
use tokio_tungstenite::accept_async;
use tracing::{debug, error, info};
use tracing_subscriber::util::SubscriberInitExt;

use assets::{GeneratedAssets, StaticAssets};
use errors::AppErrors;
use server::{hello_service_handler, http_serve_file};

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
        debug!("serving generated asset: {f}");
    }

    for r in StaticAssets::iter() {
        debug!("serving static asset: {r}");
    }

    info!("starting console/dashboard server on ws://{bind}");
    'outer: while let Ok((mut stream, _)) = bind_socket.accept().await {
        let Ok(peer_addr) = stream.peer_addr() else {
            error!("failed to get peer_addr");
            continue
        };

        info!("client connected from: {peer_addr}");
        let mut peek_buf: [u8; 512] = [0; 512];
        let Ok(peek_size) = stream
            .peek(&mut peek_buf)
            .await else {
            error!("failed to peek"); 
            continue;
        };
        let peek = String::from_utf8_lossy(&peek_buf[..peek_size]);
        if peek.contains("Upgrade: websocket") {
            info!("ws client connected, proceed to handshake");
            let Ok(ws) = accept_async(stream).await else {
                error!("failed to handshake");
                continue;
            };
            tokio::spawn(hello_service_handler(ws, peer_addr.clone()));
        } else {
            info!("http client connected, proceed to serve static files");
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
            let path_buf = PathBuf::from(path);
            debug!("peer: {peer_addr} requested: GET {path}");
            if path_buf == PathBuf::from("/") {
                if let Err(e) = http_serve_file("index.html", &mut stream).await {
                    error!("{e:?}");
                    continue;
                };
            } else {
                let file_path = path_buf
                    .strip_prefix("/")
                    .unwrap_or(path_buf.as_path())
                    .to_str()
                    .unwrap_or("index.html");
                if let Err(e) = http_serve_file(file_path, &mut stream).await {
                    error!("{e:?}");
                    continue;
                };
            }
            info!("finished serving, disconnecting client {peer_addr}");
        }
    }
    Ok(())
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
