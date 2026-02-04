use rusqlite::Result;
use serde_json::json;
use warp::http::Response;
use warp::{Rejection, Reply};
//
use crate::errors::ServiceError;
use crate::models::{NewUser, User, UserResp};
use crate::utils::crypto::{authn::AuthnToken, pwhash};
use crate::utils::db_conn;

#[derive(Debug, Deserialize)]
pub struct RegisterUserIn {
    pub email: String,
    pub password: String,
}

pub async fn user_register(ruIn: RegisterUserIn) -> Result<impl Reply, Rejection> {
    let db = db_conn::get().map_err(ServiceError::from)?;

    let newUser = NewUser::from_credentials(&ruIn.email, &ruIn.password)?;
    let user = newUser.insert(&db)?;

    // set signed cookie with userID
    let token = AuthnToken::from_userId(user.id)?;
    Ok(Response::builder()
        .header("Set-Cookie", token.header_val())
        .body(json!(UserResp::from(user)).to_string()))
}

#[derive(Debug, Deserialize)]
pub struct UserAuthIn {
    pub email: String,
    pub password: String,
}

pub async fn user_login(authIn: UserAuthIn) -> Result<impl Reply, Rejection> {
    let db = db_conn::get().map_err(ServiceError::from)?;

    let mut stmt = db
        .prepare("SELECT id, created_at, email, hash_pass FROM user where email = :email")
        .or(Err(ServiceError::Unauthorized))?;
    let mut user_iter = stmt
        .query_map(&[(":email", &authIn.email)], |row| {
            Ok(User {
                id: row.get(0)?,
                created_at: row.get(1)?,
                email: row.get(2)?,
                hash_pass: row.get(3)?,
            })
        })
        .or(Err(ServiceError::Unauthorized))?;

    if let Some(result) = user_iter.next() {
        if let Ok(user) = result {
            pwhash::verify(&user.hash_pass, &authIn.password)
                .or(Err(ServiceError::Unauthorized))?;

            // set signed cookie with userID
            let token = AuthnToken::from_userId(user.id)?;
            return Ok(Response::builder()
                .header("Set-Cookie", token.header_val())
                .body(json!(UserResp::from(user)).to_string()));
        }
    }
    Err(ServiceError::Unauthorized)?
}

#[derive(Debug, Deserialize)]
pub struct UserCheckIn {
    pub email: String,
}

pub async fn user_check(checkIn: UserCheckIn) -> Result<impl Reply, Rejection> {
    let db = db_conn::get().map_err(ServiceError::from)?;

    let mut stmt = db
        .prepare("SELECT id, created_at, email, hash_pass FROM user  where email = :email")
        .or(Err(ServiceError::Unauthorized))?;
    let mut user_iter = stmt
        .query_map(&[(":email", &checkIn.email)], |row| {
            Ok(User {
                id: row.get(0)?,
                created_at: row.get(1)?,
                email: row.get(2)?,
                hash_pass: row.get(3)?,
            })
        })
        .or(Err(ServiceError::Unauthorized))?;

    if let Some(result) = user_iter.next() {
        if let Ok(user) = result {
            return Ok(Response::builder().body(json!(UserResp::from(user)).to_string()));
        }
    }
    Err(ServiceError::Unauthorized)?
}
