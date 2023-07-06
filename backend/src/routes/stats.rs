use std::{env, sync::mpsc};

use axum::{routing::get, Json, Router};
use liserk_client::stream::UnconnectedClient;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    databases::{self, get_key, SecureStockProduct},
    error::Error,
    TX,
};

pub fn create_route() -> Router {
    Router::new().route("/stats", get(get_stats))
}

async fn get_stats() -> Result<Json<StatsResult>, Error> {
    debug!("Returning stats");

    let (response_tx, response_rx) = mpsc::channel();
    let tx = TX.lock().unwrap();
    let _ = tx.send(crate::DatabaseCommand::Query(response_tx));
    let result = response_rx.recv().expect("error in recv get_stats");
    Ok(Json(result))
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatsResult {
    low_stock: Vec<SecureStockProduct>,
    invertory: Vec<SecureStockProduct>,
    nearest_warehouse_stock: Vec<SecureStockProduct>,
}
struct Coordinates {
    latitude: f64,
    longitude: f64,
}

pub async fn query_stats(tx: mpsc::Sender<StatsResult>) -> Result<(), Error> {
    let key = get_key();
    let username = env::var("ZKD_USERNAME")?;
    let password = env::var("ZKD_PASSWORD")?;
    let db_url = env::var("ZKD_URL")?;
    let client = UnconnectedClient::default();
    let client = client.connect(&db_url).await?;

    let paris = Coordinates {
        latitude: 48.8566,
        longitude: 2.3522,
    };

    let mut client = client.authenticate(username, password, key).await?;
    let low_stock = databases::get_medications_with_low_stock(&mut client).await?;
    let invertory = databases::get_medications_for_inventory_management(&mut client).await?;
    let nearest_warehouse_stock = databases::get_medications_with_low_stock_near_location(
        &mut client,
        paris.latitude,
        paris.longitude,
    )
    .await?;
    let result = StatsResult {
        low_stock,
        invertory,
        nearest_warehouse_stock,
    };
    let _ = tx.send(result);
    Ok(())
}
