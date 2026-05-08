use actix_web::{App, HttpServer, web, HttpResponse};
use actix_web::error::JsonPayloadError;
use dotenv::dotenv;
use std::env;
use config::database::connect_db;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_appender::rolling;
use tracing::subscriber::set_global_default;


mod config;
mod routes;
mod controllers;
mod services;
mod dto;
mod repositories;
mod middlewares;
mod utils;
mod models;

fn init_tracing() {
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(non_blocking) // log ke file
        .finish();

    set_global_default(subscriber).expect("Failed to set tracing subscriber");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    init_tracing();

    let db = connect_db().await;
    let port = env::var("APP_PORT").unwrap_or("8080".to_string());

    println!("🚀 Server running at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))

            // 🔥 GLOBAL JSON ERROR HANDLER
            .app_data(
                web::JsonConfig::default().error_handler(|err, _req| {

                    let message = match &err {
                        JsonPayloadError::Deserialize(e) => {
                            let msg = e.to_string();

                            if msg.contains("UUID") {
                                "Invalid UUID format".to_string()
                            } else {
                                format!("Invalid request: {}", msg)
                            }
                        }
                        _ => "Invalid request payload".to_string(),
                    };

                    actix_web::error::InternalError::from_response(
                        err,
                        HttpResponse::BadRequest().json(serde_json::json!({
                            "success": false,
                            "error": message
                        })),
                    )
                    .into()
                })
            )

            .configure(routes::init)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}