// src/lib.rs
mod client;
mod error;
mod jobs;

pub use client::Client;
pub use error::Error;
pub use jobs::{Job, JobStatus, CreateJobParams};