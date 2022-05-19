use crate::routes::error;
use crate::service::Auth;
use crate::service::Db;
use async_session::Session;
use common::entity::user as db_user;
use common::entity::user::Entity as DbUser;
use common::sea_orm::prelude::*;
use common::sea_orm::ActiveValue::*;
use common::user::User;
use futures::future::BoxFuture;
use futures::Future;
use futures::FutureExt;
use shaku::{Component, Interface};
use std::collections::HashMap;
use std::sync::Arc;
use warp::filters::BoxedFilter;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use super::box_reply;
use super::AppFilter;

pub trait AuthRoutes: Interface {
    fn with_auth_user(self: Arc<Self>) -> BoxedFilter<(User,)>;
    fn get_filter(self: Arc<Self>) -> AppFilter;
}

#[derive(Component)]
#[shaku(interface = AuthRoutes)]
pub struct AuthRoutesImpl {
    #[shaku(inject)]
    db: Arc<dyn Db>,
    #[shaku(inject)]
    auth: Arc<dyn Auth>,
}

impl AuthRoutesImpl {
    async fn get_auth(&self, token: String) -> Result<i32, Rejection> {
        println!("{}", token.clone());
        let session = self
            .auth
            .load_session(token.clone())
            .await
            .map_err(|err| warp::reject::custom(error::Unauthorized::InvalidToken))?
            .ok_or_else(|| warp::reject::custom(error::Unauthorized::InvalidToken))?;
        let user_id = session
            .get::<i32>("user_id")
            .ok_or_else(|| warp::reject::custom(error::Unauthorized::InvalidToken))?;
        println!("{:?}", user_id);
        Ok(user_id)
    }

    async fn get_login_details(
        &self,
        map: HashMap<String, String>,
    ) -> Result<(String, String), Rejection> {
        let email = map
            .get("email")
            .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("email".into())))?;
        let password = map.get("password").ok_or_else(|| {
            warp::reject::custom(error::BadRequest::MissingField("password".into()))
        })?;
        Ok((email.clone(), password.clone()))
    }

    async fn get_signup_details(
        &self,
        map: HashMap<String, String>,
    ) -> Result<(String, String, String), Rejection> {
        let email = map
            .get("email")
            .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("email".into())))?;
        let username = map.get("username").ok_or_else(|| {
            warp::reject::custom(error::BadRequest::MissingField("username".into()))
        })?;
        let password = map.get("password").ok_or_else(|| {
            warp::reject::custom(error::BadRequest::MissingField("password".into()))
        })?;
        Ok((email.clone(), username.clone(), password.clone()))
    }

    fn with_user_id(self: &Arc<Self>) -> impl Filter<Extract = (i32,), Error = Rejection> + Clone {
        let _self = self.clone();
        warp::cookie::cookie("session-token")
            .or_else(
                |_err| async move { Err(warp::reject::custom(error::Unauthorized::NotLoggedIn)) },
            )
            .and_then(move |cookie| {
                let _self = _self.clone();
                async move { _self.get_auth(cookie).await }
            })
    }

    async fn log_in(&self, email: String, password: String) -> Result<impl Reply, Rejection> {
        if let Some(user) = DbUser::find()
            .filter(db_user::Column::Email.eq(email))
            .one(self.db.get())
            .await
            .unwrap()
        {
            let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
            if let Ok(_) = Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
                let mut session = Session::new();
                session
                    .insert("user_id", user.id)
                    .map_err(|err| warp::reject::reject())?;
                let token = self
                    .auth
                    .store_session(session)
                    .await
                    .map_err(|err| warp::reject::reject())?
                    .ok_or_else(|| warp::reject::reject())?;
                Ok(warp::reply::with_header(
                    warp::reply::with_status("OK", StatusCode::OK),
                    "set-cookie",
                    format!("session-token={}; Path=/", token),
                ))
            } else {
                Err(warp::reject::custom(
                    error::Unauthorized::InvalidCredentials,
                ))
            }
        } else {
            Err(warp::reject())
        }
    }

    async fn sign_up(
        &self,
        email: String,
        username: String,
        password: String,
    ) -> Result<impl Reply, Rejection> {
        let password_salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &password_salt)
            .unwrap()
            .to_string();
        let new_user = db_user::ActiveModel {
            email: Set(email),
            username: Set(username),
            password_hash: Set(password_hash),
            password_salt: Set(password_salt.to_string()),
            ..Default::default()
        };
        if let Ok(new_user) = new_user.insert(self.db.get()).await {
            let mut session = Session::new();
            session
                .insert("user_id", new_user.id)
                .map_err(|err| warp::reject::reject())?;
            let token = self
                .auth
                .store_session(session)
                .await
                .map_err(|err| warp::reject::reject())?
                .ok_or_else(|| warp::reject::reject())?;
            Ok(warp::reply::with_header(
                warp::reply::with_status("OK", StatusCode::OK),
                "set-cookie",
                format!("session-token={}; Path=/", token),
            ))
        } else {
            Err(warp::reject())
        }
    }
}

impl AuthRoutes for AuthRoutesImpl {
    fn with_auth_user(self: Arc<Self>) -> BoxedFilter<(User,)> {
        self.with_user_id()
            .and_then({
                move |uid: i32| {
                    let _self = self.clone();
                    async move {
                        if let Some(user) =
                            DbUser::find_by_id(uid).one(_self.db.get()).await.unwrap()
                        {
                            Ok(user.into())
                        } else {
                            Err(warp::reject::custom(error::Unauthorized::InvalidUser))
                        }
                    }
                }
            })
            .boxed()
    }

    fn get_filter(self: Arc<Self>) -> AppFilter {
        let log_in = warp::path!("auth" / "login")
            .and(warp::post())
            .and(warp::body::json())
            .and_then({
                let _self = self.clone();
                move |body| {
                    let _self = _self.clone();
                    async move { _self.get_login_details(body).await }
                }
            })
            .untuple_one()
            .and_then({
                let _self = self.clone();
                move |email, password| {
                    let _self = _self.clone();
                    async move { _self.log_in(email, password).await }
                }
            })
            .map(box_reply);

        let sign_up = warp::path!("auth" / "signup")
            .and(warp::post())
            .and(warp::body::json())
            .and_then({
                let _self = self.clone();
                move |body| {
                    let _self = _self.clone();
                    async move { _self.get_signup_details(body).await }
                }
            })
            .untuple_one()
            .and_then({
                let _self = self.clone();
                move |email, username, password| {
                    let _self = _self.clone();
                    async move { _self.sign_up(email, username, password).await }
                }
            })
            .map(box_reply);

        log_in.or(sign_up).map(box_reply).boxed()
    }
}
