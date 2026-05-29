use actix_web::{
    error::JsonPayloadError,
    web,
    App,
    HttpResponse,
    HttpServer,
};
use dotenv::dotenv;
use std::env;

use tracing::{info, warn};
use tracing_appender::{
    non_blocking::WorkerGuard,
    rolling,
};
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

use config::database::connect_db;

mod config;
mod routes;
mod controllers;
mod services;
mod dto;
mod repositories;
mod middlewares;
mod utils;
mod models;

fn init_tracing() -> WorkerGuard {
    // logs/app.log
    let file_appender =
        rolling::daily("logs", "app.log");

    let (non_blocking, guard) =
        tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        // file logger only
        .with(
            fmt::layer()
                .with_ansi(false)
                .with_target(true)
                .with_thread_ids(true)
                .with_writer(non_blocking)
        )

        // only ERROR level
        .with(
            EnvFilter::new("error")
        )
        .init();

    guard
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // init tracing
    let _guard = init_tracing();

    // database connection
    let db = connect_db().await;

    // app port
    let port =
        env::var("APP_PORT")
            .unwrap_or("8080".to_string());

    println!(
        "🚀 Server running at http://localhost:{}",
        port
    );

    HttpServer::new(move || {
        App::new()

            // database injection
            .app_data(
                web::Data::new(db.clone())
            )

            // global json error handler
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _req| {

                        warn!(
                            error = ?err,
                            "Invalid JSON payload received"
                        );

                        let message = match &err {
                            JsonPayloadError::Deserialize(e) => {
                                let msg = e.to_string();

                                if msg.contains("UUID") {
                                    "Invalid UUID format"
                                        .to_string()
                                } else {
                                    format!(
                                        "Invalid request: {}",
                                        msg
                                    )
                                }
                            }

                            JsonPayloadError::ContentType => {
                                "Content-Type must be application/json"
                                    .to_string()
                            }

                            JsonPayloadError::Payload(e) => {
                                format!(
                                    "Payload error: {}",
                                    e
                                )
                            }

                            _ => {
                                "Invalid request payload"
                                    .to_string()
                            }
                        };

                        actix_web::error::InternalError::from_response(
                            err,
                            HttpResponse::BadRequest().json(
                                serde_json::json!({
                                    "success": false,
                                    "error": message
                                }),
                            ),
                        )
                        .into()
                    }),
            )

            // routes
            .configure(routes::init)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}