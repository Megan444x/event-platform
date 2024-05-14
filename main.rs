use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use std::env;
use dotenv::dotenv;

#[derive(Serialize, Deserialize)]
struct RegisterEvent {
    name: String,
    attendees: u32,
}

async fn register_event(info: web::Json<RegisterEvent>) -> impl Responder {
    HttpResponse::Ok().json(format!("Event '{}' with {} attendees was successfully registered.", info.name, info.attendees))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_url = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/register_event")
                .route(web::post().to(register_event))
            )
            .service(
                web::resource("/health_check")
                .route(web::get().to(health_check))
            )
    })
    .bind(&server_url)?
    .workers(4)
    .run()
    .await
}