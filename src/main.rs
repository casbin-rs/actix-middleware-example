#![allow(proc_macro_derive_resolution_fallback)]

use actix_web::{get, web, App, HttpServer, Responder};

mod api;
mod config;
mod constants;
mod errors;
mod middleware;
mod models;
mod routers;
mod services;
mod utils;

#[get("/hello")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("This is {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
