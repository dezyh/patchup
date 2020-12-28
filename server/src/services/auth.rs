use argonautica::{Hasher, Verifier};

pub fn hash_password(password: &str) -> String {
    Hasher::default()
        .with_password(password)
        .with_secret_key("1234")
        .hash()
        .unwrap()
}

pub fn verify_password(hash: &str, password: &str) -> bool {
    Verifier::default()
        .with_hash(hash)
        .with_password(password)
        .with_secret_key("1234")
        .verify()
        .unwrap()
}