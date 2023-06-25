extern crate dotenv;
extern crate dotenv_codegen;

use dotenv::dotenv;
use std::net::SocketAddr;
use tracing::info;

use crate::settings::SETTINGS;

mod app;
mod databases;
mod error;
mod logger;
mod routes;
mod settings;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = app::create_app().await;
    let address = SocketAddr::from(([127, 0, 0, 1], SETTINGS.server_port));

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
