pub mod auth;
mod error;
mod user;
mod websocket;

use crate::service::*;
use futures::Future;
use shaku::{module, Component, Interface};
use std::{convert::Infallible, sync::Arc};

use warp::{filters::BoxedFilter, Filter, Rejection, Reply};

// type AppFilter = Box<dyn Filter<Extract = (Box<dyn Reply>,), Error = Infallible, Future = dyn Future<Output = Result<(Box<dyn Reply>,), Infallible>>>>;
type AppFilter = BoxedFilter<(Box<dyn Reply>,)>;

pub fn box_reply(reply: impl Reply + 'static) -> Box<dyn Reply> {
    Box::new(reply)
}

pub trait RootRoutes: Interface {
    fn get_filter(self: Arc<Self>) -> AppFilter;
}

#[derive(Component)]
#[shaku(interface = RootRoutes)]
struct RootRoutesImpl {
    #[shaku(inject)]
    auth: Arc<dyn auth::AuthRoutes>,
    #[shaku(inject)]
    user: Arc<dyn user::UserRoutes>,
    #[shaku(inject)]
    websocket: Arc<dyn websocket::WebsocketRoutes>,
}

impl RootRoutes for RootRoutesImpl {
    fn get_filter(self: Arc<Self>) -> AppFilter {
        self.user
            .clone()
            .get_filter()
            .or(self.auth.clone().get_filter())
            .or(self.websocket.clone().get_filter())
            .recover(error::handle_rejection)
            .map(|r| -> Box<dyn Reply> { Box::new(r) })
            .boxed()
    }
}

module! {
    pub RoutesModule {
        components = [RootRoutesImpl, auth::AuthRoutesImpl, user::UserRoutesImpl, websocket::WebsocketRoutesImpl],
        providers = [],

        use ServiceModule {
            components = [dyn Auth, dyn Db, dyn CharacterManager, dyn Rooms],
            providers = []
        }
    }
}
