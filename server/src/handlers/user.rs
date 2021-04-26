use crate::{
    config::db::Pool,
    constants,
    models::{
        user::{UserSignup, UserSignin},
        response::ResponseBody,
    },
    services::user,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};


// POST api/auth/signup
pub async fn post_signup(user_signup: web::Json<UserSignup>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    info!("u: {}, e: {}", user_signup.username, user_signup.email);
    match user::signup(user_signup.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_SIGNUP_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/signin
pub async fn signin(user_signin: web::Json<UserSignin>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    match user::signin(user_signin.0, &pool) {
        Ok(token_res) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token_res))),
        Err(err) => Ok(err.response()),
    }
}

// POST api/auth/signout
pub async fn signout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Some(auth_header) = req.headers().get(constants::AUTHORIZATION) {
        match user::signout(auth_header, &pool) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGOUT_SUCCESS, constants::EMPTY))),
            Err(err) => Ok(err.response()),
        }
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::MESSAGE_TOKEN_MISSING, constants::EMPTY)))
    }
}
