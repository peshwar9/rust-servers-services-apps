use std::env;

use actix_files as fs;
use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    // println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    println!("Listening on: {}, open browser and visit have a try!",addr);
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind(addr)?
    .run()
    .await
}
