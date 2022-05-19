use serde::Serialize;
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reject::{Reject, Rejection};
use warp::Reply;

#[derive(Debug)]
pub enum Unauthorized {
    NotLoggedIn,
    InvalidToken,
    InvalidUser,
    InvalidCredentials,
    SessionTimedOut,
}
impl Reject for Unauthorized {}

#[derive(Debug)]
pub enum BadRequest {
    MissingField(String),
}
impl Reject for BadRequest {}

#[derive(Serialize)]
pub struct ErrorType {
    pub msg: String,
    pub detail: Option<String>,
    pub code: u16,
}

pub async fn handle_rejection(err: Rejection) -> Result<Box<dyn Reply>, Infallible> {
    if err.is_not_found() {
        Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorType {
                msg: "NOT_FOUND".into(),
                detail: None,
                code: StatusCode::NOT_FOUND.into(),
            }),
            StatusCode::NOT_FOUND,
        )))
    } else if let Some(err) = err.find::<Unauthorized>() {
        Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorType {
                msg: "UNAUTHORIZED".into(),
                detail: Some(
                    match err {
                        Unauthorized::NotLoggedIn => "User not logged in.",
                        Unauthorized::InvalidToken => "Invalid token.",
                        Unauthorized::InvalidUser => "Token refers to invalid user.",
                        Unauthorized::InvalidCredentials => "Invalid credentials.",
                        Unauthorized::SessionTimedOut => "Session timed out.",
                    }
                    .into(),
                ),
                code: StatusCode::UNAUTHORIZED.into(),
            }),
            StatusCode::UNAUTHORIZED,
        )))
    } else if let Some(err) = err.find::<BadRequest>() {
        Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorType {
                msg: "BAD_REQUEST".into(),
                detail: Some(
                    match err {
                        BadRequest::MissingField(field_name) => {
                            format!("Field missing: {}", field_name)
                        }
                    }
                    .into(),
                ),
                code: StatusCode::BAD_REQUEST.into(),
            }),
            StatusCode::BAD_REQUEST,
        )))
    } else {
        println!("Unhandled error: {:?}", err);
        Ok(Box::new(warp::reply::with_status(
            warp::reply::json(&ErrorType {
                msg: "INTERNAL_SERVER_ERROR".into(),
                detail: None,
                code: StatusCode::INTERNAL_SERVER_ERROR.into(),
            }),
            StatusCode::INTERNAL_SERVER_ERROR,
        )))
    }
}
