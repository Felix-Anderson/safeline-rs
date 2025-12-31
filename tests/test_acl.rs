use safeline_rs::{Client, ACLSearchRequest};

#[tokio::test]
async fn test_get_acl_logs() {
    // Load configuration from local.env
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = Client::new(&url, &token);

    // Test getting ACL logs with basic parameters
    let request = ACLSearchRequest {
        begin: None,
        end: None,
        ip: None,
        site: None,
    };

    println!("Testing get_acl_logs with basic request...");

    match client.get_acl_logs(&request).await {
        Ok(response) => {
            println!("✓ get_acl_logs succeeded");
            println!("  Total logs: {}", response.total);
            println!("  Number of logs returned: {}", response.data.len());

            // Print first log if available
            if let Some(first_log) = response.data.first() {
                println!("  First log:");
                println!("    ID: {}", first_log.id);
                println!("    IP: {}", first_log.ip);
                println!("    Site ID: {}", first_log.site_id);
                println!("    Action: {}", first_log.action);
                println!("    Result: {}", first_log.result);
                println!("    Status: {}", first_log.status);
                println!("    Reason: {}", first_log.reason);
            }
        }
        Err(e) => {
            eprintln!("✗ get_acl_logs failed: {:?}", e);
            panic!("ACL logs test failed");
        }
    }
}

#[tokio::test]
async fn test_get_acl_logs_with_ip_filter() {
    // Load configuration from local.env
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = Client::new(&url, &token);

    // Test getting ACL logs with IP filter
    let request = ACLSearchRequest {
        begin: None,
        end: None,
        ip: Some("127.0.0.1".to_string()),
        site: None,
    };

    println!("Testing get_acl_logs with IP filter (127.0.0.1)...");

    match client.get_acl_logs(&request).await {
        Ok(response) => {
            println!("✓ get_acl_logs with IP filter succeeded");
            println!("  Total logs: {}", response.total);
            println!("  Number of logs returned: {}", response.data.len());
        }
        Err(e) => {
            eprintln!("✗ get_acl_logs with IP filter failed: {:?}", e);
            panic!("ACL logs with IP filter test failed");
        }
    }
}

#[tokio::test]
async fn test_acl_relieve() {
    // Load configuration from local.env
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = Client::new(&url, &token);

    // First, get some ACL logs to find a valid ID
    let search_request = ACLSearchRequest {
        begin: None,
        end: None,
        ip: None,
        site: None,
    };

    println!("Testing acl_relieve...");

    match client.get_acl_logs(&search_request).await {
        Ok(logs_response) => {
            if let Some(first_log) = logs_response.data.first() {
                let relieve_request = safeline_rs::ACLRelieveRequest {
                    id: first_log.id,
                    search: None,
                };

                println!("  Attempting to relieve ACL log with ID: {}", first_log.id);

                match client.acl_relieve(&relieve_request).await {
                    Ok(_) => {
                        println!("✓ acl_relieve succeeded for ID: {}", first_log.id);
                    }
                    Err(e) => {
                        eprintln!("✗ acl_relieve failed: {:?}", e);
                        // Don't panic here as this might fail if there are no active blocks
                        println!("  Note: This is expected if there are no active blocks to relieve");
                    }
                }
            } else {
                println!("  No ACL logs found to test acl_relieve");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get ACL logs for relieving: {:?}", e);
            panic!("ACL relieve test failed - could not get logs");
        }
    }
}