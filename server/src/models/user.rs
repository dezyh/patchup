use crate::{
    services::auth,
    constants,
    config::db::Connection,
    schema::users::{self, dsl::*},
    models::token::Token,
};
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
    pub verified: bool,
    pub session: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSignin {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserSigninResponse {
    pub username: String,
    pub email: String,
    pub session: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct UserSignup {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

impl User {

    pub fn signup(signup: UserSignup, conn: &Connection) -> Result<String, String> {
        if User::find_by_username(&signup.username, conn).is_err() {
            let hash = auth::hash_password(&signup.password);
            let user = UserSignup {
                password: hash,
                ..signup
            };
            diesel::insert_into(users).values(&user).execute(conn).unwrap();
            Ok(constants::SIGN_UP_SUCCESS.to_string())
        } else {
            Err(format!("User {} is already registered", &signup.username))
        }
    }

    pub fn signin(signin: UserSignin, conn: &Connection) -> Option<UserSigninResponse> {
        if let Ok(user) = users
            .filter(username.eq(&signin.username))
            .or_filter(email.eq(&signin.username))
            .get_result::<User>(conn) 
        {
            if auth::verify_password(&user.password, &signin.password) {
                let new_session = User::create_session();
                if User::store_session(&user.username, &new_session, conn) {
                    return Some(UserSigninResponse {
                        username: user.username,
                        email: user.email,
                        session: new_session,
                    });
                }
            }
        }
        None
    }

    pub fn signout(_id: i32, conn: &Connection) {
        if let Ok(user) = users.find(_id).get_result::<User>(conn) {
            User::store_session(&user.username, "", conn);
        }
    }

    pub fn find_by_username(_username: &str, conn: &Connection) -> QueryResult<User> {
        users.filter(username.eq(_username)).get_result::<User>(conn)
    }

    pub fn create_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    pub fn store_session(_username: &str, _session: &str, conn: &Connection) -> bool {
        if let Ok(user) = User::find_by_username(_username, conn) {
            diesel::update(users.find(user.id))
                .set(session.eq(_session.to_string()))
                .execute(conn)
                .is_ok()
        } else {
            false
        }
    }

    pub fn validate_session(token: &Token, conn: &Connection) -> bool {
        users
            .filter(username.eq(&token.user))
            .filter(session.eq(&token.session))
            .get_result::<User>(conn)
            .is_ok()
    }
}