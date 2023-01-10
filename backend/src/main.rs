mod config;
mod db;
mod api;
mod headermiddleware;

use actix_web::{App, HttpServer, middleware, web::Data};
use chrono::NaiveDate;
use diesel::{r2d2::{self, ConnectionManager}, SqliteConnection};
use log::{info, warn};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

/// This function loads environment variables, sets up a database connection pool, and starts an HTTP server for the backend
///
/// Returns:
///
/// The main function returns a Result<(), std::io::Error>
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //load env variables
    dotenvy::dotenv().expect("Failed to read .env file");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    //load various things from .env
    info!("Loading environment variables...");
    let app_host = std::env::var("APP_HOST").unwrap_or_else(|_| {
        warn!("environment variable APP_HOST not found; falling back to '127.0.0.1'.");
        "127.0.0.1".to_owned()
    });
    let app_port = std::env::var("APP_PORT").unwrap_or_else(|_| {
        warn!("environment variable APP_PORT not found; falling back to '8000'.");
        "8000".to_owned()
    });
    let db_url = std::env::var("DB_URL").expect("environment variable DB_URL not found.");

    //setup r2d2 with diesel
    let manager = ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    //start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(headermiddleware::InsertCacheHeader)
            .wrap(middleware::Logger::default())
            .app_data(Data::new(pool.clone()))
            .configure(config::config_services)
    })
        .bind((app_host, app_port.parse::<u16>().unwrap()))?
        .run()
        .await
}