use std::env;

use liserk_client::stream::{AuthenticatedClient, UnconnectedClient};
use liserk_ope::simplified_version::encrypt_ope;
use liserk_shared::query::{CompoundQuery, Query, QueryType, SingleQueryBuilder};
use rug::Float;
use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecureStockProduct {
    name: String,
    price: f64,
    stock: Float,
}

pub async fn insert_medications(inserted_medications: Vec<String>) -> Result<(), Error> {
    let username = env::var("ZKD_USERNAME").unwrap();
    let password = env::var("ZKD_PASSWORD").unwrap();
    let db_url = env::var("ZKD_URL").unwrap();
    let client = UnconnectedClient::default();
    let client = client.connect(&db_url).await.unwrap();
    let mut client = client.authenticate(username, password).await.unwrap();

    let medications = match_medications(&inserted_medications);

    for (name, price, stock) in medications {
        let encrypted_stock = encrypt_ope(stock);

        let data = SecureStockProduct {
            name: name.to_string(),
            price,
            stock: encrypted_stock,
        };
        let data_bytes = liserk_client::serialize(&data)?;

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

fn match_medications(inserted_medications: &Vec<String>) -> Vec<(String, f64, f64)> {
    let medications: Vec<(String, f64, f64)> = vec![
        ("Paracetamol".to_owned(), 9.99, 120.0),
        ("Ibuprofen".to_owned(), 12.99, 80.0),
        ("Cough Syrup".to_owned(), 6.99, 150.0),
        ("Antihistamine".to_owned(), 8.99, 90.0),
        ("Multivitamin".to_owned(), 14.99, 100.0),
        ("Aspirin".to_owned(), 7.99, 130.0),
        ("Headache Relief Pills".to_owned(), 9.99, 75.0),
        ("Allergy Relief Spray".to_owned(), 12.99, 65.0),
        ("Cold & Flu Pack".to_owned(), 19.99, 50.0),
    ];

    medications
        .into_iter()
        .filter(|(name, _, _)| inserted_medications.contains(name))
        .collect()
}

pub async fn get_medications_for_inventory_management(
    db_client: &mut AuthenticatedClient,
) -> Result<(), Error> {
    let inventory_query = SingleQueryBuilder::default()
        .with_collection("medications".to_owned())
        .with_usecase("inventory_management".to_owned())
        .build();

    let inventory_result = db_client.query(Query::Single(inventory_query)).await?;

    println!(
        "Médicaments pour la gestion des stocks: {:?}",
        inventory_result
    );

    Ok(())
}

pub async fn get_medications_with_low_stock(
    db_client: &mut AuthenticatedClient,
) -> Result<(), Error> {
    let low_stock_query = SingleQueryBuilder::default()
        .with_collection("medications".to_owned())
        .with_usecase("statistical_analysis".to_owned())
        .with_encrypted_field_less_than("stock", encrypt_ope(80))
        .build();

    let usecase_query = SingleQueryBuilder::default()
        .with_collection("medications".to_owned())
        .with_usecase("inventory_management".to_owned())
        .build();

    let compound_query = CompoundQuery {
        query_type: QueryType::And,
        queries: vec![Query::Single(low_stock_query), Query::Single(usecase_query)],
    };

    let low_stock_result = db_client.query(Query::Compound(compound_query)).await?;

    println!("Médicaments avec stock bas: {:?}", low_stock_result);

    Ok(())
}
