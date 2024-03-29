use crate::handlers::{status, user};
use actix_web::web::{get, post, resource, scope, ServiceConfig};

pub fn config_routes(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/api")
            .service(resource("status").route(get().to(status::status)))
            .service(
                scope("/user")
                    .service(resource("/signup").route(post().to(user::signup)))
                    .service(resource("/signin").route(post().to(user::signin)))
                    .service(resource("/signout").route(post().to(user::signout))),
            ),
    );
}
