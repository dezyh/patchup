use crate::{
    services::auth,
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

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="users"]
pub struct UserSignup {
    pub username: String,
    pub email: String,
    pub password: String,
    pub firstname: String,
    pub lastname: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub session: String,
}

impl User {

    /// Signs up a new user by querying the database to check if either username or password is
    /// already used. Then the plain text password is hashed and a user session is created and
    /// stored in the database.
    pub fn signup(mut signup: UserSignup, conn: &Connection) -> Result<UserResponse, String> {
        // Check that the users credentials don't already exist
        match User::find_by_any(&signup.username, conn) {
            Ok(_existing_user) => Err(format!("Username or email address is already registered")),
            Err(_) => {
                // Replace the plain text password with the hashed password
                signup.password = auth::hash_password(&signup.password);
                // Insert the new user into the database
                diesel::insert_into(users)
                    .values(&signup)
                    .execute(conn)
                    .unwrap();
                // Create and store a new user session
                let user_session = User::create_session();
                match User::store_session(&signup.username, &user_session, conn) {
                    true => Ok(UserResponse {
                        username: signup.username,
                        email: signup.email,
                        session: user_session,
                    }),
                    false => Err("Failed to store a user session".to_string()),
                }
            }
        }
    }

    /// Signs a user in by querying the database for a matching user, verifying the hashed
    /// passwords, creating a new session, and storing the new session in the database.
    pub fn signin(signin: UserSignin, conn: &Connection) -> Option<UserResponse> {
        // Query the database for the first user that has a matching username or email
        match User::find_by_any(&signin.username, conn) {
            Ok(user) => {
                // Verify the given password against the users password hash
                match auth::verify_password(&user.password, &signin.password) {
                    true => {
                        // Create a new session token and try store it in the database
                        let user_session = User::create_session();
                        match User::store_session(&user.username, &user_session, conn) {
                            true => Some(UserResponse {
                                username: user.username,
                                email: user.email,
                                session: user_session,
                            }),
                            false => None,
                        }
                    },
                    false => None
                }
            },
            Err(_) => None
        }
    }

    /// Signs a user out by removing their session in the database
    pub fn signout(_id: i32, conn: &Connection) {
        if let Ok(user) = users.find(_id).get_result::<User>(conn) {
            User::store_session(&user.username, "", conn);
        }
    }

    /// Finds a user by their username
    pub fn find_by_username(_username: &str, conn: &Connection) -> QueryResult<User> {
        users
            .filter(username.eq(_username))
            .get_result::<User>(conn)
    }

    /// Finds a user by their email address
    pub fn find_by_email(_email: &str, conn: &Connection) -> QueryResult<User> {
        users
            .filter(email.eq(_email))
            .get_result::<User>(conn)
    }

    /// Finds a user by any identifier (username or email address)
    pub fn find_by_any(_identifier: &str, conn: &Connection) -> QueryResult<User> {
        users
            .filter(username.eq(_identifier))
            .or_filter(email.eq(_identifier))
            .get_result::<User>(conn)
    }

    /// Creates a new uuid to represent a user session
    pub fn create_session() -> String {
        Uuid::new_v4().to_simple().to_string()
    }

    /// Stores a user session in the database 
    pub fn store_session(_username: &str, _session: &str, conn: &Connection) -> bool {
        match User::find_by_username(_username, conn) {
            Ok(user) => {
                diesel::update(users.find(user.id))
                    .set(session.eq(_session.to_string()))
                    .execute(conn)
                    .is_ok()
            },
            Err(_) => false,
        }
    }

    /// Validates a token by ensuring the username and user session match in the database 
    pub fn validate_session(token: &Token, conn: &Connection) -> bool {
        users
            .filter(username.eq(&token.user))
            .filter(session.eq(&token.session))
            .get_result::<User>(conn)
            .is_ok()
    }
}
