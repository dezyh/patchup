use super::schema::*;
use serde::{Deserialize, Serialize};

use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub username: String,
    pub email: String,
    pub passhash: String,
    pub firstname: String,
    pub lastname: String,
    pub verified: bool,
    pub created_at: NaiveDateTime,
}
