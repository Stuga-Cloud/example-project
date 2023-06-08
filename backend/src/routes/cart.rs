use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::Error;

pub fn create_route() -> Router {
    Router::new().route("/cart", get(get_cart))
}

async fn get_cart() -> Result<Json<Cart>, Error> {
    debug!("Returning cart");
    Ok(Json(Cart {
        status: "ok".to_owned(),
    }))
}

#[derive(Serialize, Deserialize, Debug)]
struct Cart {
    status: String,
}
