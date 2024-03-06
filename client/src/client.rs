use std::rc::Rc;
use std::time::Duration;

use futures::select;
use futures_util::{
    FutureExt,
    SinkExt, stream::{SplitSink, SplitStream}, StreamExt,
};
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage};
use gloo_timers::future::sleep;
use log::{error, info};
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::RefCell;
use crate::rpc::RpcCaller;
use crate::status::VolatileStatus;

pub struct Client {
    url: String,
    tx: Option<RefCell<SplitSink<WebSocket, WsMessage>>>,
    status: Rc<RefCell<VolatileStatus>>,
    rpc_callers: Vec<Rc<RefCell<dyn RpcCaller>>>,
    caller_tx: Sender<Vec<u8>>,
    _caller_tx_rx: RefCell<Receiver<Vec<u8>>>,
}

impl Client {
    pub(crate) fn new(url: String, status: Rc<RefCell<VolatileStatus>>) -> Self {
        let (tx, rx) = channel::<Vec<u8>>(32);
        Self {
            url,
            tx: None,
            status,
            rpc_callers: vec![],
            caller_tx: tx,
            _caller_tx_rx: RefCell::new(rx),
        }
    }

    pub(crate) fn add_service(&mut self, caller: Rc<RefCell<dyn RpcCaller>>) {
        caller.borrow_mut().set_sender(self.caller_tx.clone());
        self.rpc_callers.push(caller);
    }

    pub(crate) async fn connect(&mut self) -> Result<(), String> {
        let ws = WebSocket::open(&self.url).map_err(|e| {
            let msg = format!("failed to connect to {}: {e}", self.url);
            error!("{msg}");
            msg
        })?;
        let (tx, mut rx) = ws.split();
        self.tx.replace(RefCell::new(tx));

        // ws.close()
        self.status.borrow_mut().connected = true;
        let _ = select! {
            r = self.handle(&mut rx).fuse() => r,
            r = self.wait_cancel().fuse() => r,
            r = self.send_bytes().fuse() => r,
            r = self.test_loop().fuse() => r
        };
        info!("handle return!");
        self.status.borrow_mut().connected = false;
        Ok(())
    }

    async fn wait_cancel(&self) -> Result<(), String> {
        self.status.borrow().close_notify.notified().await;
        Ok(())
    }

    async fn test_loop(&self) -> Result<(), String> {
        loop {
            sleep(Duration::from_secs(1)).await;
            info!("client loop running...");
        }
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
                WsMessage::Text(s) => {
                    info!("received text: {s}");
                }
                WsMessage::Bytes(b) => {
                    info!("received bytes, len: {}", b.len());
                    let mut matched = false;
                    for i in self.rpc_callers.iter() {
                        matched |= i.borrow_mut().handle(&b);
                        if matched {
                            break;
                        }
                    }
                    if !matched {
                        error!("no rpc caller matched, ignoring msg...");
                    }
                }
            }
        }
        Ok(())
    }

    async fn send_bytes(&self) -> Result<(), String> {
        while let Some(s) = self._caller_tx_rx.borrow_mut().recv().await {
            if let Some(tx) = self.tx.as_ref() {
                tx.borrow_mut()
                    .send(WsMessage::Bytes(s.to_vec()))
                    .await
                    .map_err(|e| format!("failed to send bytes: {e:?}, closing connection..."))?;
            }
        }
        Ok(())
    }
}
