use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use warp::Filter;
use std::sync::Arc;
use warp::http::StatusCode;
use warp::reject::Reject;

use crate::config::Config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub email: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthError;
impl Reject for AuthError {}

pub fn create_jwt(user_id: i32, email: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        email: email.to_string(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
}

pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

pub fn with_auth(
    config: Arc<Config>,
) -> impl Filter<Extract = (Claims,), Error = warp::Rejection> + Clone {
    warp::header::<String>("authorization")
        .and(warp::any().map(move || config.clone()))
        .and_then(|auth_header: String, config: Arc<Config>| async move {
            if !auth_header.starts_with("Bearer ") {
                return Err(warp::reject::custom(AuthError));
            }

            let token = &auth_header[7..];
            match verify_jwt(token, &config.jwt_secret) {
                Ok(claims) => Ok(claims),
                Err(_) => Err(warp::reject::custom(AuthError)),
            }
        })
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}
