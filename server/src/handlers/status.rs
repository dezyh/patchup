use actix_web::HttpResponse;

pub fn status() -> HttpResponse {
    HttpResponse::Ok().body("alive".to_string())
}
