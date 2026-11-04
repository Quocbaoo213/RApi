use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

use crate::core::contract::event::MessageEvent;
use crate::core::protocol::codec::InboundPacket;

pub type MessageHandler = Arc<dyn Fn(MessageEvent) + Send + Sync>;
pub type GenericHandler = Arc<dyn Fn(serde_json::Value) + Send + Sync>;

#[derive(Default, Clone)]
pub struct PacketDispatcher {
    pending_cmds: Arc<Mutex<HashMap<u16, Vec<Sender<InboundPacket>>>>>,
    msg_handler: Arc<Mutex<Option<MessageHandler>>>,
    event_handlers: Arc<Mutex<HashMap<String, Vec<GenericHandler>>>>,
}

impl PacketDispatcher {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn prepare_cmd(&self, cmd: u16) -> Receiver<InboundPacket> {
        let (tx, rx) = channel();
        let mut map = self.pending_cmds.lock().unwrap();
        map.entry(cmd).or_default().push(tx);
        rx
    }

    pub fn remove_cmd(&self, cmd: u16) {
        let mut map = self.pending_cmds.lock().unwrap();
        map.remove(&cmd);
    }

    pub fn set_message_handler<F>(&self, handler: F)
    where
        F: Fn(MessageEvent) + Send + Sync + 'static,
    {
        let mut h = self.msg_handler.lock().unwrap();
        *h = Some(Arc::new(handler));
    }

    pub fn add_event_handler<F>(&self, event: &str, handler: F)
    where
        F: Fn(serde_json::Value) + Send + Sync + 'static,
    {
        let mut map = self.event_handlers.lock().unwrap();
        map.entry(event.to_string()).or_default().push(Arc::new(handler));
    }

    pub fn dispatch(&self, packet: InboundPacket) {
        if let Some(ref hdr) = packet.header {
            let mut map = self.pending_cmds.lock().unwrap();
            if let Some(senders) = map.get_mut(&hdr.cmd) {
                while let Some(tx) = senders.pop() {
                    let _ = tx.send(packet.clone());
                }
            }
        }

        if let Some(ref data) = packet.data {
            if let Ok(val) = serde_json::to_value(data) {
                let handlers = self.event_handlers.lock().unwrap();
                if let Some(list) = handlers.get("packet") {
                    for h in list {
                        h(val.clone());
                    }
                }
            }
        }
    }
}
