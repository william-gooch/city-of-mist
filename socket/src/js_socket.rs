use crate::*;
use futures::{Sink, SinkExt, Stream, StreamExt};
use parking_lot::{Mutex, RwLock};
use serde_json::json;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use wasm_bindgen::closure::WasmClosure;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use web_sys::WebSocket;
use ws_stream_wasm::*;

#[derive(Clone)]
pub struct JsSocket {
    id: Arc<RwLock<String>>,

    ws_meta: Arc<Mutex<WsMeta>>,
    ws_send: Arc<Mutex<Box<dyn Sink<WsMessage, Error = WsErr> + Send + Sync + Unpin>>>,
    ws_recv: Arc<Mutex<Box<dyn Stream<Item = WsMessage> + Send + Sync + Unpin>>>,

    handlers: Arc<RwLock<HashMap<String, Vec<<Self as Socket>::EventHandler>>>>,
}

impl JsSocket {
    pub fn new(ws_meta: WsMeta, ws_stream: WsStream) -> Self {
        let (send, mut recv) = ws_stream.split();
        let handlers: HashMap<String, Vec<<Self as Socket>::EventHandler>> = HashMap::new();

        let socket_id = "".to_owned();
        let mut new_self = Self {
            id: Arc::new(RwLock::new(socket_id.clone())),
            ws_meta: Arc::new(Mutex::new(ws_meta)),
            ws_send: Arc::new(Mutex::new(Box::new(send))),
            ws_recv: Arc::new(Mutex::new(Box::new(recv))),

            handlers: Arc::new(RwLock::new(handlers)),
        };

        new_self.start();

        new_self
    }

    pub fn start(&self) {
        let s = self.clone();

        self.on(
            "connect".to_owned(),
            EventHandler::wrap(|(socket, args): (JsSocket, serde_json::Value)| async move {
                let id = args.get("id").unwrap().as_str().unwrap().to_owned();
                *socket.id.write() = id;
            }),
        );

        spawn_local(s.on_message());
    }

    // async fn on_connect(mut self) {
    //     if let Some(WsMessage::Text(msg)) = self.recv().await {
    //         let data: serde_json::Value = serde_json::from_str(&msg).map_err(|e| format!("JSON error: {}", e)).unwrap();
    //         let event_type = data.get("type").unwrap().as_str().unwrap().to_owned();
    //         let id = data.get("args").unwrap().get("id").unwrap().as_str().unwrap().to_owned();
    //
    //
    //         self.id = id;
    //         log_1(&self.id.clone().into());
    //         self.on_message().await;
    //     }
    // }

    async fn on_message(self) {
        while let Some(WsMessage::Text(msg)) = self.recv().await {
            let s = self.clone();
            let res: Result<(), String> = (|| async move {
                let data: serde_json::Value =
                    serde_json::from_str(&msg).map_err(|e| format!("JSON error: {}", e))?;
                let event_type = data.get("type").unwrap().as_str().unwrap().to_owned();
                let event_args = data.get("args").unwrap().to_owned();
                let h = s.handlers();
                if let Some(hs) = h.get(&event_type[..]) {
                    futures::future::join_all(hs.into_iter().map(|handler| {
                        let event_context = s.clone();
                        let event_args = event_args.clone();
                        async move {
                            handler((event_context, event_args)).await;
                        }
                    }))
                    .await;
                }
                Ok(())
            })()
            .await;
        }
    }

    pub fn handlers(
        &self,
    ) -> impl Deref<Target = HashMap<String, Vec<<Self as Socket>::EventHandler>>> + '_ {
        self.handlers.read()
    }

    pub fn handlers_mut(
        &self,
    ) -> impl DerefMut<Target = HashMap<String, Vec<<Self as Socket>::EventHandler>>> + '_ {
        self.handlers.write()
    }

    pub fn id(&self) -> impl Deref<Target = String> + '_ {
        self.id.read()
    }

    pub async fn recv(&self) -> Option<WsMessage> {
        self.ws_recv.lock().next().await
    }

    pub async fn send(&self, message: WsMessage) -> Result<(), WsErr> {
        self.ws_send.lock().send(message).await
    }
}

#[async_trait]
impl Socket for JsSocket {
    type EventType = String;
    type EventContext = JsSocket;
    type EventArgs = serde_json::Value;
    type EventHandler = EventHandler<(Self::EventContext, Self::EventArgs)>;

    async fn emit(&self, event_type: Self::EventType, event_args: Self::EventArgs) {
        self.send(WsMessage::from(
            serde_json::to_string(&json!({
                "type": event_type,
                "args": event_args
            }))
            .unwrap(),
        ))
        .await
        .unwrap();
    }

    fn on(&self, event_type: Self::EventType, callback: Self::EventHandler) {
        let mut h = self.handlers_mut();
        let hs = h.entry(event_type).or_insert_with(|| Vec::new());
        (*hs).push(callback);
    }
}
