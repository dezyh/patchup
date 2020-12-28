use crate::handlers::*;
use actix_web::web;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    info!("Configuring routes...");
    cfg.service(
        web::scope("/api")
            .service(status::get_status)
            .service(
                web::scope("/auth")
                    .service(web::resource("/signup").route(web::post().to(user::post_signup)))
                    .service(web::resource("/signin").route(web::post().to(user::signin)))
                    .service(web::resource("/signout").route(web::post().to(user::signout)))
            )
    );
}
