use std::env;

use axum::{
    extract,
    http::HeaderMap,
    routing::{get, post},
    Json, Router,
};
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{self, types::BigDecimal, PgPool};
use tracing::info;

use crate::{
    error::{Error, InteractionError},
    utils::serialize_bigdecimal,
    DatabaseCommand, TX,
};

pub fn create_route() -> Router {
    Router::new()
        .route("/products", get(get_products))
        .route("/products", post(process_checkout))
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Product {
    id: i32,
    name: String,
    href: String,
    #[serde(serialize_with = "serialize_bigdecimal")]
    price: BigDecimal,
    description: String,
    #[serde(rename = "imageSrc")]
    image_src: String,
    #[serde(rename = "imageAlt")]
    image_alt: String,
}

async fn get_products() -> Result<Json<Vec<Product>>, Error> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&database_url).await?;

    let products: Vec<Product> = sqlx::query_as(
        r#"
        SELECT id, name, href, ROUND(price::numeric, 2) AS price, description, image_src, image_alt
        FROM products
        "#,
    )
    .fetch_all(&pool)
    .await?;

    println!("{:?}", products);
    Ok(Json(products))
}

#[derive(Debug, Serialize)]
struct Medication {
    lambda_arg: LamdbaArg,
}

#[derive(Debug, Serialize)]
struct LamdbaArg {
    medications: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub message: String,
    pub interactions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Body {
    body: Message,
}

async fn process_checkout(
    extract::Json(candidates): extract::Json<Vec<i32>>,
) -> Result<Json<bool>, Error> {
    let database_url = env::var("DATABASE_URL")?;
    let token = env::var("LAMBDA_TOKEN")?;
    let url = env::var("LAMBDA_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let sql = sqlx::query_as::<_, (String,)>("SELECT name FROM PRODUCTS WHERE ID = ANY($1)")
        .bind(&candidates)
        .fetch_all(&pool)
        .await?;
    let data: Vec<String> = sql.iter().map(|p| p.0.clone()).collect();

    let payload = json!({
        "lambda_arg": {
            "medications": data.clone()
        }
    });

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("X-Gravitee-Api-Key", token.parse().unwrap());

    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .headers(headers)
        .json(&payload)
        .send()
        .await?;

    let body: Body = res.json().await?;
    let response = body.body;

    if response.interactions.is_some() {
        let error = InteractionError {
            message: response.message,
            interactions: response.interactions.expect("is interaction"),
        };
        info!("{:?}", error);
        return Err(Error::InteractionError(error));
    }

    let tx = TX.lock().unwrap();
    let _ = tx.send(DatabaseCommand::Insert(data));

    Ok(Json(true))
}
