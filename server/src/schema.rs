table! {
    orgs (id) {
        id -> Int4,
        tag -> Varchar,
        name -> Varchar,
    }
}

table! {
    orgs_users (id) {
        id -> Int4,
        org_id -> Int4,
        user_id -> Int4,
    }
}

table! {
    patches (id) {
        id -> Int4,
        repo -> Int4,
        created -> Timestamp,
        downloads -> Int4,
        bytesize -> Nullable<Int4>,
        platform -> Nullable<Varchar>,
        source_version -> Varchar,
        target_version -> Varchar,
        data -> Nullable<Varchar>,
    }
}

table! {
    repos (id) {
        id -> Int4,
        creator -> Int4,
        manager -> Nullable<Int4>,
        created -> Timestamp,
        description -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        verified -> Bool,
        session -> Varchar,
    }
}

joinable!(orgs_users -> orgs (org_id));
joinable!(orgs_users -> users (user_id));
joinable!(patches -> repos (repo));
joinable!(repos -> orgs (manager));
joinable!(repos -> users (creator));

allow_tables_to_appear_in_same_query!(
    orgs,
    orgs_users,
    patches,
    repos,
    users,
);
