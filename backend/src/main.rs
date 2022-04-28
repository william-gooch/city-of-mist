mod socket;
mod routes;
mod service;

use crate::service::watcher::WatcherService;
use common::sea_orm::Database;
use common::user::User;
use dotenv::dotenv;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let auth = Arc::new(routes::auth::AuthService::new());
    let db = Arc::new(
        Database::connect(env::var("DATABASE_URL").unwrap())
            .await
            .unwrap(),
    );
    let user_watcher = Arc::new(WatcherService::<User>::new());

    let routes = routes::routes(db, auth, user_watcher);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
