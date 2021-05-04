use crate::{
    config::db::Pool,
    constants,
    models::{
        response::ResponseBody,
        user::{UserSignin, UserSignup},
    },
    services,
};
use actix_web::{web, HttpRequest, HttpResponse, Result};

pub async fn signup(
    user_signup: web::Json<UserSignup>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let user = user_signup.0;
    match services::user::signup(user, &pool) {
        Ok(token) => Ok(
            HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_SIGNUP_SUCCESS, token))
        ),
        Err(err) => Ok(err.response()),
    }
}

pub async fn signin(
    user_signin: web::Json<UserSignin>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse> {
    let user = user_signin.0;
    match services::user::signin(user, &pool) {
        Ok(token) => {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_LOGIN_SUCCESS, token)))
        }
        Err(err) => Ok(err.response()),
    }
}

pub async fn signout(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let auth = req.headers().get(constants::AUTHORIZATION);
    match auth {
        Some(auth) => match services::user::signout(auth, &pool) {
            Ok(_) => Ok(HttpResponse::Ok().json(ResponseBody::new(
                constants::MESSAGE_LOGOUT_SUCCESS,
                constants::EMPTY,
            ))),
            Err(err) => Ok(err.response()),
        },
        None => Ok(HttpResponse::BadRequest().json(ResponseBody::new(
            constants::MESSAGE_TOKEN_MISSING,
            constants::EMPTY,
        ))),
    }
}
