use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::Error;

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
