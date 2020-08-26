use actix_web::HttpResponse;

#[get("/status")]
fn get_status() -> HttpResponse {
    HttpResponse::Ok().body("alive".to_string())
}
