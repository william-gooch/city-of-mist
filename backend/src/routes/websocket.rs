use crate::service::*;
use serde_json::json;
use shaku::Component;
use shaku::Interface;
use socket::*;
use std::sync::Arc;
use warp::ws::Ws;
use warp::Filter;

use super::box_reply;
use super::AppFilter;

pub trait WebsocketRoutes: Interface {
    fn get_filter(self: Arc<Self>) -> AppFilter;
}

#[derive(Component)]
#[shaku(interface = WebsocketRoutes)]
pub struct WebsocketRoutesImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
    #[shaku(inject)]
    rooms: Arc<dyn crate::service::Rooms>,
}

impl WebsocketRoutes for WebsocketRoutesImpl {
    fn get_filter(self: Arc<Self>) -> AppFilter {
        warp::path!("ws")
            //.and(with_auth_user(db.clone(), auth))
            .and(warp::ws())
            .map({
                let _self = self.clone();
                move |/*user: User, */ ws: Ws| {
                    let _self = self.clone();
                    ws.on_upgrade(move |socket| {
                        let _self = _self.clone();
                        async move {
                            let s = WsSocket::new(socket, _self.rooms.get().clone());
                            s.join("dice".to_owned());
                            s.on(
                                "ping".to_owned(),
                                EventHandler::wrap({
                                    move |(socket, _): (WsSocket, serde_json::Value)| async move {
                                        println!("got ping");
                                        socket.emit("pong".to_owned(), json!({})).await
                                    }
                                }),
                            );
                            s.on(
                            "msg".to_owned(),
                            EventHandler::wrap({
                                move |(socket, args): (WsSocket, serde_json::Value)| async move {
                                    let to = args.get("to").unwrap().as_str().unwrap().to_owned();
                                    let content =
                                        args.get("content").unwrap().as_str().unwrap().to_owned();
                                    socket
                                        .to(to)
                                        .emit_except(
                                            "msg".to_owned(),
                                            json!({ "content": content }),
                                            &socket,
                                        )
                                        .await
                                }
                            }),
                        );
                            s.on(
                            "dice".to_owned(),
                            EventHandler::wrap({
                                move |(socket, args): (WsSocket, serde_json::Value)| async move {
                                    let seed = rand::random::<u64>();
                                    socket
                                        .to("dice".to_owned())
                                        .emit("dice".to_owned(), json!({ "seed": seed }))
                                        .await
                                }
                            }),
                        );
                            s.on(
                            "character".to_owned(),
                            EventHandler::wrap({
                                move |(socket, args): (WsSocket, serde_json::Value)| async move {
                                    let character_id =
                                        args.get("cid").unwrap().as_i64().unwrap() as i32;
                                    let room = format!("character/{}", character_id);
                                    socket.join(room.clone());
                                    socket
                                        .to(room)
                                        .emit(
                                            "character/update".to_owned(),
                                            json!({ "character": data::example_character() }),
                                        )
                                        .await;
                                }
                            }),
                        );
                            s.emit("connect".to_owned(), json!({ "id": s.id() })).await;
                            s.to("client_1".to_owned())
                                .emit("asdf".to_owned(), json!({}))
                                .await;
                        }
                    })
                }
            })
            .map(box_reply)
            .boxed()
    }
}
