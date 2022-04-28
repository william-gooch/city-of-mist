use common::user::User;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use crate::routes::auth::*;
use crate::service::database::*;

async fn get_me(user: User) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::with_status(
        format!("Logged in as {}", user.email),
        StatusCode::OK,
    ))
}

async fn get_user(uid: String, db: Db) -> Result<impl Reply, Infallible> {
    // if let Some(user) = db.find_one::<User>(doc! { "_id": ObjectId::parse_str(&uid[..]).unwrap() }, None).await.unwrap() {
    //     Ok(warp::reply::with_status(warp::reply::json(&user), StatusCode::OK).into_response())
    // } else {
    //     Ok(warp::reply::with_status("User not found.".to_owned(), StatusCode::NOT_FOUND).into_response())
    // }
    Ok(
        warp::reply::with_status("User not found.".to_owned(), StatusCode::NOT_FOUND)
            .into_response(),
    )
}

pub fn user_routes(
    db: Db,
    auth: Auth,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let get_me = warp::path!("user" / "me")
        .and(warp::get())
        .and(with_auth_user(db.clone(), auth))
        .and_then(get_me);

    let get_user = warp::path!("user" / "by_id" / String)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and_then(get_user);

    get_me.or(get_user)
}
