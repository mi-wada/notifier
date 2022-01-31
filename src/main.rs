use std::{
    collections::hash_map::DefaultHasher,
    env,
    hash::{Hash, Hasher},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = {
        let args: Vec<String> = env::args().collect();
        args[1].clone()
    };
    let body = reqwest::blocking::get(&url)?.text()?;

    println!("url: {}", url);
    let mut hasher = DefaultHasher::new();

    body.hash(&mut hasher);
    println!("body's hash: {}", hasher.finish());

    Ok(())
}
