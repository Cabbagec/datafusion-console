use std::rc::Rc;

use log::error;
use prost::Message;
use tokio::sync::mpsc::Sender;

use proto_gen::control::{hello_service::Service, HelloReply, HelloService};

use crate::{spawn_local, RefCell};

pub trait RpcCaller {
    // fn add_client(&self, client: &Client);
    fn set_sender(&mut self, sender: Sender<Vec<u8>>);
    fn handle(&mut self, bytes: &[u8]) -> bool;
}

#[derive(Default)]
pub struct HelloRpc {
    sender: Option<Rc<RefCell<Sender<Vec<u8>>>>>,
}

impl HelloRpc {
    fn new() -> Self {
        Default::default()
    }

    pub fn say_hello(&self) {
        if let Some(s) = self.sender.as_ref() {
            let m = HelloService {
                service: Some(Service::HelloReplyMsg(HelloReply {
                    message: "hi from wasm".to_string(),
                })),
            }
            .encode_to_vec();
            let ss = s.clone();
            spawn_local(async move {
                let _ = ss.borrow().send(m).await.map_err(|e| {
                    let msg = format!("failed to say hello, send err: {e:?}");
                    error!("{msg}");
                });
            });
        }
    }
}

impl RpcCaller for HelloRpc {
    fn set_sender(&mut self, sender: Sender<Vec<u8>>) {
        self.sender.replace(Rc::new(RefCell::new(sender)));
    }

    fn handle(&mut self, bytes: &[u8]) -> bool {
        let m = HelloService::decode(bytes);
        true
    }

    // fn add_client(&self, client: &Client) {
    //     client.add_caller()
    // }
}
