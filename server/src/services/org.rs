use actix_web::{
    web,
    http::StatusCode,
};
use crate::{
    config::db::Pool,
    error::ServiceError,
    models::{
        org::{Org, OrgInfo},
        user::{User},
    }
};

pub fn create(org: OrgInfo, pool: &web::Data<Pool>) -> Result<OrgInfo, ServiceError> {
    match Org::create(&org, &pool.get().unwrap()) {
        true => Ok(org),
        false => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "bad error".to_string(),
        ))
    }
}

pub fn add_user(org: &str, user: &str, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Org::find_by_tag(&org, &pool.get().unwrap()) {
        Ok(org) => {
            match User::find_by_any(&user, &pool.get().unwrap()) {
                Ok(user) => {
                    org.add_user(&user, &pool.get().unwrap());
                    Ok(())
                },
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "user not found".to_string()))
            }
        },
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "org not found".to_string()))
    }
}

pub fn remove_user(org: &str, user: &str, pool: &web::Data<Pool>) -> Result<(), ServiceError> {
    match Org::find_by_tag(&org, &pool.get().unwrap()) {
        Ok(org) => {
            match User::find_by_any(&user, &pool.get().unwrap()) {
                Ok(user) => {
                    org.remove_user(&user, &pool.get().unwrap());
                    Ok(())
                },
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "user not found".to_string()))
            }
        },
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "org not found".to_string()))
    }
}

pub fn get(org: &str, pool: &web::Data<Pool>) -> Result<(Org, Vec<String>), ServiceError> {
    match Org::find_by_tag(&org, &pool.get().unwrap()) {
        Ok(org) => {
            match org.find_all_users(&pool.get().unwrap()) {
                Ok(users) => Ok((org, users)),
                Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "org not found".to_string()))
            }
        }
        Err(_) => Err(ServiceError::new(StatusCode::INTERNAL_SERVER_ERROR, "org not found".to_string()))
    }
}
