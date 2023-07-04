extern crate dotenv;
extern crate dotenv_codegen;

use crate::settings::SETTINGS;
use dotenv::dotenv;
use lazy_static::lazy_static;
use liserk_shared::query::Query;
use std::{
    net::SocketAddr,
    sync::{
        mpsc::{self, Receiver, Sender},
        Mutex,
    },
    thread,
};
use tracing::{error, info};

mod app;
mod databases;
mod error;
mod logger;
mod routes;
mod settings;
mod utils;

#[derive(Debug, Clone)]
pub enum DatabaseCommand {
    Insert(Vec<String>),
    Query(Query),
}

lazy_static! {
    static ref TX: Mutex<Sender<DatabaseCommand>> = {
        let (tx, rx) = mpsc::channel();

        // Lancer un thread séparé pour écouter les messages envoyés sur le canal
        thread::spawn(move || {
            listen_channel(rx)
        });

        Mutex::new(tx)
    };
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let address = SocketAddr::from(([127, 0, 0, 1], SETTINGS.server_port));
    let app = app::create_app().await;

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

fn listen_channel(rx: Receiver<DatabaseCommand>) {
    while let Ok(command) = rx.recv() {
        let rt = tokio::runtime::Runtime::new().unwrap();

        let result = match command {
            DatabaseCommand::Insert(data) => rt.block_on(databases::insert_medications(data)),
            DatabaseCommand::Query(_) => todo!(),
        };
        if result.is_err() {
            error!("error with database {:?}", result);
        }
    }
}
