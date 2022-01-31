use std::{
    collections::hash_map::DefaultHasher,
    env,
    hash::{Hash, Hasher},
};

use aws_sdk_dynamodb::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = {
        let args: Vec<String> = env::args().collect();
        args[1].clone()
    };
    let body = reqwest::get(&url).await?.text().await?;

    println!("url: {}", url);
    let mut hasher = DefaultHasher::new();

    body.hash(&mut hasher);
    println!("body's hash: {}", hasher.finish());

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let resp = req.send().await?;
    println!("tables: {:?}", resp.table_names);

    Ok(())
}
