use actix_web::{
    http::{header::HeaderValue, StatusCode},
    web,
};

use crate::{
    config::db::Pool,
    constants,
    error::ServiceError,
    models::token::{Token, TokenJson},
    models::user::{User, UserSignin, UserSignup},
};

/// Verify a given JWT is valid and return the corrosponding user from the database
pub fn verify(auth_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<User, ()> {
    if let Ok(auth) = auth_header.to_str() {
        if auth.starts_with("Bearer") {
            let token = auth[6..auth.len()].trim();
            if let Ok(data) = Token::decode(token.to_string()) {
                if let Ok(username) = Token::verify(&data, pool) {
                    if let Ok(user) = User::find_by_username(&username, &pool.get().unwrap()) {
                        return Ok(user);
                    }
                }
            }
        }
    }
    Err(())
}

/// Sign up a new user and encode the user response in a JWT.
pub fn signup(user: UserSignup, pool: &web::Data<Pool>) -> Result<TokenJson, ServiceError> {
    match User::signup(user, &pool.get().unwrap()) {
        Ok(user) => Ok(Token::generate_json(user)),
        Err(message) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            message,
        )),
    }
}

/// Sign in to an existing user and encode the user response in a JWT.
pub fn signin(user: UserSignin, pool: &web::Data<Pool>) -> Result<TokenJson, ServiceError> {
    match User::signin(user, &pool.get().unwrap()) {
        Some(user) => Ok(Token::generate_json(user)),
        None => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::SIGN_IN_FAILURE.to_string(),
        )),
    }
}

/// Sign out of the current user by verifying the given JWT.
pub fn signout(auth_header: &HeaderValue, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match verify(auth_header, &pool) {
        Ok(user) => {
            User::signout(user.id, &pool.get().unwrap());
            Ok(())
        }
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::SIGN_OUT_TOKEN_ERROR.to_string(),
        )),
    }
}
