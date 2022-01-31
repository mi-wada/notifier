use std::{
    collections::hash_map::DefaultHasher,
    env,
    hash::{Hash, Hasher},
};

use aws_sdk_dynamodb::{model::AttributeValue, Client};

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
    let hash_value = hasher.finish();
    println!("body's hash: {}", hash_value);

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let get_req = client
        .get_item()
        .key("url", AttributeValue::S(url.to_string()))
        .table_name("notifer");
    let resp = get_req.send().await?;
    match resp.item() {
        Some(resp) => {
            let hash = resp.get("hash").unwrap().as_n().unwrap();
            if hash != &hash_value.to_string() {
                println!("🥟")
            }
        }
        None => {}
    }

    let put_req = client
        .put_item()
        .item("url", AttributeValue::S(url))
        .item("hash", AttributeValue::N(hash_value.to_string()))
        .table_name("notifer");
    let _resp = put_req.send().await?;

    Ok(())
}
