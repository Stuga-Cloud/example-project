const MEDICATIONS: [(&str, f32, usize)] = [
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

#[derive(Debug, Serialize, Deserialize)]
pub struct SecureStockProduct {
    name: String,
    price: f32,
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
        let encrypted_stock = client.ope_encrypt(stock);

        let data = SecureStockProduct {
            name: name.to_string(),
            price,
            stock: encrypted_stock,
        };
        let data_bytes = liserk_client::serialize(&data);

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

fn match_medications(inserted_medications: &Vec<String>) -> Vec<(&str, f64, i32)> {
    MEDICATIONS
        .iter()
        .filter(|(name, _, _)| inserted_medications.contains(&name.to_string()))
        .cloned()
        .collect()
}

pub async fn get_medications_for_inventory_management(
    db_client: &mut DatabaseClient,
) -> Result<(), Error> {
    let inventory_query = SingleQueryBuilder::default()
        .collection("medications")
        .usecase("inventory_management")
        .build();

    let inventory_result = db_client.query(Query::Single(inventory_query)).await?;

    println!(
        "Médicaments pour la gestion des stocks: {:?}",
        inventory_result
    );

    Ok(())
}

pub async fn get_medications_with_low_stock(db_client: &mut DatabaseClient) -> Result<(), Error> {
    let low_stock_query = SingleQueryBuilder::default()
        .collection("medications")
        .usecase("statistical_analysis")
        .with_encrypted_field_less_than("stock", db_client.ope_encrypt(80))
        .build();

    let usecase_query = SingleQueryBuilder::default()
        .collection("medications")
        .usecase("inventory_management")
        .build();

    let compound_query = CompoundQuery {
        query_type: QueryType::And,
        queries: vec![Query::Single(low_stock_query), Query::Single(usecase_query)],
    };

    let low_stock_result = db_client.query(Query::Compound(compound_query)).await?;

    println!("Médicaments avec stock bas: {:?}", low_stock_result);

    Ok(())
}
