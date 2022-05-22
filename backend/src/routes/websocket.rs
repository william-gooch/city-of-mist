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
    character_mgr: Arc<dyn CharacterManager>,
    #[shaku(inject)]
    rooms: Arc<dyn crate::service::Rooms>,
    #[shaku(inject)]
    ws_handler: Arc<dyn WsHandler>,
}

impl WebsocketRoutes for WebsocketRoutesImpl {
    fn get_filter(self: Arc<Self>) -> AppFilter {
        warp::path!("ws")
            //.and(with_auth_user(db.clone(), auth))
            .and(warp::ws())
            .map({
                let _self = self.clone();
                move |ws: Ws| {
                    let _self = self.clone();
                    ws.on_upgrade(move |socket| _self.ws_handler.clone().make_new().start(socket))
                }
            })
            .map(box_reply)
            .boxed()
    }
}
