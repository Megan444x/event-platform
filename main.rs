use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct EventInfo {
    name: String,
    attendee_count: u32,
}

async fn register_event(event_info: web::Json<EventInfo>) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Event '{}' with {} attendees was successfully registered.",
        event_info.name, event_info.attendee_count
    ))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
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
    .bind(&server_address)?
    .workers(4)
    .run()
    .await
}