use crate:: {
    config::db::Connection,
    schema::{
        users::{dsl::*},
        orgs::{self, dsl::*}, 
        orgs_users::{self, dsl::*},
    },
    models::user::User,
};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct Org {
    pub id: i32,
    pub tag: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable)]
#[table_name="orgs_users"]
pub struct OrgUser {
    pub id: i32,
    pub org_id: i32,
    pub user_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="orgs"]
pub struct OrgInfo {
    pub tag: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name="orgs_users"]
pub struct NewOrgUser {
    pub org_id: i32,
    pub user_id: i32,
}

impl OrgUser {

    /// Creates a new organisation user connection that can be inserted into the database.
    pub fn new(org: &Org, user: &User) -> NewOrgUser {
        NewOrgUser {
            org_id: org.id,
            user_id: user.id,
        }
    }
}

impl Org {

    /// Finds a single organisation by tag.
    pub fn find_by_tag(query: &str, conn: &Connection) -> QueryResult<Org> {
        orgs
            .filter(tag.eq(query))
            .get_result::<Org>(conn)
    }

    /// Finds all users of the current organisation.
    pub fn find_all_users(self: &Self, conn: &Connection) -> QueryResult<Vec<String>> {
        orgs_users
            .filter(org_id.eq(&self.id))
            .inner_join(users)
            .select(username)
            .get_results::<String>(conn)
    }

    /// Creates a new organisation and stores it in the database.
    pub fn create(org: &OrgInfo, conn: &Connection) -> bool {
        match Org::find_by_tag(&org.tag, conn) {
            Ok(_org) => false,
            Err(_) => {
                let transaction = diesel::insert_into(orgs)
                    .values(org)
                    .execute(conn);
                match transaction {
                    Ok(_) => true,
                    Err(_) => false,
                }
            }
        }
    }

    /// Adds a single user to the organisation.
    pub fn add_user(self: &Self, user: &User, conn: &Connection) -> bool {
        let transaction = diesel::insert_into(orgs_users)
            .values(OrgUser::new(self, user))
            .execute(conn);
        match transaction {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Removes a single user from the organisation.
    pub fn remove_user(self: &Self, user: &User, conn: &Connection) -> bool {
        let transaction = diesel::delete(orgs_users
                .filter(org_id.eq(&self.id))
                .filter(user_id.eq(&user.id)))
            .execute(conn);
        match transaction {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
