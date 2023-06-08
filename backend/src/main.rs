use lazy_static::lazy_static;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::info;

mod app;
mod error;
mod logger;
mod routes;
mod settings;

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub environment: String,
    pub log_level: String,
    pub server_port: u16,
}

impl Settings {
    fn new() -> Self {
        Self {
            environment: "dev".to_string(),
            log_level: "debug".to_string(),
            server_port: 8008,
        }
    }
}

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new();
}

#[tokio::main]
async fn main() {
    let app = app::create_app().await;
    let address = SocketAddr::from(([127, 0, 0, 1], SETTINGS.server_port));

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
