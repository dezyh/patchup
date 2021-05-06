table! {
    orgs (id) {
        id -> Int4,
        tag -> Nullable<Varchar>,
        name -> Nullable<Varchar>,
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
        repo -> Nullable<Int4>,
        created -> Nullable<Date>,
        downloads -> Nullable<Int4>,
        bytesize -> Nullable<Int4>,
        platform -> Nullable<Varchar>,
        source_version -> Nullable<Varchar>,
        target_version -> Nullable<Varchar>,
        data -> Nullable<Varchar>,
    }
}

table! {
    repos (id) {
        id -> Int4,
        creator -> Nullable<Int4>,
        manager -> Nullable<Int4>,
        created -> Nullable<Date>,
        description -> Nullable<Varchar>,
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
