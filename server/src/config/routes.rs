use crate::controllers::*;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(web::scope("/api").service(status::get_status));
}
