table! {
    users (username) {
        username -> Varchar,
        email -> Varchar,
        passhash -> Varchar,
        firstname -> Varchar,
        lastname -> Varchar,
        verified -> Bool,
        created_at -> Timestamp,
    }
}
