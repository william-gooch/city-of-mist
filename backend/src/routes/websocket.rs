use crate::routes::auth::*;
use crate::service::database::*;
use crate::service::watcher::*;
use common::user::User;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_util::sync::PollSender;
use warp::ws::{Ws};
use warp::{Filter, Rejection, Reply};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct HandlerState {
    user: Option<User>,
}

pub struct HandlerStateWatcher {
    user_watcher: Watcher<User>,
}

impl HandlerStateWatcher {
    pub fn watch(&self) -> ReceiverStream<HandlerState> {
        let (tx, rx) = mpsc::channel(16);
        let tx = PollSender::new(tx);
        let rx = ReceiverStream::new(rx);
        tokio::task::spawn(
            self.user_watcher
                .watch()
                .map(|user| {
                    Ok(HandlerState {
                        user: Some(user.unwrap()),
                    })
                })
                .forward(tx),
        );

        rx
    }
}

pub struct WebSocketHandler {
    sender: PollSender<Result<warp::ws::Message, warp::Error>>,
    db: Db,
    watcher: HandlerStateWatcher,
}

impl WebSocketHandler {
    pub fn new(
        sender: PollSender<Result<warp::ws::Message, warp::Error>>,
        db: Db,
        watcher: HandlerStateWatcher,
    ) -> WebSocketHandler {
        WebSocketHandler {
            sender,
            db,
            watcher,
        }
    }

    pub fn run(mut self) {
        tokio::spawn(async move { self.main_loop().await });
    }

    async fn main_loop(mut self) {
        let mut rx = self.watcher.watch();
        while let state = rx.next().await.unwrap() {
            self.sender
                .send(Ok(warp::ws::Message::text(
                    serde_json::to_string(&state).unwrap(),
                )))
                .await
                .unwrap();
        }
    }
}

pub fn websocket_route(
    db: Db,
    auth: Auth,
    user_watcher: UserWatcher,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("ws")
        .and(with_auth_user(db.clone(), auth))
        .and(warp::ws())
        .and(with_db(db))
        .and(with_watcher(user_watcher))
        .map(|user: User, ws: Ws, db: Db, user_watcher: UserWatcher| {
            ws.on_upgrade(|mut socket| async move {
                println!("Connection from new host!");
                let (client_ws_sender, mut client_ws_rcv) = socket.split();
                let (client_sender, client_rcv) =
                    mpsc::channel::<Result<warp::ws::Message, warp::Error>>(16);
                let client_sender = PollSender::new(client_sender);
                let client_rcv = ReceiverStream::new(client_rcv);

                tokio::task::spawn(client_rcv.forward(client_ws_sender));
                WebSocketHandler::new(
                    client_sender,
                    db,
                    HandlerStateWatcher {
                        user_watcher: user_watcher.get_by_id(user.id).await,
                    },
                )
                .run();

                while let Some(item) = client_ws_rcv.next().await {
                    match item {
                        Ok(msg) => {
                            println!("{:?}", msg);
                            user_watcher.get_by_id(user.id).await.trigger(user.clone());
                        }
                        Err(err) => println!("{}", err),
                    }
                }
            })
        })
}
