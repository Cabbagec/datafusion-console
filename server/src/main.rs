use std::net::SocketAddr;
use std::process::ExitCode;

use datafusion::execution::{
    memory_pool::{FairSpillPool, GreedyMemoryPool},
    runtime_env::{RuntimeConfig, RuntimeEnv},
};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, WebSocketStream};
use tracing::{debug, error, info};
use tracing_subscriber::util::SubscriberInitExt;

use errors::AppErrors;
use server::hello_service_handler;

mod config;
mod errors;
mod messages;
mod server;

pub async fn serve() -> Result<(), AppErrors> {
    let bind = "0.0.0.0:8081";
    let bind_socket = TcpListener::bind(bind)
        .await
        .map_err(|e| format!("failed to bind to {bind}"))?;

    debug!("starting dashboard server on ws://{bind}");
    while let Ok((stream, _)) = bind_socket.accept().await {
        let (ws, endpoint) = handshake(stream).await?;
        tokio::spawn(hello_service_handler(ws, endpoint));
    }
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
