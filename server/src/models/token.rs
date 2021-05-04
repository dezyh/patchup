use actix_web::web;
use jsonwebtoken::{EncodingKey,Header,DecodingKey, TokenData, Validation};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{
    config::db::Pool,
    models::user::{User, UserResponse},
};

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

#[derive(Serialize, Deserialize)]
pub struct TokenJson {
    pub token: String,
    pub token_type: String,
}

impl Token {
    
    pub fn generate(signin: UserResponse) -> String {
        let now = Utc::now().timestamp_nanos() / NANOSECONDS;
        let payload = Token {
            iat: now,
            exp: now + ONE_WEEK,
            user: signin.username,
            session: signin.session,
        };
        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(&KEY)).unwrap()
    }

    pub fn generate_json(signin: UserResponse) -> TokenJson {
        let token = Token::generate(signin);
        TokenJson {
            token,
            token_type: "bearer".to_string()
        }
    }

    pub fn decode(token: String) -> jsonwebtoken::errors::Result<TokenData<Token>> {
        jsonwebtoken::decode::<Token>(&token, &DecodingKey::from_secret(&KEY), &Validation::default())
    }

    pub fn decode_auth_header(auth: &str) -> Result<jsonwebtoken::errors::Result<TokenData<Token>>, String> {
        match auth.starts_with("bearer") {
            true => {
                let token = auth[6..auth.len()].trim();
                Ok(Token::decode(token.to_string()))
            },
            false => Err("Invalid auth header".to_string())
        }
    }

    //if let Ok(auth) = auth_header.to_str() {
    //    if auth.starts_with("bearer") {
    //        let token = auth[6..auth.len()].trim();
    //        if let Ok(token_data) = Token::decode(token.to_string()) {

    pub fn verify(token_data: &TokenData<Token>, pool: &web::Data<Pool>) -> Result<String, String> {
        match User::validate_session(&token_data.claims, &pool.get().unwrap()) {
            true => Ok(token_data.claims.user.to_string()),
            false => Err("Invalid token".to_string())
        }
    }
}
