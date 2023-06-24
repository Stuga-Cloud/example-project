use std::env;

use axum::{
    extract,
    routing::{get, post},
    Json, Router,
};
use liserk_client::UnconnectedClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{self, types::BigDecimal, PgPool};
use tracing::info;

use crate::{
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
        "SELECT name, price FROM PRODUCTS WHERE ID = ANY($1)",
        &candidates[..]
    )
    .fetch_all(&pool)
    .await?;
    let data: Vec<String> = sql.iter().map(|p| p.name.clone()).collect();
    let payload = Medication { medications: data };

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
    Ok(Json(true))
}

async fn insert_medications() -> Result<(), Error> {
    let username = env::var("ZKD_USERNAME").unwrap();
    let password = env::var("ZKD_PASSWORD").unwrap();
    let db_url = env::var("ZKD_URL").unwrap();
    let client = UnconnectedClient::default();
    let client = client.connect(&db_url).await.unwrap();
    let mut client = client.authenticate(username, password).await.unwrap();
    // Liste de médicaments avec nom, prix, et quantité en stock
    let medications = vec![
        ("Paracetamol", 9.99, 120),
        ("Ibuprofen", 12.99, 80),
        ("Cough Syrup", 6.99, 150),
        ("Antihistamine", 8.99, 90),
        ("Multivitamin", 14.99, 100),
        ("Aspirin", 7.99, 130),
        ("Headache Relief Pills", 9.99, 75),
        ("Allergy Relief Spray", 12.99, 65),
        ("Cold & Flu Pack", 19.99, 50),
    ];

    for (name, price, stock) in medications {
        let encrypted_stock = client.ope_encrypt(stock); // Ici, on assume que cette méthode existe

        let data = json!({
            "name": name,
            "price": price,
            "stock": encrypted_stock,
        }); // TODO use serde CBOR and the warpper of liserk_client
        let data_bytes = serde_json::to_vec(&data)?;

        let acl = vec!["manager".to_string(), "stock_analyst".to_string()];
        let usecases = vec![
            "inventory_management".to_string(),
            "statistical_analysis".to_string(),
        ];
        let collection = "medications".to_string();

        client.insert(collection, data_bytes, acl, usecases).await?;
    }

    Ok(())
}
