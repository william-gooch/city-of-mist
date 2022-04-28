use crate::routes::error;
use crate::service::database::*;
use async_session::{MemoryStore, Session, SessionStore};
use common::entity::user as db_user;
use common::entity::user::Entity as DbUser;
use common::sea_orm::prelude::*;
use common::sea_orm::ActiveValue::*;
use common::user::User;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::{reject::InvalidQuery, reject::Reject, Filter, Rejection, Reply};

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

pub struct AuthService {
    store: MemoryStore,
}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService {
            store: MemoryStore::new(),
        }
    }
}

pub type Auth = Arc<AuthService>;

async fn get_auth(token: String, auth: Auth) -> Result<i32, Rejection> {
    println!("{}", token.clone());
    let session = auth
        .store
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

async fn get_login_details(map: HashMap<String, String>) -> Result<(String, String), Rejection> {
    let email = map
        .get("email")
        .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("email".into())))?;
    let password = map
        .get("password")
        .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("password".into())))?;
    Ok((email.clone(), password.clone()))
}

async fn get_signup_details(
    map: HashMap<String, String>,
) -> Result<(String, String, String), Rejection> {
    let email = map
        .get("email")
        .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("email".into())))?;
    let username = map
        .get("username")
        .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("username".into())))?;
    let password = map
        .get("password")
        .ok_or_else(|| warp::reject::custom(error::BadRequest::MissingField("password".into())))?;
    Ok((email.clone(), username.clone(), password.clone()))
}

pub fn with_auth(auth: Auth) -> impl Filter<Extract = (Auth,), Error = Infallible> + Clone {
    warp::any().map(move || auth.clone())
}

fn with_user_id(auth: Auth) -> impl Filter<Extract = (i32,), Error = Rejection> + Clone {
    warp::cookie::cookie("session-token")
        .or_else(|_err| async move { Err(warp::reject::custom(error::Unauthorized::NotLoggedIn)) })
        .and(with_auth(auth))
        .and_then(get_auth)
}

pub fn with_auth_user(
    db: Db,
    auth: Auth,
) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    with_user_id(auth)
        .and(with_db(db))
        .and_then(|uid: i32, db: Db| async move {
            if let Some(user) = DbUser::find_by_id(uid).one(db.as_ref()).await.unwrap() {
                Ok(user.into())
            } else {
                Err(warp::reject::custom(error::Unauthorized::InvalidUser))
            }
        })
}

async fn log_in(
    email: String,
    password: String,
    db: Db,
    auth: Auth,
) -> Result<impl Reply, Rejection> {
    if let Some(user) = DbUser::find()
        .filter(db_user::Column::Email.eq(email))
        .one(db.as_ref())
        .await
        .unwrap()
    {
        let parsed_hash = PasswordHash::new(&user.password_hash).unwrap();
        if let Ok(_) = Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
            let mut session = Session::new();
            session
                .insert("user_id", user.id)
                .map_err(|err| warp::reject::reject())?;
            let token = auth
                .store
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
    email: String,
    username: String,
    password: String,
    db: Db,
    auth: Auth,
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
    if let Ok(new_user) = new_user.insert(db.as_ref()).await {
        let mut session = Session::new();
        session
            .insert("user_id", new_user.id)
            .map_err(|err| warp::reject::reject())?;
        let token = auth
            .store
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

pub fn auth_routes(
    db: Db,
    auth: Auth,
) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let log_in = warp::path!("auth" / "login")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(get_login_details)
        .untuple_one()
        .and(with_db(db.clone()))
        .and(with_auth(auth.clone()))
        .and_then(log_in);

    let sign_up = warp::path!("auth" / "signup")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(get_signup_details)
        .untuple_one()
        .and(with_db(db))
        .and(with_auth(auth))
        .and_then(sign_up);

    log_in.or(sign_up)
}
