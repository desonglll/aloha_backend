use actix_web::{HttpResponse, Responder};

pub async fn health_check() -> impl Responder {
    println!("health_check");
    HttpResponse::Ok().finish()
}
