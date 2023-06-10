use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{self, PgPool};
use tracing::debug;

use crate::error::Error;

pub fn create_route() -> Router {
    Router::new().route("/products", get(get_products))
}

async fn get_products() -> Result<Json<Vec<Product>>, Error> {
    debug!("Sending product");
    const database_url: &str = "postgresql://myuser:mypassword@localhost/mydatabase";
    let pool = PgPool::connect(&database_url).await?;

    let rows = sqlx::query!(
        r#"
        SELECT id, name, href, price, description, image_src, image_alt
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
            price: row.price,
            description: row.description,
            image_src: row.image_src,
            image_alt: row.image_alt,
        };
        products.push(product);
    }
    Ok(Json(products))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: u32,
    name: String,
    href: String,
    price: f32,
    description: String,
    #[serde(rename = "imageSrc")]
    image_src: String,
    #[serde(rename = "imageAlt")]
    image_alt: String,
}
