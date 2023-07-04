use std::env;

use axum::{routing::get, Json, Router};
use liserk_client::stream::UnconnectedClient;
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::{
    databases::{self, get_key},
    error::Error,
};

pub fn create_route() -> Router {
    Router::new().route("/stats", get(get_stats))
}

async fn get_stats() -> Result<Json<Stats>, Error> {
    debug!("Returning stats");
    Ok(Json(Stats {
        status: "ok".to_owned(),
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Stats {
    status: String,
}

pub async fn query_stats() -> Result<(), Error> {
    let key = get_key();
    let username = env::var("ZKD_USERNAME")?;
    let password = env::var("ZKD_PASSWORD")?;
    let db_url = env::var("ZKD_URL")?;
    let client = UnconnectedClient::default();
    let client = client.connect(&db_url).await?;
    let mut client = client.authenticate(username, password, key).await?;
    let data = databases::get_medications_with_low_stock(&mut client).await?;
    let data = databases::get_medications_for_inventory_management(&mut client).await?;
    Ok(())
}
