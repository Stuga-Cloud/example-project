use std::env;

use liserk_client::{
    deserialize, generate_key, load_key_from_file, save_key_to_file,
    stream::QueryResult,
    stream::{AuthenticatedClient, UnconnectedClient},
};
use liserk_shared::query::{
    CompoundQuery, CompoundQueryBuilder, Query, QueryType, SingleQuery, SingleQueryBuilder,
};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecureStockProduct {
    name: String,
    price: f64,
    stock: f64,
}

pub fn get_key() -> [u8; 32] {
    let path = env::var("KEY_PATH").expect("KEY_PATH env var exist");
    let key = load_key_from_file(&path);
    if key.is_err() {
        let key = generate_key();
        let _ = save_key_to_file(&key, &path);
        return key;
    }
    key.expect("checked before")
}

fn map_result_to_vec(query_result: QueryResult) -> Result<Vec<SecureStockProduct>, Error> {
    match query_result {
        QueryResult::EmptyResult => Ok(Vec::new()),
        QueryResult::SingleValue(value) => Ok(vec![deserialize(&value)?]),
        QueryResult::MultipleValues(values) => {
            let results: Result<Vec<SecureStockProduct>, Error> = values
                .iter()
                .map(|x| deserialize(x).map_err(Error::ZeroKnowledgeDatabase))
                .collect();
            results
        }
    }
}

pub async fn insert_medications(inserted_medications: Vec<String>) -> Result<(), Error> {
    let key = get_key();
    let username = env::var("ZKD_USERNAME")?;
    let password = env::var("ZKD_PASSWORD")?;
    let db_url = env::var("ZKD_URL")?;
    let client = UnconnectedClient::default();
    let client = client.connect(&db_url).await?;
    let mut client = client.authenticate(username, password, key).await?;

    let medications = match_medications(&inserted_medications);

    for (name, price, stock) in medications {
        let data = SecureStockProduct {
            name: name.to_string(),
            price,
            stock,
        };
        let data_bytes = liserk_client::serialize(&data)?;

        let acl = vec!["manager".to_string(), "stock_analyst".to_string()];
        let usecases = vec![
            "inventory_management".to_string(),
            "statistical_analysis".to_string(),
        ];
        let collection = "medications".to_string();
        let ope_collection = "medication:stock:ope".to_string();

        client
            .insert(
                collection,
                data_bytes,
                Vec::new(),
                acl.clone(),
                usecases.clone(),
            )
            .await?;
        client
            .insert_ope(stock, acl, usecases, ope_collection)
            .await?;
    }
    client.terminate_connection().await?;

    Ok(())
}

#[inline]
#[allow(dead_code)]
fn generate_nonce() -> [u8; 12] {
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    nonce
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
) -> Result<Vec<SecureStockProduct>, Error> {
    let inventory_query = SingleQueryBuilder::default()
        .with_collection("medications".to_owned())
        .with_usecase("inventory_management".to_owned())
        .build();

    let inventory_result = db_client.query(Query::Single(inventory_query)).await?;

    map_result_to_vec(inventory_result)
}

pub async fn get_medications_with_low_stock(
    db_client: &mut AuthenticatedClient,
) -> Result<Vec<SecureStockProduct>, Error> {
    let low_stock_query = SingleQueryBuilder::default()
        .with_collection("medication:stock:ope".to_owned())
        .with_usecase("statistical_analysis".to_owned())
        .with_encrypted_field_less_than(80.0)
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

    map_result_to_vec(low_stock_result)
}

pub async fn get_medications_with_low_stock_near_location(
    db_client: &mut AuthenticatedClient,
    latitude: f64,
    longitude: f64,
) -> Result<Vec<SecureStockProduct>, Error> {
    let nearest_warehouse_id = get_nearest_warehouse_id(db_client, latitude, longitude).await?;

    let low_stock_query = SingleQueryBuilder::default()
        .with_collection("medication:stock:ope".to_owned())
        .with_usecase("statistical_analysis".to_owned())
        .with_encrypted_field_less_than(80.0)
        .build();

    // TODO
    let warehouse_query = Query::GetById {
        id: nearest_warehouse_id.to_string(),
        collection: "stock".to_owned(),
    };

    let compound_query = CompoundQueryBuilder::default()
        .with_query_type(QueryType::And)
        .with_query(warehouse_query)
        .with_query(Query::Single(low_stock_query))
        .build();

    let low_stock_result = db_client.query(Query::Compound(compound_query)).await?;

    map_result_to_vec(low_stock_result)
}

#[derive(Debug, FromRow)]
pub struct Warehouse {
    pub id: i32,
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
}

async fn get_nearest_warehouse_id(
    db_client: &mut AuthenticatedClient,
    latitude: f64,
    longitude: f64,
) -> Result<i32, Error> {
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;
    let warehouses_query = "
        SELECT id, latitude, longitude
        FROM warehouses
    ";

    let warehouses: Vec<Warehouse> = sqlx::query_as(warehouses_query).fetch_all(&pool).await?;

    let mut nearest_warehouse_id = 0;
    let mut min_distance = f64::MAX;

    for warehouse in warehouses {
        let distance =
            haversine_distance(latitude, longitude, warehouse.latitude, warehouse.longitude);
        if distance < min_distance {
            min_distance = distance;
            nearest_warehouse_id = warehouse.id;
        }
    }

    Ok(nearest_warehouse_id)
}

fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let earth_radius_km = 6371.0;

    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();

    let a = (dlat / 2.0).sin() * (dlat / 2.0).sin()
        + lat1.to_radians().cos()
            * lat2.to_radians().cos()
            * (dlon / 2.0).sin()
            * (dlon / 2.0).sin();

    let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());

    earth_radius_km * c
}
