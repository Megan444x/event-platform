use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize)]
struct EventRegistrationData {
    name: String,
    attendee_count: u32,
}

async fn handle_event_registration(form_data: web::Json<EventRegistrationData>) -> impl Responder {
    HttpResponse::Ok().json(format!(
        "Event '{}' with {} attendees was successfully registered.",
        form_data.name, form_data.attendee_count
    ))
}

async fn perform_health_check() -> impl Responder {
    HttpResponse::Ok().body("Server is up and running!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let server_address = env::var("SERVER_URL").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/register_event")
                    .route(web::post().to(handle_event_registration))
            )
            .service(
                web::resource("/health_check")
                    .route(web::get().to(perform_health_check))
            )
    })
    .bind(&server_address)?
    .workers(4) // Fine-tuning the number of workers may help with performance depending on your deployment environment.
    .run()
    .await
}