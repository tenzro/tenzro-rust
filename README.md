# Tenzro Rust SDK

Official Rust SDK for interacting with the Tenzro API.

## Installation

```toml
[dependencies]
tenzro = "0.1.0"
```

## Usage

```rust
use tenzro::{Client, CreateJobParams};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key".to_string(), "node-id".to_string())?;

    // Create a training job
    let mut config = HashMap::new();
    config.insert("epochs".to_string(), json!(100));
    config.insert("batch_size".to_string(), json!(32));

    let job = client.create_job(CreateJobParams {
        type_: "training".to_string(),
        model: "custom-model".to_string(),
        config,
    }).await?;
    println!("Created job: {}", job.id);

    // Get job status
    let job_status = client.get_job(&job.id).await?;
    println!("Job status: {:?}", job_status.status);

    // List jobs
    let jobs = client.list_jobs(10, 0).await?;
    println!("Total jobs: {}", jobs.len());

    // Cancel job
    let cancelled_job = client.cancel_job(&job.id).await?;
    println!("Cancelled job: {}", cancelled_job.id);

    Ok(())
}
```

## API Reference

### Client

```rust
Client::new(api_key: String, nid: String) -> Result<Client, Error>
```

### Methods

- `create_job(params: CreateJobParams) -> Result<Job, Error>`
- `get_job(job_id: &str) -> Result<Job, Error>`
- `list_jobs(limit: u32, offset: u32) -> Result<Vec<Job>, Error>`
- `cancel_job(job_id: &str) -> Result<Job, Error>`

## License

MIT License