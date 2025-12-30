use crate::error::{Error, Result};
use reqwest::Client as HttpClient;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::debug;

/// SafeLine API Client
#[derive(Debug, Clone)]
pub struct Client {
    base_url: String,
    api_token: String,
    http_client: HttpClient,
}

impl Client {
    /// Create a new SafeLine API client
    ///
    /// # Arguments
    ///
    /// * `base_url` - Base URL of the SafeLine API (e.g., "https://your-safeline-host:9443/api")
    /// * `api_token` - API token for authentication
    pub fn new(base_url: &str, api_token: &str) -> Self {
        // Ensure base_url ends with /api if not already present
        let base_url = if !base_url.ends_with("/api") {
            format!("{}/api", base_url.trim_end_matches('/'))
        } else {
            base_url.to_string()
        };

        let http_client = HttpClient::builder()
            .timeout(std::time::Duration::from_secs(30))
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            base_url,
            api_token: api_token.to_string(),
            http_client,
        }
    }

    /// Execute an HTTP request
    async fn do_request<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        body: Option<&T>,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, endpoint);

        let mut request = self
            .http_client
            .request(method, &url)
            .header("X-SLCE-API-TOKEN", &self.api_token)
            .header("Content-Type", "application/json");

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send().await?;

        let response_text = response.text().await?;
        debug!("response_text:{response_text}");

        // Try to parse as API response first
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<R>>(&response_text) {
            if api_response.err.is_some() {
                return Err(Error::ApiError {
                    code: api_response.err,
                    message: api_response.msg.unwrap_or_default(),
                });
            }

            // Handle the data field - if it's None and R is Default, use default
            match api_response.data {
                Some(data) => Ok(data),
                None => {
                    // For empty responses, return a default value if possible
                    serde_json::from_str::<R>("{}").map_err(|_| {
                        Error::InvalidResponse(
                            "API returned no data but response type requires data".to_string(),
                        )
                    })
                }
            }
        } else {
            // If not API response format, try direct deserialization
            serde_json::from_str(&response_text).map_err(Error::from)
        }
    }

    /// Execute a GET request
    pub async fn get<R: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<R> {
        self.do_request::<(), R>(reqwest::Method::GET, endpoint, None)
            .await
    }

    /// Execute a POST request
    pub async fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R> {
        self.do_request(reqwest::Method::POST, endpoint, Some(body))
            .await
    }

    /// Execute a PUT request
    pub async fn put<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R> {
        self.do_request(reqwest::Method::PUT, endpoint, Some(body))
            .await
    }

    /// Execute a DELETE request
    pub async fn delete<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        body: &T,
    ) -> Result<R> {
        self.do_request(reqwest::Method::DELETE, endpoint, Some(body))
            .await
    }

    /// Execute a GET request with query parameters
    pub async fn get_with_query<R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        params: &HashMap<String, String>,
    ) -> Result<R> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self
            .http_client
            .request(reqwest::Method::GET, &url)
            .header("X-SLCE-API-TOKEN", &self.api_token);

        for (key, value) in params {
            request = request.query(&[(key, value)]);
        }

        let response = request.send().await?;

        let response_text = response.text().await?;
        debug!("response_text:{response_text}");
        // Try to parse as API response first
        if let Ok(api_response) = serde_json::from_str::<ApiResponse<R>>(&response_text) {
            if api_response.err.is_some() {
                return Err(Error::ApiError {
                    code: api_response.err,
                    message: api_response.msg.unwrap_or_default(),
                });
            }

            // Handle the data field
            match api_response.data {
                Some(data) => Ok(data),
                None => {
                    // For empty responses, return a default value if possible
                    serde_json::from_str::<R>("{}").map_err(|_| {
                        Error::InvalidResponse(
                            "API returned no data but response type requires data".to_string(),
                        )
                    })
                }
            }
        } else {
            // If not API response format, try direct deserialization
            serde_json::from_str(&response_text).map_err(Error::from)
        }
    }
}

/// Standard API response wrapper
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub msg: Option<String>,
    pub err: Option<String>,
    pub data: Option<T>,
}
