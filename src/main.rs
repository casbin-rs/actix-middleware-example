#![allow(proc_macro_derive_resolution_fallback)]
#![warn(unused_must_use)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use actix_web::{get, web, Responder};
use actix_web::{App, HttpServer};
use std::env;

mod api;
mod config;
mod constants;
mod errors;
mod middleware;
mod models;
mod routers;
mod schema;
mod services;
mod utils;

#[get("/hello")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("This is {}!", name)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file, please add it");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_size: u32 = std::env::var("POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);

    let pool = config::db::migrate_and_config_db(&database_url, pool_size);

    // setup middleware and actor service here

    // add preset rules here

    // host http server
    HttpServer::new(move || App::new().app_data(pool.clone()))
        .bind(&app_url)?
        .run()
        .await?;

    Ok(())
}
