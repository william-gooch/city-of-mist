use serde_json::json;
use shaku::{Component, Interface};
use socket::ws::*;
use std::sync::Arc;
use warp::ws::WebSocket;

#[async_trait::async_trait]
pub trait WsHandler: Interface {
    async fn start(self: Arc<Self>, ws: WebSocket);
    fn make_new(self: Arc<Self>) -> Arc<dyn WsHandler>;
}

#[derive(Component, Clone)]
#[shaku(interface = WsHandler)]
pub struct WsHandlerImpl {
    #[shaku(inject)]
    character_mgr: Arc<dyn crate::service::CharacterManager>,
    #[shaku(inject)]
    rooms: Arc<dyn crate::service::Rooms>,
}

#[async_trait::async_trait]
impl WsHandler for WsHandlerImpl {
    async fn start(self: Arc<Self>, ws: WebSocket) {
        let ws = WsSocket::new(ws, self.rooms.get().clone());
        ws.join("dice");
        ws.on(
            "ping",
            EventHandler::wrap({
                move |(socket, _): (WsSocket, serde_json::Value)| async move {
                    println!("got ping");
                    socket.emit("pong", json!({})).await;

                    Ok(())
                }
            }),
        );
        ws.on(
            "msg",
            EventHandler::wrap({
                move |(socket, args): (WsSocket, serde_json::Value)| async move {
                    let to = args
                        .get("to")
                        .ok_or("No `to` argument.")?
                        .as_str()
                        .ok_or("Argument `to` is not of type String")?;
                    let content = args
                        .get("content")
                        .ok_or("No `content` argument.")?
                        .as_str()
                        .ok_or("Argument `content` is not of type String");
                    socket
                        .to(to)
                        .emit_except("msg", json!({ "content": content }), &socket)
                        .await;

                    Ok(())
                }
            }),
        );
        ws.on(
            "dice",
            EventHandler::wrap({
                move |(socket, _args): (WsSocket, serde_json::Value)| async move {
                    let seed = rand::random::<u64>();
                    socket
                        .to("dice")
                        .emit("dice", json!({ "seed": seed }))
                        .await;

                    Ok(())
                }
            }),
        );
        ws.on(
            "character",
            EventHandler::wrap({
                let _self = self.clone();
                move |(socket, args): (WsSocket, serde_json::Value)| {
                    let _self = _self.clone();
                    async move {
                        let character_id = args
                            .get("cid")
                            .unwrap()
                            .as_i64()
                            .ok_or("No `cid` parameter specified.")?
                            as i32;
                        let character = _self
                            .character_mgr
                            .load(character_id)
                            .await
                            .ok_or("No such character found.")?;
                        let room = format!("character/{}", character_id);
                        socket.join(room.clone());
                        socket
                            .to(room)
                            .emit("character/update", json!({ "character": *character }))
                            .await;

                        Ok(())
                    }
                }
            }),
        );
        ws.on(
            "character/new",
            EventHandler::wrap({
                let _self = self.clone();
                move |(socket, _args): (WsSocket, serde_json::Value)| {
                    let _self = _self.clone();
                    async move {
                        let new_character = _self
                            .character_mgr
                            .create(data::example_character())
                            .await
                            .map_err(|_| "Couldn't create new character.")?;
                        let room = format!(
                            "character/{}",
                            new_character.id.ok_or("New character doesn't have ID.")?
                        );
                        socket.join(room.clone());
                        socket
                            .to(room)
                            .emit("character/update", json!({ "character": *new_character }))
                            .await;

                        Ok(())
                    }
                }
            }),
        );
        ws.on(
            "character/modify",
            EventHandler::wrap({
                let _self = self.clone();
                move |(socket, args): (WsSocket, serde_json::Value)| {
                    let _self = _self.clone();
                    async move {
                        let cid = args
                            .get("cid")
                            .ok_or("No `cid` parameter specified.")?
                            .as_i64()
                            .ok_or("Argument `cid` is not an integer")?
                            as i32;
                        let updated = args
                            .get("character")
                            .ok_or("No `character` parameter specified.")?;
                        let new_character = _self
                            .character_mgr
                            .mutate_from_json(cid, updated.clone())
                            .await?;

                        let room = format!("character/{}", new_character.id.unwrap());
                        socket.join(room.clone());

                        socket
                            .to(room)
                            .emit("character/update", json!({ "character": *new_character }))
                            .await;

                        Ok(())
                    }
                }
            }),
        );
        ws.emit("connect", json!({ "id": ws.id() })).await;
        ws.to("client_1").emit("asdf", json!({})).await;
    }

    fn make_new(self: Arc<Self>) -> Arc<dyn WsHandler> {
        Arc::new((*self).clone())
    }
}
