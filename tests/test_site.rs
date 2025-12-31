// ============================================================================
// Site API Tests
// ============================================================================

/// Test listing all websites (GET /open/site)
#[tokio::test]
async fn test_list_websites() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing list_websites...");

    match client.list_websites().await {
        Ok(response) => {
            println!("✓ list_websites succeeded");
            println!("  Total websites: {}", response.total);
            println!("  Number of websites returned: {}", response.data.len());

            if let Some(first_site) = response.data.first() {
                println!("  First website:");
                println!("    ID: {}", first_site.id);
                println!("    Title: {}", first_site.title);
                println!("    Server names: {:?}", first_site.server_names);
                println!("    Ports: {:?}", first_site.ports);
                println!("    Upstreams: {:?}", first_site.upstreams);
                println!("    Group ID: {}", first_site.group_id);
                println!("    Is enabled: {}", first_site.is_enabled);
                println!("    Mode: {}", first_site.mode);
            }
        }
        Err(e) => {
            eprintln!("✗ list_websites failed: {:?}", e);
            panic!("List websites test failed");
        }
    }
}

/// Test creating a new website (POST /open/site)
#[tokio::test]
async fn test_create_website() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing create_website...");

    // First, get a list of groups to use for the new website
    match client.list_website_groups().await {
        Ok(groups_response) => {
            let group_id = if let Some(first_group) = groups_response.data.first() {
                first_group.id
            } else {
                // Create a default group if none exists
                println!("  No groups found, creating default group...");
                let create_group_req = safeline_rs::CreateGroupRequest {
                    name: "Default Group".to_string(),
                };
                match client.create_website_group(&create_group_req).await {
                    Ok(group_response) => group_response.data,
                    Err(e) => {
                        eprintln!("✗ Failed to create default group: {:?}", e);
                        let error_msg = e.to_string();
                        if error_msg.contains("分组管理未开启") || error_msg.contains("未开启") {
                            println!("  Note: Website group management is not enabled. Skipping test.");
                            return; // Skip the test
                        } else {
                            panic!("Create website test failed - could not create default group");
                        }
                    }
                }
            };

            let create_req = safeline_rs::WebsiteRequest {
                id: None,
                server_names: vec!["test.example.com".to_string()],
                ports: vec!["80".to_string()],
                upstreams: vec!["http://127.0.0.1:8080".to_string()],
                group_id,
                comment: Some("Test website created by SDK".to_string()),
                email: Some("admin@example.com".to_string()),
                cert_id: None,
                load_balance: Some("round-robin".to_string()),
                redirect_status_code: None,
                health_check: None,
                stat_enabled: Some(true),
                static_default: Some(false),
                index: None,
                r#type: None,
            };

            println!("  Creating website with server_name: test.example.com");

            match client.create_website(&create_req).await {
                Ok(response) => {
                    println!("✓ create_website succeeded");
                    println!("  Created website ID: {}", response.data);
                }
                Err(e) => {
                    eprintln!("✗ create_website failed: {:?}", e);
                    panic!("Create website test failed");
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get website groups: {:?}", e);
            panic!("Create website test failed - could not get groups");
        }
    }
}

/// Test getting website detail (GET /open/site/{id})
#[tokio::test]
async fn test_get_website_detail() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing get_website_detail...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Getting detail for website ID: {}", site_id);

                match client.get_website_detail(site_id).await {
                    Ok(detail_response) => {
                        println!("✓ get_website_detail succeeded");
                        println!("  Website ID: {}", detail_response.data.id);
                        println!("  Title: {}", detail_response.data.title);
                        println!("  Server names: {:?}", detail_response.data.server_names);
                        println!("  Ports: {:?}", detail_response.data.ports);
                        println!("  Upstreams: {:?}", detail_response.data.upstreams);
                        println!("  Group ID: {}", detail_response.data.group_id);
                        println!("  Is enabled: {}", detail_response.data.is_enabled);
                        println!("  Created at: {}", detail_response.data.created_at);
                        println!("  Updated at: {}", detail_response.data.updated_at);
                    }
                    Err(e) => {
                        eprintln!("✗ get_website_detail failed: {:?}", e);
                        panic!("Get website detail test failed");
                    }
                }
            } else {
                println!("  No websites found to test get_website_detail");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Get website detail test failed - could not get websites");
        }
    }
}

/// Test updating a website (PUT /open/site/{id})
#[tokio::test]
async fn test_update_website() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing update_website...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating website ID: {}", site_id);

                let update_req = safeline_rs::WebsiteRequest {
                    id: Some(site_id),
                    server_names: first_site.server_names.clone(),
                    ports: first_site.ports.clone(),
                    upstreams: first_site.upstreams.clone(),
                    group_id: first_site.group_id,
                    comment: Some("Updated comment by SDK test".to_string()),
                    email: first_site.email.clone(),
                    cert_id: first_site.cert_id,
                    load_balance: first_site.load_balance.clone(),
                    redirect_status_code: first_site.redirect_status_code,
                    health_check: first_site.health_check.clone(),
                    stat_enabled: first_site.stat_enabled,
                    static_default: first_site.static_default,
                    index: first_site.index,
                    r#type: first_site.r#type,
                };

                match client.update_website(site_id, &update_req).await {
                    Ok(_) => {
                        println!("✓ update_website succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ update_website failed: {:?}", e);
                        panic!("Update website test failed");
                    }
                }
            } else {
                println!("  No websites found to test update_website");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Update website test failed - could not get websites");
        }
    }
}

/// Test deleting websites (DELETE /open/site)
#[tokio::test]
async fn test_delete_websites() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing delete_websites...");

    // First create a test website to delete
    let create_req = safeline_rs::WebsiteRequest {
        id: None,
        server_names: vec!["test-delete.example.com".to_string()],
        ports: vec!["80".to_string()],
        upstreams: vec!["http://127.0.0.1:8080".to_string()],
        group_id: 1,
        comment: Some("Test website to be deleted".to_string()),
        email: None,
        cert_id: None,
        load_balance: None,
        redirect_status_code: None,
        health_check: None,
        stat_enabled: None,
        static_default: None,
        index: None,
        r#type: None,
    };

    match client.create_website(&create_req).await {
        Ok(create_response) => {
            let site_id = create_response.data;
            println!("  Created test website ID: {} for deletion", site_id);

            let delete_req = safeline_rs::DeleteWebsiteRequest {
                ids: vec![site_id],
            };

            match client.delete_websites(&delete_req).await {
                Ok(_) => {
                    println!("✓ delete_websites succeeded");
                }
                Err(e) => {
                    eprintln!("✗ delete_websites failed: {:?}", e);
                    panic!("Delete websites test failed");
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to create test website: {:?}", e);
            let error_msg = e.to_string();
            if error_msg.contains("分组管理未开启") || error_msg.contains("未开启") {
                println!("  Note: Website group management is not enabled. Skipping test.");
            } else {
                panic!("Delete websites test failed - could not create test website");
            }
        }
    }
}

/// Test updating website basic info (PUT /open/site/{id}/basic_info)
#[tokio::test]
async fn test_put_website_basic_info() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing put_website_basic_info...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating basic info for website ID: {}", site_id);

                let basic_info_req = safeline_rs::PutWebsiteBasicInfoRequest {
                    comment: Some("Updated basic info by SDK".to_string()),
                    group_id: first_site.group_id,
                    icon: Some("🌐".to_string()),
                };

                match client.put_website_basic_info(site_id, &basic_info_req).await {
                    Ok(_) => {
                        println!("✓ put_website_basic_info succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ put_website_basic_info failed: {:?}", e);
                        panic!("Put website basic info test failed");
                    }
                }
            } else {
                println!("  No websites found to test put_website_basic_info");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Put website basic info test failed - could not get websites");
        }
    }
}

/// Test updating website defense (PUT /open/site/{id}/defense)
#[tokio::test]
async fn test_put_website_defense() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing put_website_defense...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating defense for website ID: {}", site_id);

                let defense_req = safeline_rs::PutWebsiteDefenseRequest {
                    id: site_id,
                    enable: true,
                    pattern: None,
                    negate: None,
                    auth_source_ids: None,
                    auth_callback: None,
                    review: None,
                    tfa_enabled: None,
                    portal_redirect: None,
                };

                match client.put_website_defense(site_id, &defense_req).await {
                    Ok(_) => {
                        println!("✓ put_website_defense succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ put_website_defense failed: {:?}", e);
                        panic!("Put website defense test failed");
                    }
                }
            } else {
                println!("  No websites found to test put_website_defense");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Put website defense test failed - could not get websites");
        }
    }
}

/// Test updating website mode (PUT /open/site/mode)
#[tokio::test]
async fn test_put_website_mode() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing put_website_mode...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating mode for website ID: {}", site_id);

                let mode_req = safeline_rs::PutWebsiteModeRequest {
                    ids: vec![site_id],
                    mode: 0, // 0 = normal mode
                };

                match client.put_website_mode(&mode_req).await {
                    Ok(_) => {
                        println!("✓ put_website_mode succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ put_website_mode failed: {:?}", e);
                        panic!("Put website mode test failed");
                    }
                }
            } else {
                println!("  No websites found to test put_website_mode");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Put website mode test failed - could not get websites");
        }
    }
}

/// Test updating website chaos (PUT /open/site/{id}/chaos)
#[tokio::test]
async fn test_website_chaos() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing website_chaos...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating chaos for website ID: {}", site_id);

                let chaos_req = safeline_rs::WebsiteChaosRequest {
                    is_enabled: false,
                    html_encryption: Some(false),
                    html_fast_decryption: Some(false),
                    js_encryption: Some(false),
                    js_path: None,
                    img_encryption: Some(false),
                    img_text: Some(false),
                    img_watermark: Some(false),
                };

                match client.website_chaos(site_id, &chaos_req).await {
                    Ok(_) => {
                        println!("✓ website_chaos succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ website_chaos failed: {:?}", e);
                        panic!("Website chaos test failed");
                    }
                }
            } else {
                println!("  No websites found to test website_chaos");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Website chaos test failed - could not get websites");
        }
    }
}

/// Test updating website challenge (PUT /open/site/challenge)
#[tokio::test]
async fn test_website_challenge() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing website_challenge...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating challenge for website ID: {}", site_id);

                let challenge_req = safeline_rs::WebsiteChallengeRequest {
                    id: site_id,
                    enable: false,
                    level: None,
                    pattern: None,
                    negate: None,
                    replay: None,
                    expire: None,
                };

                match client.website_challenge(&challenge_req).await {
                    Ok(_) => {
                        println!("✓ website_challenge succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ website_challenge failed: {:?}", e);
                        panic!("Website challenge test failed");
                    }
                }
            } else {
                println!("  No websites found to test website_challenge");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Website challenge test failed - could not get websites");
        }
    }
}

/// Test setting website waiting room (PUT /open/site/{id}/waiting_room)
#[tokio::test]
async fn test_set_website_waiting_room() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing set_website_waiting_room...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Setting waiting room for website ID: {}", site_id);

                let waiting_room_req = safeline_rs::SetWebsiteWaitingRoomRequest {
                    is_enabled: false,
                    max_concurrent: 100,
                    session_timeout: 300,
                };

                match client.set_website_waiting_room(site_id, &waiting_room_req).await {
                    Ok(_) => {
                        println!("✓ set_website_waiting_room succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ set_website_waiting_room failed: {:?}", e);
                        panic!("Set website waiting room test failed");
                    }
                }
            } else {
                println!("  No websites found to test set_website_waiting_room");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Set website waiting room test failed - could not get websites");
        }
    }
}

/// Test website health check (PUT /open/site/healthcheck)
#[tokio::test]
async fn test_website_health_check() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing website_health_check...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let health_check_req = safeline_rs::WebsiteHealthCheckRequest {
                    hosts: first_site.server_names.clone(),
                    upstreams: first_site.upstreams.clone(),
                };

                println!("  Performing health check for hosts: {:?}", first_site.server_names);

                match client.website_health_check(&health_check_req).await {
                    Ok(_) => {
                        println!("✓ website_health_check succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ website_health_check failed: {:?}", e);
                        panic!("Website health check test failed");
                    }
                }
            } else {
                println!("  No websites found to test website_health_check");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Website health check test failed - could not get websites");
        }
    }
}

/// Test listing website groups (GET /open/site/group)
#[tokio::test]
async fn test_list_website_groups() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing list_website_groups...");

    match client.list_website_groups().await {
        Ok(response) => {
            println!("✓ list_website_groups succeeded");
            println!("  Number of groups: {}", response.data.len());

            if let Some(first_group) = response.data.first() {
                println!("  First group:");
                println!("    ID: {}", first_group.id);
                println!("    Name: {}", first_group.name);
                println!("    Position: {}", first_group.position);
                println!("    Site count: {}", first_group.site_count);
            }
        }
        Err(e) => {
            eprintln!("✗ list_website_groups failed: {:?}", e);
            panic!("List website groups test failed");
        }
    }
}

/// Test creating a website group (POST /open/site/group)
#[tokio::test]
async fn test_create_website_group() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing create_website_group...");

    let create_req = safeline_rs::CreateGroupRequest {
        name: "Test Group".to_string(),
    };

    match client.create_website_group(&create_req).await {
        Ok(response) => {
            println!("✓ create_website_group succeeded");
            println!("  Created group ID: {}", response.data);
        }
        Err(e) => {
            eprintln!("✗ create_website_group failed: {:?}", e);
            // Check if the error is due to feature not being enabled
            let error_msg = e.to_string();
            if error_msg.contains("分组管理未开启") || error_msg.contains("未开启") {
                println!("  Note: Website group management is not enabled. Skipping test.");
            } else {
                panic!("Create website group test failed");
            }
        }
    }
}

/// Test updating a website group (PUT /open/site/group/{id})
#[tokio::test]
async fn test_update_website_group() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing update_website_group...");

    match client.list_website_groups().await {
        Ok(groups_response) => {
            if let Some(first_group) = groups_response.data.first() {
                let group_id = first_group.id;
                println!("  Updating group ID: {}", group_id);

                let update_req = safeline_rs::UpdateGroupRequest {
                    name: "Updated Test Group".to_string(),
                };

                match client.update_website_group(group_id, &update_req).await {
                    Ok(_) => {
                        println!("✓ update_website_group succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ update_website_group failed: {:?}", e);
                        panic!("Update website group test failed");
                    }
                }
            } else {
                println!("  No groups found to test update_website_group");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get website groups: {:?}", e);
            panic!("Update website group test failed - could not get groups");
        }
    }
}

/// Test deleting a website group (DELETE /open/site/group/{id})
#[tokio::test]
async fn test_delete_website_group() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing delete_website_group...");

    // First create a test group to delete
    let create_req = safeline_rs::CreateGroupRequest {
        name: "Group to Delete".to_string(),
    };

    match client.create_website_group(&create_req).await {
        Ok(create_response) => {
            let group_id = create_response.data;
            println!("  Created test group ID: {} for deletion", group_id);

            match client.delete_website_group(group_id).await {
                Ok(_) => {
                    println!("✓ delete_website_group succeeded");
                }
                Err(e) => {
                    eprintln!("✗ delete_website_group failed: {:?}", e);
                    panic!("Delete website group test failed");
                }
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to create test group: {:?}", e);
            // Check if the error is due to feature not being enabled
            let error_msg = e.to_string();
            if error_msg.contains("分组管理未开启") || error_msg.contains("未开启") {
                println!("  Note: Website group management is not enabled. Skipping test.");
            } else {
                panic!("Delete website group test failed - could not create test group");
            }
        }
    }
}

/// Test sorting website groups (PUT /open/site/group/{id}/sort)
#[tokio::test]
async fn test_sort_website_groups() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing sort_website_groups...");

    match client.list_website_groups().await {
        Ok(groups_response) => {
            if groups_response.data.len() >= 2 {
                let first_group_id = groups_response.data[0].id;
                let second_group_id = groups_response.data[1].id;

                println!("  Sorting groups with IDs: {} and {}", first_group_id, second_group_id);

                let sort_req = safeline_rs::SortGroupRequest {
                    position: vec![second_group_id, first_group_id],
                };

                match client.sort_website_groups(first_group_id, &sort_req).await {
                    Ok(_) => {
                        println!("✓ sort_website_groups succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ sort_website_groups failed: {:?}", e);
                        panic!("Sort website groups test failed");
                    }
                }
            } else {
                println!("  Not enough groups to test sort_website_groups (need at least 2)");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get website groups: {:?}", e);
            panic!("Sort website groups test failed - could not get groups");
        }
    }
}

/// Test sorting websites (PUT /open/site/{id}/sort)
#[tokio::test]
async fn test_sort_websites() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing sort_websites...");

    match client.list_websites().await {
        Ok(list_response) => {
            if list_response.data.len() >= 2 {
                let first_site_id = list_response.data[0].id;
                let second_site_id = list_response.data[1].id;

                println!("  Sorting websites with IDs: {} and {}", first_site_id, second_site_id);

                let sort_req = safeline_rs::SortWebsiteRequest {
                    position: vec![second_site_id, first_site_id],
                };

                match client.sort_websites(first_site_id, &sort_req).await {
                    Ok(_) => {
                        println!("✓ sort_websites succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ sort_websites failed: {:?}", e);
                        panic!("Sort websites test failed");
                    }
                }
            } else {
                println!("  Not enough websites to test sort_websites (need at least 2)");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Sort websites test failed - could not get websites");
        }
    }
}

/// Test website group switch (PUT /open/site/group/switch)
#[tokio::test]
async fn test_website_group_switch() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing website_group_switch...");

    let switch_req = safeline_rs::GroupSwitchRequest {
        enable: true,
    };

    match client.website_group_switch(&switch_req).await {
        Ok(response) => {
            println!("✓ website_group_switch succeeded");
            println!("  Switch state: {}", response.data);
        }
        Err(e) => {
            eprintln!("✗ website_group_switch failed: {:?}", e);
            let error_msg = e.to_string();
            if error_msg.contains("分组管理未开启") || error_msg.contains("未开启") {
                println!("  Note: Website group management is not enabled. Skipping test.");
            } else {
                panic!("Website group switch test failed");
            }
        }
    }
}

/// Test getting nginx config (GET /open/site/{id}/nginx_config)
#[tokio::test]
async fn test_get_nginx_config() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing get_nginx_config...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Getting nginx config for website ID: {}", site_id);

                match client.get_nginx_config(site_id).await {
                    Ok(response) => {
                        println!("✓ get_nginx_config succeeded");
                        println!("  Config length: {} chars", response.data.config.len());
                        println!("  Custom location length: {} chars", response.data.custom_location.len());
                    }
                    Err(e) => {
                        eprintln!("✗ get_nginx_config failed: {:?}", e);
                        panic!("Get nginx config test failed");
                    }
                }
            } else {
                println!("  No websites found to test get_nginx_config");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Get nginx config test failed - could not get websites");
        }
    }
}

/// Test updating nginx config (PUT /open/site/{id}/nginx_config)
#[tokio::test]
async fn test_update_nginx_config() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing update_nginx_config...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating nginx config for website ID: {}", site_id);

                // First get current config
                match client.get_nginx_config(site_id).await {
                    Ok(current_config) => {
                        let update_req = safeline_rs::NginxConfigUpdateRequest {
                            custom_location: current_config.data.custom_location.clone(),
                        };

                        match client.update_nginx_config(site_id, &update_req).await {
                            Ok(_) => {
                                println!("✓ update_nginx_config succeeded");
                            }
                            Err(e) => {
                                eprintln!("✗ update_nginx_config failed: {:?}", e);
                                panic!("Update nginx config test failed");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("✗ Failed to get current nginx config: {:?}", e);
                        panic!("Update nginx config test failed - could not get current config");
                    }
                }
            } else {
                println!("  No websites found to test update_nginx_config");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Update nginx config test failed - could not get websites");
        }
    }
}

/// Test updating site group (PUT /open/site/{id}/group)
#[tokio::test]
async fn test_update_site_group() {
    let url = std::env::var("SAFELINE_URL")
        .unwrap_or_else(|_| "https://0.0.0.0:9443/api".to_string());
    let token = std::env::var("SAFELINE_TOKEN")
        .unwrap_or_else(|_| "zHtRjyNvVDpI6mrYhZdhEbPOVBWkdFXG".to_string());

    let client = safeline_rs::Client::new(&url, &token);

    println!("Testing update_site_group...");

    match client.list_websites().await {
        Ok(list_response) => {
            if let Some(first_site) = list_response.data.first() {
                let site_id = first_site.id;
                println!("  Updating group for website ID: {}", site_id);

                let update_req = safeline_rs::UpdateSiteGroupRequest {
                    group_id: first_site.group_id,
                };

                match client.update_site_group(site_id, &update_req).await {
                    Ok(_) => {
                        println!("✓ update_site_group succeeded");
                    }
                    Err(e) => {
                        eprintln!("✗ update_site_group failed: {:?}", e);
                        panic!("Update site group test failed");
                    }
                }
            } else {
                println!("  No websites found to test update_site_group");
            }
        }
        Err(e) => {
            eprintln!("✗ Failed to get websites: {:?}", e);
            panic!("Update site group test failed - could not get websites");
        }
    }
}