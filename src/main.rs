// src/main.rs
use tenzro::{Client, CreateJobParams};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key".to_string(), "node-id".to_string())?;

    let mut config = HashMap::new();
    config.insert("epochs".to_string(), json!(100));
    config.insert("batch_size".to_string(), json!(32));

    let job = client.create_job(CreateJobParams {
        type_: "training".to_string(),
        model: "custom-model".to_string(),
        config,
    }).await?;
    
    println!("Created job: {}", job.id);
    Ok(())
}