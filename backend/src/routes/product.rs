use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::error::Error;

pub fn create_route() -> Router {
    Router::new().route("/products", get(get_products))
}

async fn get_products() -> Result<Json<Vec<Product>>, Error> {
    debug!("Sending product");
    let products = generate_fake_data();
    Ok(Json(products))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    id: u32,
    name: String,
    href: String,
    price: String,
    description: String,
    #[serde(rename = "imageSrc")]
    image_src: String,
    #[serde(rename = "imageAlt")]
    image_alt: String,
}

impl Product {
    pub fn new(
        id: u32,
        name: &str,
        href: &str,
        price: &str,
        description: &str,
        image_src: &str,
    ) -> Self {
        Self {
            id,
            name: name.to_owned(),
            href: href.to_owned(),
            price: price.to_owned(),
            description: description.to_owned(),
            image_src: image_src.to_owned(),
            image_alt: String::from("medecine fiole"),
        }
    }
}

fn generate_fake_data() -> Vec<Product> {
    let products = vec![
        Product::new(
            1,
            "Paracetamol",
            "#",
            "$9.99",
            "Relieves pain and reduces fever",
            "medecin_1.png",
        ),
        Product::new(
            2,
            "Ibuprofen",
            "#",
            "$12.99",
            "Effective for reducing inflammation and pain",
            "medecin_2.png",
        ),
        Product::new(
            3,
            "Aspirin",
            "#",
            "$7.99",
            "Used for pain relief and to reduce the risk of heart attack and stroke",
            "medecin_2.png",
        ),
        Product::new(
            4,
            "Cough Syrup",
            "#",
            "$6.99",
            "Provides relief from cough and congestion",
            "medecin_3.png",
        ),
        Product::new(
            5,
            "Antihistamine",
            "#",
            "$8.99",
            "Helps relieve allergy symptoms",
            "medecin_4.png",
        ),
        Product::new(
            6,
            "Multivitamin",
            "#",
            "$14.99",
            "Provides essential vitamins and minerals",
            "medecin_5.png",
        ),
        Product::new(
            7,
            "Cold & Flu Pack",
            "#",
            "$19.99",
            "Includes various medications for cold and flu symptoms",
            "medecin_pack.png",
        ),
    ];

    products
}