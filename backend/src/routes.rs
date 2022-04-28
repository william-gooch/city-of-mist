pub mod auth;
mod error;
mod user;
mod websocket;

use crate::routes::auth::Auth;
use crate::service::database::Db;
use crate::service::watcher::UserWatcher;
use std::convert::Infallible;

use warp::{Filter, Reply};

pub fn routes(
    db: Db,
    auth: Auth,
    user_watcher: UserWatcher,
) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    user::user_routes(db.clone(), auth.clone())
        .or(auth::auth_routes(db.clone(), auth.clone()))
        .or(websocket::websocket_route(
            db.clone(),
            auth.clone(),
            user_watcher.clone(),
        ))
        .recover(error::handle_rejection)
}
