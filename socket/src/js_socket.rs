use async_trait::async_trait;
use futures::{Future, Sink, SinkExt, Stream, StreamExt};
use serde_json::json;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::{log_1, log_2};
use ws_stream_wasm::*;

type EventHandlerReturnType = Result<(), String>;
type EventHandlerFunc<T> = dyn (Fn(T) -> Pin<Box<dyn Future<Output = EventHandlerReturnType>>>);

pub struct EventHandler<T> {
    callback: Rc<EventHandlerFunc<T>>,
}

impl<T> EventHandler<T> {
    pub fn wrap<F, G>(func: F) -> Self
    where
        F: (Fn(T) -> G) + 'static,
        G: Future<Output = EventHandlerReturnType> + 'static,
    {
        let callback: Rc<EventHandlerFunc<T>> = Rc::new(move |args| {
            let future: Pin<Box<dyn Future<Output = EventHandlerReturnType>>> =
                Box::pin(func(args));
            future
        });

        EventHandler { callback }
    }
}

impl<T> std::ops::Deref for EventHandler<T> {
    type Target = EventHandlerFunc<T>;

    fn deref(&self) -> &Self::Target {
        &*self.callback
    }
}

#[async_trait(?Send)]
pub trait Socket {
    type EventType: Clone;
    type EventContext: Clone;
    type EventArgs: Clone;
    type EventHandler: Deref<Target = EventHandlerFunc<(Self::EventContext, Self::EventArgs)>>;

    fn on(&self, event_type: Self::EventType, callback: Self::EventHandler);
    async fn emit(
        &self,
        event_type: Self::EventType,
        event_args: Self::EventArgs,
    ) -> Result<(), WsErr>;
}

pub struct JsMeta(Option<WsMeta>);

impl std::ops::Deref for JsMeta {
    type Target = Option<WsMeta>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Drop for JsMeta {
    fn drop(&mut self) {
        let ws_meta = std::mem::take(&mut self.0);
        spawn_local(async move {
            log_1(&"Closing socket...".into());
            if let Some(ws_meta) = ws_meta {
                if ws_meta.ready_state() == WsState::Open {
                    ws_meta.close().await.unwrap();
                }
            }
        })
    }
}

#[derive(Clone)]
pub struct JsSocket {
    id: Rc<RefCell<String>>,

    ws_meta: Rc<RefCell<JsMeta>>,
    ws_send: Rc<RefCell<Box<dyn Sink<WsMessage, Error = WsErr> + Unpin>>>,
    ws_recv: Rc<RefCell<Box<dyn Stream<Item = WsMessage> + Unpin>>>,

    handlers: Rc<RefCell<HashMap<String, Vec<<Self as Socket>::EventHandler>>>>,
}

impl JsSocket {
    pub fn new(ws_meta: WsMeta, ws_stream: WsStream) -> Self {
        let (send, mut recv) = ws_stream.split();
        let handlers: HashMap<String, Vec<<Self as Socket>::EventHandler>> = HashMap::new();

        let socket_id = "".to_owned();
        let mut new_self = Self {
            id: Rc::new(RefCell::new(socket_id.clone())),
            ws_meta: Rc::new(RefCell::new(JsMeta(Some(ws_meta)))),
            ws_send: Rc::new(RefCell::new(Box::new(send))),
            ws_recv: Rc::new(RefCell::new(Box::new(recv))),

            handlers: Rc::new(RefCell::new(handlers)),
        };

        new_self.start();

        new_self
    }

    pub fn start(&self) {
        let s = self.clone();

        self.on(
            "connect".to_owned(),
            EventHandler::wrap(|(socket, args): (JsSocket, serde_json::Value)| async move {
                let id = args
                    .get("id")
                    .ok_or_else(|| "No connection id specified.")?
                    .as_str()
                    .ok_or_else(|| "Connection id could not be coerced to string")?
                    .to_owned();
                *socket.id.borrow_mut() = id;

                Ok(())
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
                log_2(
                    &format!("Received: {}", event_type).into(),
                    &JsValue::from_serde(&event_args).unwrap(),
                );
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
        self.handlers.borrow()
    }

    pub fn handlers_mut(
        &self,
    ) -> impl DerefMut<Target = HashMap<String, Vec<<Self as Socket>::EventHandler>>> + '_ {
        self.handlers.borrow_mut()
    }

    pub fn id(&self) -> impl Deref<Target = String> + '_ {
        self.id.borrow()
    }

    pub async fn recv(&self) -> Option<WsMessage> {
        self.ws_recv.borrow_mut().next().await
    }

    pub async fn send(&self, message: WsMessage) -> Result<(), WsErr> {
        self.ws_send.borrow_mut().send(message).await
    }
}

#[async_trait(?Send)]
impl Socket for JsSocket {
    type EventType = String;
    type EventContext = JsSocket;
    type EventArgs = serde_json::Value;
    type EventHandler = EventHandler<(Self::EventContext, Self::EventArgs)>;

    async fn emit(
        &self,
        event_type: Self::EventType,
        event_args: Self::EventArgs,
    ) -> Result<(), WsErr> {
        self.send(WsMessage::from(
            serde_json::to_string(&json!({
                "type": event_type,
                "args": event_args
            }))
            .unwrap(),
        ))
        .await
    }

    fn on(&self, event_type: Self::EventType, callback: Self::EventHandler) {
        let mut h = self.handlers_mut();
        let hs = h.entry(event_type).or_insert_with(|| Vec::new());
        (*hs).push(callback);
    }
}
