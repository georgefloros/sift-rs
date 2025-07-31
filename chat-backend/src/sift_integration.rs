use anyhow::{Context, Result};
use reqwest::{Client, ClientBuilder};
use std::env;
use tracing::{info, warn};

use crate::models::{SiftValidationRequest, SiftValidationResponse};

#[derive(Clone)]
pub struct SiftClient {
    client: Client,
    base_url: String,
}

impl SiftClient {
    pub fn new() -> Self {
        let base_url = env::var("SIFT_API_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let client = ClientBuilder::new()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self { client, base_url }
    }

    /// Validate a query against input data using the sift-rs API
    pub async fn validate_query(
        &self,
        query: &serde_json::Value,
        input: &serde_json::Value,
    ) -> Result<bool> {
        let url = format!("{}/validate", self.base_url);
        
        let request_body = vec![SiftValidationRequest {
            input: input.clone(),
            query: query.clone(),
        }];

        info!("Validating query against sift-rs API: {}", url);

        let response = self
            .client
            .post(&url)
            .json(&request_body)
            .send()
            .await
            .context("Failed to send request to sift-rs API")?;

        if !response.status().is_success() {
            warn!("Sift API returned error status: {}", response.status());
            return Err(anyhow::anyhow!(
                "Sift API returned error status: {}",
                response.status()
            ));
        }

        let validation_results: Vec<SiftValidationResponse> = response
            .json()
            .await
            .context("Failed to parse sift-rs API response")?;

        Ok(validation_results
            .first()
            .map(|r| r.valid)
            .unwrap_or(false))
    }

    /// Test connectivity to the sift-rs API
    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        
        info!("Checking health of sift-rs API: {}", url);

        match self.client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => {
                warn!("Failed to connect to sift-rs API: {}", e);
                Err(e.into())
            }
        }
    }
}
