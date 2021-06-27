use actix_files as fs;
use actix_web::{error, web, App, Error, HttpResponse, HttpServer, Result};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Listening on: 127.0.0.1:8080, open browser and visit have a try!");
    HttpServer::new(|| {
        App::new().service(fs::Files::new("/static", "./static").show_files_listing())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
