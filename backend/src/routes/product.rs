use std::env;

use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{self, types::BigDecimal, PgPool};
use tracing::info;

use crate::{
    databases,
    error::{Error, InteractionError},
    utils::serialize_bigdecimal,
};

pub fn create_route() -> Router {
    Router::new()
        .route("/products", get(get_products))
        .route("/products", post(process_checkout))
}

#[derive(Serialize, Deserialize, Debug)]
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

    let rows = sqlx::query!(
        r#"
        SELECT id, name, href, ROUND(price::numeric, 2) AS price, description, image_src, image_alt
        FROM products
        "#,
    )
    .fetch_all(&pool)
    .await?;

    let mut products: Vec<Product> = Vec::new();

    for row in rows {
        let product = Product {
            id: row.id,
            name: row.name,
            href: row.href,
            price: row.price.unwrap_or(BigDecimal::from(0)),
            description: row.description,
            image_src: row.image_src,
            image_alt: row.image_alt,
        };
        products.push(product);
    }
    println!("{:?}", products);
    Ok(Json(products))
}

#[derive(Debug, Serialize)]
struct Medication {
    medications: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    pub message: String,
    pub interactions: Option<Vec<Vec<String>>>,
}

async fn process_checkout(
    extract::Json(candidates): extract::Json<Vec<i32>>,
) -> Result<Json<bool>, Error> {
    let database_url = env::var("DATABASE_URL").unwrap();
    let token = env::var("LAMBDA_TOKEN").unwrap();
    let url = env::var("LAMBDA_URL").unwrap();
    let pool = PgPool::connect(&database_url).await?;
    let sql = sqlx::query!(
        "SELECT name FROM PRODUCTS WHERE ID = ANY($1)",
        &candidates[..]
    )
    .fetch_all(&pool)
    .await?;
    let data: Vec<String> = sql.iter().map(|p| p.name.clone()).collect();
    let payload = Medication {
        medications: data.clone(),
    };

    let client = reqwest::Client::new();
    let response: Message = client
        .post(url)
        .header("X-AUTH-TOKEN", token)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;
    if response.interactions.is_some() {
        let error = InteractionError {
            message: response.message,
            interactions: response.interactions.expect("is interaction"),
        };
        info!("{:?}", error);
        return Err(Error::InteractionError(error));
    }

    databases::insert_medications(data);

    Ok(Json(true))
}
