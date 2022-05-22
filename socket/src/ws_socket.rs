use async_trait::async_trait;
use futures::Future;
use futures::{Sink, SinkExt, Stream, StreamExt};
use nanoid::nanoid;
use serde_json::json;
use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
use std::ops::DerefMut;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::RwLock;
use warp::ws::{Message, WebSocket};
use warp::Error;

type EventHandlerReturnType = Result<(), String>;
type EventHandlerFunc<T> =
    dyn (Fn(T) -> Pin<Box<dyn Future<Output = EventHandlerReturnType> + Send>>) + Send + Sync;

pub struct EventHandler<T> {
    callback: Arc<EventHandlerFunc<T>>,
}

impl<T> EventHandler<T> {
    pub fn wrap<F, G>(func: F) -> Self
    where
        F: (Fn(T) -> G) + Send + Sync + 'static,
        G: Future<Output = EventHandlerReturnType> + Send + 'static,
    {
        let callback: Arc<EventHandlerFunc<T>> = Arc::new(move |args| {
            let future: Pin<Box<dyn Future<Output = EventHandlerReturnType> + Send>> =
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

#[async_trait]
pub trait Socket {
    type EventType: Clone;
    type EventContext: Clone;
    type EventArgs: Clone;
    type EventHandler: Deref<Target = EventHandlerFunc<(Self::EventContext, Self::EventArgs)>>;

    fn on(&self, event_type: Self::EventType, callback: Self::EventHandler);
    async fn emit(&self, event_type: Self::EventType, event_args: Self::EventArgs);
}

#[derive(Clone)]
pub struct WsSocket {
    id: String,
    ws_send: Arc<Mutex<Box<dyn Sink<Message, Error = Error> + Send + Sync + Unpin>>>,
    ws_recv: Arc<Mutex<Box<dyn Stream<Item = Result<Message, Error>> + Send + Sync + Unpin>>>,

    handlers: Arc<RwLock<HashMap<String, Vec<<Self as Socket>::EventHandler>>>>,
    rooms: Rooms,
}

impl PartialEq for WsSocket {
    fn eq(&self, other: &WsSocket) -> bool {
        self.id == other.id
    }
}
impl Eq for WsSocket {}

impl Hash for WsSocket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl WsSocket {
    pub fn new(ws: WebSocket, rooms: Rooms) -> Self {
        let (send, mut recv) = ws.split();
        let handlers: HashMap<String, Vec<<Self as Socket>::EventHandler>> = HashMap::new();

        let socket_id = nanoid!();
        let new_self = Self {
            id: socket_id.clone(),
            ws_send: Arc::new(Mutex::new(Box::new(send))),
            ws_recv: Arc::new(Mutex::new(Box::new(recv))),

            handlers: Arc::new(RwLock::new(handlers)),
            rooms,
        };

        new_self.join(socket_id);
        new_self.join("*".to_owned());
        new_self.start();

        new_self
    }

    pub fn id(&self) -> &str {
        &self.id[..]
    }

    pub async fn handlers(
        &self,
    ) -> impl Deref<Target = HashMap<String, Vec<<Self as Socket>::EventHandler>>> + '_ {
        self.handlers.read().await
    }

    pub async fn handlers_mut(
        &self,
    ) -> impl DerefMut<Target = HashMap<String, Vec<<Self as Socket>::EventHandler>>> + '_ {
        self.handlers.write().await
    }

    pub async fn recv(&self) -> Option<Result<Message, Error>> {
        self.ws_recv.lock().await.next().await
    }

    pub async fn send(&self, message: Message) -> Result<(), Error> {
        self.ws_send.lock().await.send(message).await
    }

    pub fn start(&self) {
        let s = self.clone();
        tokio::spawn(async move {
            let s = s.clone();
            println!("[{}] Client Connected", s.id());
            while let Some(Ok(msg)) = s.recv().await {
                let s = s.clone();
                let _s = s.clone();
                let res: Result<String, String> = (|| async move {
                    if msg.is_text() {
                        let text = msg.to_str().unwrap();
                        let data: serde_json::Value = serde_json::from_str(&text)
                            .map_err(|e| format!("JSON error: {}", e))?;
                        let event_type = data.get("type").unwrap().as_str().unwrap().to_owned();
                        let event_args = data.get("args").unwrap().to_owned();
                        let h = s.handlers().await;
                        if let Some(hs) = h.get(&event_type[..]) {
                            futures::future::join_all(hs.into_iter().map(|handler| {
                                let event_context = s.clone();
                                let socket = s.clone();
                                let event_args = event_args.clone();
                                async move {
                                    if let Err(err) = handler((event_context, event_args)).await {
                                        socket
                                            .to(socket.id())
                                            .emit("error", json!({ "error": err }))
                                            .await
                                    }
                                }
                            }))
                            .await;
                        }
                        Ok(format!("Received: {}", text.to_owned()))
                    } else if msg.is_close() {
                        Err("Client Disconnected".to_owned())
                    } else {
                        Ok(format!("Received: {:?}", msg))
                    }
                })()
                .await;

                if let Err(err) = res {
                    eprintln!("[{}] {}", _s.id(), err);
                    _s.leave_all();
                    break;
                }
            }
        });
    }

    pub fn to(&self, room_id: impl Into<String>) -> impl DerefMut<Target = Room> + Send + '_ {
        self.rooms.get(room_id.into())
    }

    pub fn join(&self, room_id: impl Into<String>) {
        self.rooms.get(room_id.into()).add_member(self.clone());
    }

    pub fn leave(&self, room_id: impl Into<String>) {
        self.rooms.get(room_id.into()).remove_member(self);
    }

    pub fn leave_all(&self) {
        self.rooms.remove_from_all(self);
    }
}

#[async_trait]
impl Socket for WsSocket {
    type EventType = &'static str;
    type EventContext = WsSocket;
    type EventArgs = serde_json::Value;
    type EventHandler = EventHandler<(Self::EventContext, Self::EventArgs)>;

    async fn emit(&self, event_type: Self::EventType, event_args: Self::EventArgs) {
        self.send(Message::text(
            serde_json::to_string(&json!({
                "type": event_type,
                "args": event_args
            }))
            .unwrap(),
        ))
        .await
        .unwrap_or_else(|_| self.leave_all());
    }

    fn on(&self, event_type: Self::EventType, callback: Self::EventHandler) {
        let mut h = tokio::task::block_in_place(move || self.handlers.blocking_write());
        let hs = h.entry(event_type.to_owned()).or_insert_with(|| Vec::new());
        (*hs).push(callback);
    }
}

#[derive(Clone)]
pub struct Rooms(Arc<parking_lot::Mutex<HashMap<String, Room>>>);

impl Default for Rooms {
    fn default() -> Self {
        Self::new()
    }
}

impl Rooms {
    pub fn new() -> Self {
        Self(Arc::new(parking_lot::Mutex::new(HashMap::new())))
    }

    pub fn get(&self, room_id: String) -> impl DerefMut<Target = Room> + Send + '_ {
        parking_lot::MutexGuard::map(self.0.lock(), |l| l.entry(room_id).or_insert(Room::new()))
    }

    pub fn remove_from_all(&self, socket: &WsSocket) {
        for (_key, room) in self.0.lock().iter_mut() {
            room.remove_member(socket);
        }
    }
}

pub struct Room {
    members: Arc<parking_lot::RwLock<HashSet<WsSocket>>>,
}

impl Room {
    pub fn new() -> Self {
        Self {
            members: Arc::new(parking_lot::RwLock::new(HashSet::new())),
        }
    }

    fn add_member(&mut self, member: WsSocket) {
        self.members.write().insert(member);
    }

    fn remove_member(&mut self, member: &WsSocket) {
        self.members.write().remove(member);
    }

    pub async fn emit(
        &self,
        event_type: <WsSocket as Socket>::EventType,
        event_args: <WsSocket as Socket>::EventArgs,
    ) {
        futures::future::join_all(self.members.read().iter().map(|member| {
            let event_type = event_type.clone();
            let event_args = event_args.clone();
            async move { member.emit(event_type, event_args).await }
        }))
        .await;
    }

    pub async fn emit_except(
        &self,
        event_type: <WsSocket as Socket>::EventType,
        event_args: <WsSocket as Socket>::EventArgs,
        except: &WsSocket,
    ) {
        futures::future::join_all(self.members.read().iter().filter_map(|member| {
            if member == except {
                None
            } else {
                let event_type = event_type.clone();
                let event_args = event_args.clone();
                Some(async move { member.emit(event_type, event_args).await })
            }
        }))
        .await;
    }
}
