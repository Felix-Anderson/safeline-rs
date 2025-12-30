//! SafeLine SDK IP Group Example
//!
//! This example demonstrates how to use the SafeLine SDK to manage IP groups.

use safeline_rs::{Client, IPGroupCreateRequest, IPGroupListRequest};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the client with your SafeLine host and API token
    let client = Client::new(
        "https://127.0.0.1:9443/api",
        "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
    );

    println!("=== SafeLine Rs IP Group Example ===\n");

    // 1. Create an IP group
    println!("1. Creating IP group...");
    let create_req = IPGroupCreateRequest {
        comment: Some("测试IP组".to_string()),
        ips: vec![
            "192.168.1.1".to_string(),
            "10.0.0.1".to_string(),
            "172.16.0.1".to_string(),
        ],
        reference: None,
    };

    match client.ip_group_create(&create_req).await {
        Ok(response) => {
            println!("✓ IP group created successfully with ID: {}", response.data);

            // Store the created ID for later use

            let created_id = response.data;

            // 2. List IP groups
            println!("\n2. Listing IP groups...");
            let list_req = IPGroupListRequest { top: Some(10) };

            match client.ip_group_list(&list_req).await {
                Ok(list_response) => {
                    println!("✓ Found {} IP groups:", list_response.total);
                    for (index, group) in list_response.nodes.iter().enumerate() {
                        println!(
                            "  {}. ID: {}, Comment: {:?}, IP Count: {}, Reference: {:?}",
                            index + 1,
                            group.id.unwrap_or(0),
                            group.comment,
                            group.total.unwrap_or(0),
                            group.reference
                        );
                    }
                }
                Err(e) => println!("✗ Failed to list IP groups: {}", e),
            }

            // 3. Get IP group details
            println!("\n3. Getting IP group details...");
            match client
                .ip_group_detail(&safeline_rs::IPGroupDetailRequest { id: created_id })
                .await
            {
                Ok(detail_response) => {
                    let group = detail_response.data;
                    println!("✓ IP Group Details:");
                    println!("  ID: {}", group.id.unwrap_or(0));
                    println!("  Comment: {:?}", group.comment);
                    println!("  IPs: {:?}", group.ips);
                    println!("  Total: {}", group.total.unwrap_or(0));
                    println!("  Reference: {:?}", group.reference);
                    println!("  Updated At: {:?}", group.updated_at);
                    println!("  Builtin: {:?}", group.builtin);
                }
                Err(e) => println!("✗ Failed to get IP group details: {}", e),
            }

            // 4. Update IP group
            println!("\n4. Updating IP group...");
            let update_req = safeline_rs::IPGroupUpdateRequest {
                id: created_id,
                builtin: Some(false),
                comment: Some("更新后的IP组".to_string()),
                ips: Some(vec![
                    "192.168.1.1".to_string(),
                    "10.0.0.1".to_string(),
                    "172.16.0.1".to_string(),
                    "172.16.0.2".to_string(),
                ]),
                reference: Some("updated-reference".to_string()),
            };

            match client.ip_group_update(&update_req).await {
                Ok(_) => println!("✓ IP group updated successfully"),
                Err(e) => println!("✗ Failed to update IP group: {}", e),
            }

            // 5. Append IPs to IP group
            println!("\n5. Appending IPs to IP group...");
            let append_req = safeline_rs::IPGroupAppendRequest {
                ip_group_ids: vec![created_id],
                ips: vec!["172.16.0.3".to_string(), "172.16.0.4".to_string()],
            };

            match client.ip_group_append(&append_req).await {
                Ok(_) => println!("✓ IPs appended to IP group successfully"),
                Err(e) => println!("✗ Failed to append IPs to IP group: {}", e),
            }

            // 6. Get search engine spider group ID

            println!("\n6. Getting search engine spider group ID...");

            match client.ip_group_crawler().await {
                Ok(crawler_response) => {
                    println!("✓ Search engine spider group ID: {}", crawler_response.data);

                    // 7. Update search engine spider IPs
                    println!("\n7. Updating search engine spider IPs...");
                    match client.ip_group_crawler_update().await {
                        Ok(_) => println!("✓ Search engine spider IPs updated successfully"),
                        Err(e) => println!("✗ Failed to update search engine spider IPs: {}", e),
                    }
                }
                Err(e) => println!("✗ Failed to get search engine spider group ID: {}", e),
            }

            // 8. Get IPs by link
            println!("\n8. Getting IPs by link...");
            let link_req = safeline_rs::IPGroupLinkRequest {
                href: "https://example.com".to_string(),
            };

            match client.ip_group_link(&link_req).await {
                Ok(link_response) => {
                    println!("✓ Found {} IPs through link:", link_response.data.ips.len());
                    for (index, ip) in link_response.data.ips.iter().enumerate() {
                        println!("  {}. {}", index + 1, ip);
                    }
                }
                Err(e) => println!("✗ Failed to get IPs by link: {}", e),
            }

            // 9. Create IP group by link
            println!("\n9. Creating IP group by link...");
            let create_by_link_req = safeline_rs::IPGroupCreateByLinkRequest {
                comment: "通过链接创建的IP组".to_string(),
                url: "https://example.com".to_string(),
            };

            match client.ip_group_create_by_link(&create_by_link_req).await {
                Ok(_) => println!("✓ IP group created by link successfully"),
                Err(e) => println!("✗ Failed to create IP group by link: {}", e),
            }

            // 10. Delete IP group
            println!("\n10. Deleting IP group...");
            let delete_req = safeline_rs::IPGroupDeleteRequest {
                ids: vec![created_id],
            };

            match client.ip_group_delete(&delete_req).await {
                Ok(_) => println!("✓ IP group deleted successfully"),
                Err(e) => println!("✗ Failed to delete IP group: {}", e),
            }
        }
        Err(e) => {
            println!("✗ Failed to create IP group: {}", e);
            println!("Make sure your SafeLine host and API token are correct.");
        }
    }

    println!("\n=== Example completed ===");
    Ok(())
}
