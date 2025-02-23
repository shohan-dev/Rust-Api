use actix_web::{web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct GreetResponse {
    message: String,
}

pub async fn greet(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();
    let response = GreetResponse {
        message: format!("Hello, {}! 👋", name),
        _Time: format!("Time: {}", chrono::Local::now().naive_local()),
    };
    HttpResponse::Ok().json(response)
}
