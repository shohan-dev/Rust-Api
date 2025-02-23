use actix_web::{HttpResponse, Responder};
use serde_json::json;

pub async fn hello() -> impl Responder {
    let response = json!({ "message": "Hello, World! 🌍","route": "/greet/shohan", "Route is Name" });
    HttpResponse::Ok().json(response)
}
