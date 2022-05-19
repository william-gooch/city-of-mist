use common::user::User;
use shaku::{Component, Interface};
use std::convert::Infallible;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use super::auth::AuthRoutes;
use crate::service::{Auth, Db};

use super::{box_reply, AppFilter};

pub trait UserRoutes: Interface {
    fn get_filter(self: Arc<Self>) -> AppFilter;
}

#[derive(Component)]
#[shaku(interface = UserRoutes)]
pub struct UserRoutesImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
    #[shaku(inject)]
    auth: Arc<dyn AuthRoutes>,
}

impl UserRoutesImpl {
    async fn get_me(&self, user: User) -> Result<impl Reply, Infallible> {
        Ok(warp::reply::with_status(
            format!("Logged in as {}", user.email),
            StatusCode::OK,
        ))
    }

    async fn get_user(&self, uid: String) -> Result<impl Reply, Infallible> {
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
}

impl UserRoutes for UserRoutesImpl {
    fn get_filter(self: Arc<Self>) -> AppFilter {
        let get_me = warp::path!("user" / "me")
            .and(warp::get())
            .and(self.auth.clone().with_auth_user())
            .and_then({
                let _self = self.clone();
                move |user| {
                    let _self = _self.clone();
                    async move { _self.get_me(user).await }
                }
            })
            .map(box_reply);

        let get_user = warp::path!("user" / "by_id" / String)
            .and(warp::get())
            .and_then({
                let _self = self.clone();
                move |uid| {
                    let _self = _self.clone();
                    async move { _self.get_user(uid).await }
                }
            })
            .map(box_reply);

        get_me.or(get_user).map(box_reply).boxed()
    }
}
