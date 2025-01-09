// src/client.rs
use reqwest::{Client as ReqwestClient, header};
use crate::{Job, CreateJobParams, Error};

pub struct Client {
    client: ReqwestClient,
    base_url: String,
    api_key: String,
    nid: String,
}

impl Client {
    pub fn new(api_key: String, nid: String) -> Result<Self, Error> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );
        
        let client = ReqwestClient::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            client,
            base_url: "https://api.tenzro.com/v0".to_string(),
            api_key,
            nid,
        })
    }

    pub async fn create_job(&self, params: CreateJobParams) -> Result<Job, Error> {
        let url = format!("{}/jobs", self.base_url);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Node-ID", &self.nid)
            .json(&params)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::ResponseError(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }

    pub async fn get_job(&self, job_id: &str) -> Result<Job, Error> {
        let url = format!("{}/jobs/{}", self.base_url, job_id);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Node-ID", &self.nid)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::ResponseError(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }

    pub async fn list_jobs(&self, limit: u32, offset: u32) -> Result<Vec<Job>, Error> {
        let url = format!("{}/jobs", self.base_url);
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Node-ID", &self.nid)
            .query(&[("limit", limit), ("offset", offset)])
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::ResponseError(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }

    pub async fn cancel_job(&self, job_id: &str) -> Result<Job, Error> {
        let url = format!("{}/jobs/{}/cancel", self.base_url, job_id);
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("X-Node-ID", &self.nid)
            .send()
            .await?;

        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            Err(Error::ResponseError(format!(
                "API request failed with status: {}",
                response.status()
            )))
        }
    }
}
