// Module imports
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure route <1>
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

//Configure handler <2>
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello. EzyTutors is alive and kicking")
}

// Instantiate and run the HTTP server 
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Construct app and configure routes <3>
    let app = move || App::new().configure(general_routes);

    // Start HTTP server <4>
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
} 