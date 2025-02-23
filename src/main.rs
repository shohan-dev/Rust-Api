mod routes;

use actix_web::{web, App, HttpServer};
use routes::{greet::greet, hello::hello};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Read PORT from environment variable or default to 8080
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    println!("🚀 Server running on port: {}", port);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Welcome to Rust App 🚀" }))
            .route("/hello", web::get().to(hello))
            .route("/greet/{name}", web::get().to(greet))
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}
