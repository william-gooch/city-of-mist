use futures::{Sink, Stream, StreamExt};
use std::sync::Arc;
use warp::ws::{WebSocket, Message};
use warp::{Error};

trait Socket {
    type Room: Eq;
    type EventType;
    type EventArgs;

    // fn join(room: Self::Room);
    // fn leave(room: Self::Room);
    fn emit(r#type: Self::EventType, args: Self::EventArgs);
    fn on(r#type: Self::EventType, callback: Box<dyn Fn(Self::EventArgs)>);
}

struct WsSocket {
    id: String,
    ws_send: Arc<dyn Sink<Message, Error = Error>>,
    ws_recv: Arc<dyn Stream<Item = Result<Message, Error>>>,
}

impl WsSocket {
    pub fn new(ws: WebSocket) -> Self {
        let (send, recv) = ws.split();

        Self {
            id: "".to_owned(),
            ws_send: Arc::new(send),
            ws_recv: Arc::new(recv),
        }
    }
}

impl Socket for WsSocket {
    type Room = String;
    type EventType = String;
    type EventArgs = ();

    fn emit(r#type: Self::EventType, args: Self::EventArgs) {
    }

    fn on(r#type: Self::EventType, callback: Box<dyn Fn(Self::EventArgs)>) {
    }
}

