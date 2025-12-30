use reqwest::get;
use safeline_rs::{IPGroupAppendRequest, IPGroupCreateRequest, IPGroupListRequest};
use serde_json::json;
use tracing::info;

use crate::base::get_client;

mod base;

#[tokio::test]
async fn test_create_ip_group() {
    let cli = base::get_client();
    let r = cli
        .ip_group_create(&IPGroupCreateRequest {
            comment: Some("test_group".to_string()),
            ips: vec![],
            reference: None,
        })
        .await;

    info!("test_create_ip_group:{}", json!(r.unwrap()).to_string());
}

#[tokio::test]
async fn test_get_ipgroup() {
    let cli = get_client();
    let res = cli
        .ip_group_list(&IPGroupListRequest { top: Some(10) })
        .await;
    info!("test_get_ipgroup:{}", json!(res.unwrap()))
}

#[tokio::test]
async fn test_add_ip_to_ipgroup() {
    let cli = get_client();
    let res = cli
        .ip_group_append(&IPGroupAppendRequest {
            ip_group_ids: vec![1444],
            ips: vec!["0.0.0.0".to_string()],
        })
        .await
        .unwrap();
}
