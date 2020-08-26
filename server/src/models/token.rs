use crate::{
    config::db::Pool,
    models::user::{User, UserSigninResponse},
};
use actix_web::web;
use jsonwebtoken::{EncodingKey,Header,DecodingKey, TokenData, Validation};
use chrono::Utc;
use serde::{Deserialize, Serialize};


pub static KEY: [u8; 16] = *include_bytes!("../secret.key");
static ONE_WEEK: i64 = 60*60*24*7;
static NANOSECONDS: i64 = 1_000_000_000;

#[derive(Serialize, Deserialize)]
pub struct Token {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64, 
    // data
    pub user: String,
    pub session: String,
}

impl Token {
    
    pub fn generate(signin: UserSigninResponse) -> String {
        let now = Utc::now().timestamp_nanos() / NANOSECONDS;
        let payload = Token {
            iat: now,
            exp: now + ONE_WEEK,
            user: signin.username,
            session: signin.session,
        };
        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    pub fn decode(token: String) -> jsonwebtoken::errors::Result<TokenData<Token>> {
        jsonwebtoken::decode::<Token>(&token, &DecodingKey::from_secret(&KEY), &Validation::default())
    }

    pub fn verify(token_data: &TokenData<Token>, pool: &web::Data<Pool>) -> Result<String, String> {
        if User::validate_session(&token_data.claims, &pool.get().unwrap()) {
            Ok(token_data.claims.user.to_string())
        } else {
            Err("Invalid token".to_string())
        }
    }
}