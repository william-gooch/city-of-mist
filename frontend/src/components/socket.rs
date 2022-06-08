use crate::state::*;
use common::character::Character;
use send_wrapper::SendWrapper;
use serde::{Deserialize, Serialize};
use serde_json::json;
use socket::js::*;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::*, JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use web_sys::console::{log_1, log_2};
use ws_stream_wasm::WsMeta;
use yew_agent::{Agent, AgentLink, Context, HandlerId};
use yewdux::prelude::*;

#[derive(Clone)]
pub enum SocketState {
    Noop,
    InitSocket(JsSocket),
    ServerCharacterUpdate { character: Character },
    ClientCharacterUpdate { character: Character },
}

#[derive(Serialize, Deserialize)]
pub struct SocketMessage(pub String, pub serde_json::Value);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SocketEvent {
    Connect { id: String },
    DiceRoll { seed: u64 },
}

#[derive(Clone)]
pub struct SocketConnection {
    state_dispatch: Dispatch<BasicStore<State>>,
    link: SendWrapper<AgentLink<Self>>,
    socket: Option<JsSocket>,
    connections: Rc<RefCell<HashSet<HandlerId>>>,
}

impl SocketConnection {
    async fn try_connect() -> Option<JsSocket> {
        let (ws_meta, ws_stream) = WsMeta::connect("ws://localhost:3030/ws", None)
            .await
            .ok()?;
        let socket = JsSocket::new(ws_meta, ws_stream);
        Some(socket)
    }

    async fn retry_connect() -> JsSocket {
        loop {
            if let Some(socket) = Self::try_connect().await {
                log_1(&"Connected!".into());
                break socket;
            } else {
                log_1(&"Couldn't connect, retrying...".into());
                gloo_timers::future::TimeoutFuture::new(100).await;
            }
        }
    }

    fn respond_callback(
        &self,
        callback: impl Fn((JsSocket, serde_json::Value)) -> <Self as Agent>::Output
            + Send
            + Sync
            + 'static,
    ) -> EventHandler<(JsSocket, serde_json::Value)> {
        let this = self.clone();
        EventHandler::wrap(move |(socket, args): (JsSocket, serde_json::Value)| {
            let connections = this.connections.clone();
            let link = this.link.clone();
            let msg = callback((socket, args));
            async move {
                for conn in connections.borrow().iter() {
                    link.respond(*conn, msg.clone());
                }

                Ok(())
            }
        })
    }

    fn state_callback(
        &self,
        callback: impl Fn((JsSocket, serde_json::Value)) -> <Self as Agent>::Message
            + Send
            + Sync
            + 'static,
    ) -> EventHandler<(JsSocket, serde_json::Value)> {
        let this = self.clone();
        EventHandler::wrap(move |(socket, args): (JsSocket, serde_json::Value)| {
            let connections = this.connections.clone();
            let link = this.link.clone();
            let msg = callback((socket, args));
            async move {
                for conn in connections.borrow().iter() {
                    link.send_message(msg.clone());
                }

                Ok(())
            }
        })
    }

    fn setup_events(&mut self) {
        if let Some(socket) = &self.socket {
            socket.on(
                "connect".to_owned(),
                self.respond_callback(|(socket, _args)| {
                    let id = socket.id().clone();
                    SocketEvent::Connect { id: id.clone() }
                }),
            );
            socket.on(
                "dice".to_owned(),
                self.respond_callback(move |(_socket, args)| {
                    let seed = args.get("seed").unwrap().as_u64().unwrap();
                    SocketEvent::DiceRoll { seed }
                }),
            );
            socket.on(
                "character/update".to_owned(),
                self.state_callback(move |(_socket, args)| {
                    let character: Character =
                        serde_json::from_value(args.get("character").unwrap().to_owned()).unwrap();
                    SocketState::ServerCharacterUpdate { character }
                }),
            );
        }
    }

    fn on_connected(connections: &Rc<RefCell<HashSet<HandlerId>>>, id: HandlerId) {
        if let Ok(mut conns) = connections.try_borrow_mut() {
            conns.insert(id);
        } else {
            let connections = connections.clone();
            let callback = Closure::wrap(
                Box::new(move || Self::on_connected(&connections, id)) as Box<dyn Fn()>
            );
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback(callback.as_ref().unchecked_ref());
            callback.forget();
        }
    }

    fn on_disconnected(connections: &Rc<RefCell<HashSet<HandlerId>>>, id: HandlerId) {
        if let Ok(mut conns) = connections.try_borrow_mut() {
            conns.remove(&id);
        } else {
            let connections = connections.clone();
            let callback = Closure::wrap(
                Box::new(move || Self::on_disconnected(&connections, id)) as Box<dyn Fn()>
            );
            web_sys::window()
                .unwrap()
                .set_timeout_with_callback(callback.as_ref().unchecked_ref());
            callback.forget();
        }
    }
}

impl Agent for SocketConnection {
    type Reach = Context<Self>;
    type Message = SocketState;
    type Input = SocketMessage;
    type Output = SocketEvent;

    fn create(link: AgentLink<Self>) -> Self {
        link.send_future(async move {
            let socket = Self::retry_connect().await;
            SocketState::InitSocket(socket)
        });

        Self {
            state_dispatch: Dispatch::bridge_state(link.callback(|state: Rc<State>| {
                if let Some(character) = state.character.as_ref() {
                    // SocketState::ClientCharacterUpdate {
                    //     character: character.clone(),
                    // }
                    SocketState::Noop
                } else {
                    SocketState::Noop
                }
            })),
            link: SendWrapper::new(link),
            socket: None,
            connections: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    fn update(&mut self, msg: Self::Message) {
        match msg {
            SocketState::Noop => (),
            SocketState::InitSocket(socket) => {
                log_1(&"socket initialized!".into());
                self.socket = Some(socket);
                self.setup_events();
                self.link
                    .send_input(SocketMessage("character".to_owned(), json!({ "cid": 1 })));
            }
            SocketState::ServerCharacterUpdate { character } => {
                self.state_dispatch.reduce(|state| {
                    state.character = Some(character);
                })
            }
            SocketState::ClientCharacterUpdate { character } => {
                self.link.send_input(SocketMessage(
                    "character/update".to_owned(),
                    json!({ "cid": 1, "character": character }),
                ));
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _id: HandlerId) {
        let SocketMessage(event_type, event_args) = msg;
        log_2(
            &format!("Sent: {}", event_type).into(),
            &JsValue::from_serde(&event_args).unwrap(),
        );
        if let Some(s) = &self.socket {
            let s = s.clone();
            spawn_local(async move {
                s.emit(event_type, event_args).await;
            });
        }
    }

    fn connected(&mut self, id: HandlerId) {
        Self::on_connected(&self.connections, id);
    }

    fn disconnected(&mut self, id: HandlerId) {
        Self::on_disconnected(&self.connections, id);
    }
}
