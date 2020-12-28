use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::user::{User,UserSignup,UserSignin},
    models::token::Token,
};
use actix_web::{web, http::{StatusCode, header::HeaderValue}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TokenBodyResponse {
    pub token: String,
    pub token_type: String,
}

pub fn signup(user: UserSignup, pool: &web::Data<Pool>) -> Result<String, ServiceError> {
    match User::signup(user, &pool.get().unwrap()) {
        Ok(message) => Ok(message),
        Err(message) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, message))
    }
}

pub fn signin(user: UserSignin, pool: &web::Data<Pool>) -> Result<TokenBodyResponse, ServiceError> {
    match User::signin(user, &pool.get().unwrap()) {
        Some(valid_user) => {
            match serde_json::from_value(json!({ "token": Token::generate(valid_user), "token_type": "bearer" })) {
                Ok(token_res) => Ok(token_res),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::SIGN_IN_FAILURE.to_string()))
            }
        },
        None => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::SIGN_IN_FAILURE.to_string()))
    }
}

pub fn signout(auth_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    if let Ok(auth) = auth_header.to_str() {
        if auth.starts_with("bearer") {
            let token = auth[6..auth.len()].trim();
            if let Ok(token_data) = Token::decode(token.to_string()) {
                if let Ok(username) = Token::verify(&token_data, pool) {
                    if let Ok(user) = User::find_by_username(&username, &pool.get().unwrap()) {
                        User::signout(user.id, &pool.get().unwrap());
                        return Ok(());
                    }
                }
            }
        }
    }
    Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, constants::SIGN_OUT_TOKEN_ERROR.to_string()))
}