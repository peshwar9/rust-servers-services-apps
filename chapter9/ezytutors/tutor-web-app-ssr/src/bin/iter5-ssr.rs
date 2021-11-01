#[path = "../iter5/mod.rs"]
mod iter5;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use iter5::{dbaccess, errors, handler, model, routes, state::AppState};
use routes::app_config;
use sqlx::postgres::PgPool;
use std::env;

use tera::Tera;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    //Start HTTP server
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Listening on: {}", &host_port);
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await.unwrap();
    // Construct App State
    let shared_data = web::Data::new(AppState { db: db_pool });

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/iter5/**/*")).unwrap();

        App::new()
            .data(tera)
            .app_data(shared_data.clone())
            .configure(app_config)
    })
    .bind(&host_port)?
    .run()
    .await
}
