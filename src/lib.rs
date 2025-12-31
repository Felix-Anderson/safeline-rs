//! # SafeLine SDK
//!
//! Rust SDK for SafeLine WAF API
//!
//! ## Example
//!
//! ```rust
//! use safeline_rs::{Client, IPGroupCreateRequest};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("https://your-safeline-host:9443", "your-api-token");
//!     
//!     let create_req = IPGroupCreateRequest {
//!         comment: Some("Test IP Group".to_string()),
//!         ips: vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()],
//!         reference: Some("test-reference".to_string()),
//!     };
//!     
//!     let response = client.ip_group_create(&create_req).await?;
//!     println!("IP Group created with ID: {}", response.data);
//!     
//!     Ok(())
//! }
//! ```

pub mod acl;
pub mod auth;
pub mod cert;
pub mod client;
pub mod error;
pub mod ipgroup;
pub mod ja4;
pub mod models;
pub mod other;
pub mod policy;
pub mod record;
pub mod site;
pub mod stat;
pub mod system;

pub use client::Client;
pub use error::{Error, Result};
pub use models::*;
