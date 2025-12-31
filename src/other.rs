use crate::client::Client;
use crate::error::Result;
use crate::models::*;
use std::collections::HashMap;

impl Client {
    // ============================================================================
    // Alarm
    // ============================================================================

    /// Get alarm config (GET /alarm)
    ///
    /// Retrieves alarm configuration.
    ///
    /// # Returns
    ///
    /// Returns alarm configuration
    pub async fn get_alarm_config(&self) -> Result<Alarm> {
        self.get("/alarm").await
    }

    /// Update alarm config (PUT /alarm)
    ///
    /// Updates alarm configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Alarm configuration request
    pub async fn update_alarm_config(&self, request: &PutAlarmConfigRequest) -> Result<AlarmConfigResponse> {
        self.put("/alarm", request).await
    }

    /// Test alarm config (POST /alarm/test)
    ///
    /// Tests alarm configuration.
    ///
    /// # Returns
    ///
    /// Returns test result
    pub async fn test_alarm_config(&self) -> Result<AlarmConfigResponse> {
        self.post("/alarm/test", &()).await
    }

    // ============================================================================
    // Portal
    // ============================================================================

    /// Get portal (GET /open/portal)
    ///
    /// Retrieves portal configuration.
    ///
    /// # Returns
    ///
    /// Returns portal configuration
    pub async fn get_portal(&self) -> Result<PortalGetResponse> {
        self.get("/open/portal").await
    }

    /// Get portal proxy config (GET /open/portal/proxy_config)
    ///
    /// Retrieves portal proxy configuration.
    ///
    /// # Returns
    ///
    /// Returns portal proxy configuration
    pub async fn get_portal_proxy_config(&self) -> Result<ProxyConfig> {
        self.get("/open/portal/proxy_config").await
    }

    /// Put portal proxy config (POST /open/portal/proxy_config)
    ///
    /// Updates portal proxy configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Proxy configuration request
    pub async fn put_portal_proxy_config(&self, request: &PortalProxyConfigRequest) -> Result<()> {
        self.post("/open/portal/proxy_config", request).await
    }

    /// Get portal style (GET /open/portal/style)
    ///
    /// Retrieves portal style configuration.
    ///
    /// # Returns
    ///
    /// Returns portal style configuration
    pub async fn get_portal_style(&self) -> Result<PortalStyle> {
        self.get("/open/portal/style").await
    }

    /// Put portal style (PUT /commercial/portal/style)
    ///
    /// Updates portal style configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Portal style request
    pub async fn put_portal_style(&self, request: &PortalStyle) -> Result<()> {
        self.put("/commercial/portal/style", request).await
    }

    // ============================================================================
    // Report
    // ============================================================================

    /// List reports (GET /business/report)
    ///
    /// Retrieves a list of reports.
    ///
    /// # Arguments
    ///
    /// * `name` - Report name filter
    /// * `page` - Page number
    /// * `page_size` - Page size
    ///
    /// # Returns
    ///
    /// Returns a list of reports
    pub async fn list_reports(&self, name: Option<String>, page: i32, page_size: i32) -> Result<ReportListResponse> {
        let mut params = HashMap::new();
        if let Some(name) = name {
            params.insert("name".to_string(), name);
        }
        params.insert("page".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        self.get_with_query("/business/report", &params).await
    }

    /// Create report (POST /business/report)
    ///
    /// Creates a new report.
    ///
    /// # Arguments
    ///
    /// * `request` - Report creation request
    pub async fn create_report(&self, request: &ReportCreateRequest) -> Result<()> {
        self.post("/business/report", request).await
    }

    /// Get report detail (GET /business/report/{id})
    ///
    /// Retrieves detailed information about a specific report.
    ///
    /// # Arguments
    ///
    /// * `id` - Report ID
    ///
    /// # Returns
    ///
    /// Returns detailed report information
    pub async fn get_report_detail(&self, id: i32) -> Result<ReportGetResponse> {
        self.get(&format!("/business/report/{}", id)).await
    }

    /// Delete report (DELETE /business/report/{id})
    ///
    /// Deletes a report.
    ///
    /// # Arguments
    ///
    /// * `id` - Report ID
    pub async fn delete_report(&self, id: i32) -> Result<()> {
        self.delete(&format!("/business/report/{}", id), &()).await
    }

    // ============================================================================
    // Anti Tamper
    // ============================================================================

    /// List site anti tamper (GET /business/site/{site_id}/anti_tamper)
    ///
    /// Retrieves anti tamper configuration for a site.
    ///
    /// # Arguments
    ///
    /// * `site_id` - Site ID
    /// * `changed` - Filter by changed status
    ///
    /// # Returns
    ///
    /// Returns anti tamper configuration
    pub async fn list_site_anti_tamper(&self, site_id: i32, changed: Option<bool>) -> Result<AntiTamperListResponse> {
        let mut params = HashMap::new();
        if let Some(changed) = changed {
            params.insert("changed".to_string(), changed.to_string());
        }
        let endpoint = format!("/business/site/{}/anti_tamper", site_id);
        if params.is_empty() {
            self.get(&endpoint).await
        } else {
            self.get_with_query(&endpoint, &params).await
        }
    }

    /// Create anti tamper (POST /business/site/{site_id}/anti_tamper)
    ///
    /// Creates anti tamper configuration for a site.
    ///
    /// # Arguments
    ///
    /// * `site_id` - Site ID
    /// * `request` - Anti tamper creation request
    pub async fn create_anti_tamper(&self, site_id: i32, request: &AntiTamperCreateRequest) -> Result<()> {
        self.post(&format!("/business/site/{}/anti_tamper", site_id), request).await
    }

    /// Refresh anti tamper (PUT /business/site/{site_id}/anti_tamper)
    ///
    /// Refreshes anti tamper configuration for a site.
    ///
    /// # Arguments
    ///
    /// * `site_id` - Site ID
    pub async fn refresh_anti_tamper(&self, site_id: i32) -> Result<()> {
        self.put(&format!("/business/site/{}/anti_tamper", site_id), &()).await
    }

    /// Get anti tamper detail (GET /business/anti_tamper/{id})
    ///
    /// Retrieves detailed information about anti tamper.
    ///
    /// # Arguments
    ///
    /// * `id` - Anti tamper ID
    ///
    /// # Returns
    ///
    /// Returns detailed anti tamper information
    pub async fn get_anti_tamper_detail(&self, id: i32) -> Result<AntiTamperDetailResponse> {
        self.get(&format!("/business/anti_tamper/{}", id)).await
    }

    /// Update anti tamper (PUT /business/anti_tamper/{id})
    ///
    /// Updates anti tamper configuration.
    ///
    /// # Arguments
    ///
    /// * `id` - Anti tamper ID
    pub async fn update_anti_tamper(&self, id: i32) -> Result<()> {
        self.put(&format!("/business/anti_tamper/{}", id), &()).await
    }

    /// Delete anti tamper (DELETE /business/anti_tamper/{id})
    ///
    /// Deletes anti tamper configuration.
    ///
    /// # Arguments
    ///
    /// * `id` - Anti tamper ID
    pub async fn delete_anti_tamper(&self, id: i32) -> Result<()> {
        self.delete(&format!("/business/anti_tamper/{}", id), &()).await
    }

    /// Get anti tamper page (GET /business/anti_tamper/{id}/page)
    ///
    /// Retrieves anti tamper page content.
    ///
    /// # Arguments
    ///
    /// * `id` - Anti tamper ID
    /// * `content` - Content type (1 or 2)
    ///
    /// # Returns
    ///
    /// Returns anti tamper page content
    pub async fn get_anti_tamper_page(&self, id: i32, content: i32) -> Result<String> {
        let mut params = HashMap::new();
        params.insert("content".to_string(), content.to_string());
        self.get_with_query(&format!("/business/anti_tamper/{}/page", id), &params).await
    }

    /// Get anti tamper resource (GET /business/site/{site_id}/anti_tamper/resource/{resource_id})
    ///
    /// Retrieves anti tamper resource.
    ///
    /// # Arguments
    ///
    /// * `site_id` - Site ID
    /// * `resource_id` - Resource ID
    ///
    /// # Returns
    ///
    /// Returns anti tamper resource
    pub async fn get_anti_tamper_resource(&self, site_id: i32, resource_id: i32) -> Result<Resource> {
        self.get(&format!("/business/site/{}/anti_tamper/resource/{}", site_id, resource_id)).await
    }

    // ============================================================================
    // Auth Defense
    // ============================================================================

    /// List auth defense users (GET /open/auth_defense/user)
    ///
    /// Retrieves a list of auth defense users.
    ///
    /// # Returns
    ///
    /// Returns a list of auth defense users
    pub async fn list_auth_defense_users(&self) -> Result<AuthDefenseUserListResponse> {
        self.get("/open/auth_defense/user").await
    }

    /// Get auth defense user (GET /open/auth_defense/user/{user_id})
    ///
    /// Retrieves detailed information about an auth defense user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    ///
    /// # Returns
    ///
    /// Returns detailed user information
    pub async fn get_auth_defense_user(&self, user_id: i32) -> Result<AuthDefenseGetUserResponse> {
        self.get(&format!("/open/auth_defense/user/{}", user_id)).await
    }

    /// Create auth defense user (POST /open/auth_defense/user)
    ///
    /// Creates a new auth defense user.
    ///
    /// # Arguments
    ///
    /// * `request` - User creation request
    ///
    /// # Returns
    ///
    /// Returns the created user ID
    pub async fn create_auth_defense_user(&self, request: &AuthDefenseCreateUserRequest) -> Result<CreateUserResponse> {
        self.post("/open/auth_defense/user", request).await
    }

    /// Update auth defense user (PUT /open/auth_defense/user/{user_id})
    ///
    /// Updates an auth defense user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `request` - User update request
    pub async fn update_auth_defense_user(&self, user_id: i32, request: &AuthDefenseUpdateUserRequest) -> Result<()> {
        self.put(&format!("/open/auth_defense/user/{}", user_id), request).await
    }

    /// Delete auth defense users (DELETE /open/auth_defense/user)
    ///
    /// Deletes auth defense users.
    ///
    /// # Arguments
    ///
    /// * `request` - Delete user request
    pub async fn delete_auth_defense_users(&self, request: &AuthDefenseDeleteUserRequest) -> Result<()> {
        self.delete("/open/auth_defense/user", request).await
    }

    /// Reset auth defense user TOTP (POST /open/auth_defense/user/{user_id}/reset_totp)
    ///
    /// Resets TOTP for an auth defense user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    pub async fn reset_auth_defense_user_totp(&self, user_id: i32) -> Result<GetUserTotpResponse> {
        self.post(&format!("/open/auth_defense/user/{}/reset_totp", user_id), &()).await
    }

    /// Unbind auth defense user (POST /open/auth_defense/user/{user_id}/unbind)
    ///
    /// Unbinds an auth defense user.
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID
    /// * `request` - Unbind request
    pub async fn unbind_auth_defense_user(&self, user_id: i32, request: &AuthDefenseUnbindUserRequest) -> Result<()> {
        self.post(&format!("/open/auth_defense/user/{}/unbind", user_id), request).await
    }

    /// List auth defense sources (GET /open/auth_defense/source)
    ///
    /// Retrieves a list of auth defense sources.
    ///
    /// # Returns
    ///
    /// Returns a list of auth defense sources
    pub async fn list_auth_defense_sources(&self) -> Result<AuthDefenseSourceListResponse> {
        self.get("/open/auth_defense/source").await
    }

    /// Get auth defense source (GET /open/auth_defense/source/{id})
    ///
    /// Retrieves detailed information about an auth defense source.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    ///
    /// # Returns
    ///
    /// Returns detailed source information
    pub async fn get_auth_defense_source(&self, id: i32) -> Result<AuthDefenseSource> {
        self.get(&format!("/open/auth_defense/source/{}", id)).await
    }

    /// Create auth defense source (POST /open/auth_defense/source)
    ///
    /// Creates a new auth defense source.
    ///
    /// # Arguments
    ///
    /// * `request` - Source creation request
    ///
    /// # Returns
    ///
    /// Returns the created source ID
    pub async fn create_auth_defense_source(&self, request: &AuthDefenseCreateSourceRequest) -> Result<CreateUserResponse> {
        self.post("/open/auth_defense/source", request).await
    }

    /// Update auth defense source (PUT /open/auth_defense/source/{id})
    ///
    /// Updates an auth defense source.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    /// * `request` - Source update request
    pub async fn update_auth_defense_source(&self, id: i32, request: &AuthDefenseUpdateSourceRequest) -> Result<()> {
        self.put(&format!("/open/auth_defense/source/{}", id), request).await
    }

    /// Delete auth defense source (DELETE /open/auth_defense/source/{id})
    ///
    /// Deletes an auth defense source.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    pub async fn delete_auth_defense_source(&self, id: i32) -> Result<()> {
        self.delete(&format!("/open/auth_defense/source/{}", id), &()).await
    }

    /// List auth defense source users (GET /open/auth_defense/source/{id}/user)
    ///
    /// Retrieves users from an auth defense source.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    ///
    /// # Returns
    ///
    /// Returns a list of source users
    pub async fn list_auth_defense_source_users(&self, id: i32) -> Result<AuthDefenseSourceUserListResponse> {
        self.get(&format!("/open/auth_defense/source/{}/user", id)).await
    }

    /// Put auth defense source user (PUT /open/auth_defense/source/{id}/user/{user_id})
    ///
    /// Updates an auth defense source user.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    /// * `user_id` - User ID
    /// * `request` - Update request
    pub async fn put_auth_defense_source_user(&self, id: i32, user_id: i32, request: &AuthDefensePutSourceUserRequest) -> Result<()> {
        self.put(&format!("/open/auth_defense/source/{}/user/{}", id, user_id), request).await
    }

    /// List auth defense user reviews (GET /open/auth_defense/user/review)
    ///
    /// Retrieves a list of auth defense user reviews.
    ///
    /// # Returns
    ///
    /// Returns a list of user reviews
    pub async fn list_auth_defense_user_reviews(&self) -> Result<AuthDefenseUserReviewListResponse> {
        self.get("/open/auth_defense/user/review").await
    }

    /// Get auth defense user review (GET /open/auth_defense/user/review/{review_id})
    ///
    /// Retrieves detailed information about an auth defense user review.
    ///
    /// # Arguments
    ///
    /// * `review_id` - Review ID
    ///
    /// # Returns
    ///
    /// Returns detailed review information
    pub async fn get_auth_defense_user_review(&self, review_id: i32) -> Result<AuthDefenseUserReviewListItem> {
        self.get(&format!("/open/auth_defense/user/review/{}", review_id)).await
    }

    /// Review auth defense user (POST /open/auth_defense/user/review/{review_id})
    ///
    /// Reviews an auth defense user.
    ///
    /// # Arguments
    ///
    /// * `review_id` - Review ID
    /// * `request` - Review request
    pub async fn review_auth_defense_user(&self, review_id: i32, request: &AuthDefenseUserReviewRequest) -> Result<()> {
        self.post(&format!("/open/auth_defense/user/review/{}", review_id), request).await
    }

    /// Merge auth defense users (POST /open/auth_defense/user/merge)
    ///
    /// Merges auth defense users.
    ///
    /// # Arguments
    ///
    /// * `request` - Merge request
    ///
    /// # Returns
    ///
    /// Returns merged user information
    pub async fn merge_auth_defense_users(&self, request: &AuthDefenseMergeUserRequest) -> Result<AuthDefenseMergeUserResponse> {
        self.post("/open/auth_defense/user/merge", request).await
    }

    /// Get auth defense logs (GET /open/auth_defense/source/{id}/user)
    ///
    /// Retrieves auth defense logs.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    ///
    /// # Returns
    ///
    /// Returns auth defense logs
    pub async fn get_auth_defense_logs(&self, id: i32) -> Result<AuthDefenseLogsResponse> {
        self.get(&format!("/open/auth_defense/source/{}/user", id)).await
    }

    /// Get auth defense logs v2 (GET /open/v2/records/auth_defense)
    ///
    /// Retrieves auth defense logs v2.
    ///
    /// # Returns
    ///
    /// Returns auth defense logs v2
    pub async fn get_auth_defense_logs_v2(&self) -> Result<GetAuthDefenseLogsV2Response> {
        self.get("/open/v2/records/auth_defense").await
    }

    /// Get auth defense source password (GET /open/auth_defense/source/password)
    ///
    /// Retrieves auth defense source password configuration.
    ///
    /// # Returns
    ///
    /// Returns password configuration
    pub async fn get_auth_defense_source_password(&self) -> Result<PasswordConfig> {
        self.get("/open/auth_defense/source/password").await
    }

    /// List auth defense groups (GET /open/auth_defense/source/{id}/user)
    ///
    /// Retrieves auth defense groups.
    ///
    /// # Arguments
    ///
    /// * `id` - Source ID
    ///
    /// # Returns
    ///
    /// Returns auth defense groups
    pub async fn list_auth_defense_groups(&self, id: i32) -> Result<AuthDefenseGroupListResponse> {
        self.get(&format!("/open/auth_defense/source/{}/user", id)).await
    }

    // ============================================================================
    // Challenge
    // ============================================================================

    /// Get challenge config (GET /open/challenge/config)
    ///
    /// Retrieves challenge configuration.
    ///
    /// # Returns
    ///
    /// Returns challenge configuration
    pub async fn get_challenge_config(&self) -> Result<ChallengeConfig> {
        self.get("/open/challenge/config").await
    }

    /// Get challenge logs (GET /api/challenge/logs)
    ///
    /// Retrieves challenge logs.
    ///
    /// # Returns
    ///
    /// Returns challenge logs
    pub async fn get_challenge_logs(&self) -> Result<ChallengeLogsResponse> {
        self.get("/api/challenge/logs").await
    }

    // ============================================================================
    // Cloud
    // ============================================================================

    /// Get cloud policies (GET /open/cloud/policies)
    ///
    /// Retrieves cloud policies.
    ///
    /// # Returns
    ///
    /// Returns cloud policies
    pub async fn get_cloud_policies(&self) -> Result<Vec<CloudPoliciesItem>> {
        self.get("/open/cloud/policies").await
    }

    /// Subscribe cloud policy (POST /open/cloud/policies/subscribe)
    ///
    /// Subscribes to a cloud policy.
    ///
    /// # Arguments
    ///
    /// * `request` - Subscribe request
    pub async fn subscribe_cloud_policy(&self, request: &CloudPoliciesSubscribeRequest) -> Result<()> {
        self.post("/open/cloud/policies/subscribe", request).await
    }

    // ============================================================================
    // Detector
    // ============================================================================

    /// Get detector (GET /open/detector)
    ///
    /// Retrieves detector configuration.
    ///
    /// # Returns
    ///
    /// Returns detector configuration
    pub async fn get_detector(&self) -> Result<DetectorMode> {
        self.get("/open/detector").await
    }

    /// Set detector (POST /open/detector)
    ///
    /// Sets detector configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Detector configuration request
    pub async fn set_detector(&self, request: &DetectorRequest) -> Result<()> {
        self.post("/open/detector", request).await
    }

    // ============================================================================
    // Intelligence
    // ============================================================================

    /// Get intelligence (GET /open/intelligence)
    ///
    /// Retrieves intelligence configuration.
    ///
    /// # Returns
    ///
    /// Returns intelligence configuration
    pub async fn get_intelligence(&self) -> Result<Intelligence> {
        self.get("/open/intelligence").await
    }

    /// Get intelligence IP library (GET /open/intelligence/ip_lib)
    ///
    /// Retrieves intelligence IP library.
    ///
    /// # Returns
    ///
    /// Returns IP library
    pub async fn get_intelligence_ip_lib(&self) -> Result<i32> {
        self.get("/open/intelligence/ip_lib").await
    }

    /// Put threat info (PUT /open/intelligence)
    ///
    /// Updates threat info configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Threat info request
    pub async fn put_threat_info(&self, request: &PutThreatInfoRequest) -> Result<()> {
        self.put("/open/intelligence", request).await
    }

    /// Put threat lib (PUT /open/intelligence)
    ///
    /// Updates threat library configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Threat lib request
    pub async fn put_threat_lib(&self, request: &PutThreatLibRequest) -> Result<()> {
        self.put("/open/intelligence", request).await
    }

    /// Set share behaviour (POST /open/share_behaviour)
    ///
    /// Sets share behaviour configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Share behaviour request
    pub async fn set_share_behaviour(&self, request: &SetShareBehaviourRequest) -> Result<()> {
        self.post("/open/share_behaviour", request).await
    }

    /// Set share fingerprint (POST /open/share_fingerprint)
    ///
    /// Sets share fingerprint configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Share fingerprint request
    pub async fn set_share_fingerprint(&self, request: &SetShareFingerprintRequest) -> Result<()> {
        self.post("/open/share_fingerprint", request).await
    }

    // ============================================================================
    // MCP
    // ============================================================================

    /// Get MCP (GET /mcp)
    ///
    /// Retrieves MCP configuration.
    ///
    /// # Returns
    ///
    /// Returns MCP configuration
    pub async fn get_mcp(&self) -> Result<McpGetResponse> {
        self.get("/mcp").await
    }

    /// Set MCP (POST /mcp)
    ///
    /// Sets MCP configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - MCP configuration request
    pub async fn set_mcp(&self, request: &McpSetRequest) -> Result<McpGetResponse> {
        self.post("/mcp", request).await
    }

    // ============================================================================
    // Security Posture
    // ============================================================================

    /// Get security posture statistics (GET /open/security_posture/statistics)
    ///
    /// Retrieves security posture statistics.
    ///
    /// # Returns
    ///
    /// Returns security posture statistics
    pub async fn get_security_posture_statistics(&self) -> Result<SecurityPostureStatisticsResponse> {
        self.get("/open/security_posture/statistics").await
    }

    /// Get security posture realtime (GET /open/security_posture/realtime)
    ///
    /// Retrieves real-time security posture.
    ///
    /// # Returns
    ///
    /// Returns real-time security posture
    pub async fn get_security_posture_realtime(&self) -> Result<SecurityPostureRealtimeResponse> {
        self.get("/open/security_posture/realtime").await
    }

    /// Get security posture trends (GET /open/security_posture/trends)
    ///
    /// Retrieves security posture trends.
    ///
    /// # Arguments
    ///
    /// * `query_type` - Query type
    ///
    /// # Returns
    ///
    /// Returns security posture trends
    pub async fn get_security_posture_trends(&self, query_type: Option<String>) -> Result<SecurityPostureTrendsResponse> {
        let mut params = HashMap::new();
        if let Some(query_type) = query_type {
            params.insert("query_type".to_string(), query_type);
        }
        if params.is_empty() {
            self.get("/open/security_posture/trends").await
        } else {
            self.get_with_query("/open/security_posture/trends", &params).await
        }
    }

    /// Set site security posture (POST /open/security_posture/site)
    ///
    /// Sets security posture for a site.
    ///
    /// # Arguments
    ///
    /// * `request` - Security posture request
    pub async fn set_site_security_posture(&self, request: &SetSiteSecurityPostureRequest) -> Result<()> {
        self.post("/open/security_posture/site", request).await
    }

    // ============================================================================
    // Skynet
    // ============================================================================

    /// Get global semantics (GET /open/skynet/rule)
    ///
    /// Retrieves global semantics configuration.
    ///
    /// # Returns
    ///
    /// Returns global semantics configuration
    pub async fn get_global_semantics(&self) -> Result<GetGlobalSemanticsResponse> {
        self.get("/open/skynet/rule").await
    }

    /// Get skynet rule (GET /commercial/skynet/rule)
    ///
    /// Retrieves skynet rule configuration.
    ///
    /// # Returns
    ///
    /// Returns skynet rule configuration
    pub async fn get_skynet_rule(&self) -> Result<GetSkynetRuleResponse> {
        self.get("/commercial/skynet/rule").await
    }

    /// Put skynet rule (PUT /commercial/skynet/rule)
    ///
    /// Updates skynet rule configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Skynet rule request
    pub async fn put_skynet_rule(&self, request: &PutSkynetRuleRequest) -> Result<()> {
        self.put("/commercial/skynet/rule", request).await
    }

    /// Put skynet rule switch (PUT /open/skynet/rule/switch)
    ///
    /// Toggles skynet rule.
    ///
    /// # Arguments
    ///
    /// * `request` - Skynet rule switch request
    pub async fn put_skynet_rule_switch(&self, request: &PutSkynetRuleSwitchRequest) -> Result<()> {
        self.put("/open/skynet/rule/switch", request).await
    }

    // ============================================================================
    // Settings
    // ============================================================================

    /// Get manager info (GET /ManagerInfo)
    ///
    /// Retrieves manager information.
    ///
    /// # Returns
    ///
    /// Returns manager information
    pub async fn get_manager_info(&self) -> Result<GetManagerInfoResponse> {
        self.get("/ManagerInfo").await
    }

    /// Update manager info (PUT /ManagerInfo)
    ///
    /// Updates manager information.
    ///
    /// # Arguments
    ///
    /// * `request` - Manager info update request
    pub async fn update_manager_info(&self, request: &PutManagerInfoRequest) -> Result<()> {
        self.put("/ManagerInfo", request).await
    }

    /// Get block pages (GET /commercial/block_page)
    ///
    /// Retrieves block pages.
    ///
    /// # Arguments
    ///
    /// * `r#type` - Block page type
    ///
    /// # Returns
    ///
    /// Returns block pages
    pub async fn get_block_pages(&self, r#type: Option<String>) -> Result<GetSpecialPageResponse> {
        let mut params = HashMap::new();
        if let Some(page_type) = r#type {
            params.insert("type".to_string(), page_type);
        }
        if params.is_empty() {
            self.get("/commercial/block_page").await
        } else {
            self.get_with_query("/commercial/block_page", &params).await
        }
    }

    /// Update block pages (PUT /commercial/block_page)
    ///
    /// Updates block pages.
    ///
    /// # Arguments
    ///
    /// * `request` - Block page update request
    pub async fn update_block_pages(&self, request: &PutSpecialPageRequest) -> Result<()> {
        self.put("/commercial/block_page", request).await
    }

    /// List block pages (GET /commercial/block_page_list)
    ///
    /// Lists block pages.
    ///
    /// # Returns
    ///
    /// Returns list of block pages
    pub async fn list_block_pages(&self) -> Result<ListSpecialPageResponse> {
        self.get("/commercial/block_page_list").await
    }

    /// Get frontend style (GET /business/frontend_style)
    ///
    /// Retrieves frontend style configuration.
    ///
    /// # Returns
    ///
    /// Returns frontend style configuration
    pub async fn get_frontend_style(&self) -> Result<GetFrontendStyleResponse> {
        self.get("/business/frontend_style").await
    }

    /// Set frontend style (PUT /business/frontend_style)
    ///
    /// Sets frontend style configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Frontend style request
    pub async fn set_frontend_style(&self, request: &PutFrontendStyleRequest) -> Result<()> {
        self.put("/business/frontend_style", request).await
    }

    /// Get dashboard token (POST /api/commercial/dashboard/token)
    ///
    /// Retrieves dashboard token.
    ///
    /// # Arguments
    ///
    /// * `request` - Dashboard token request
    ///
    /// # Returns
    ///
    /// Returns dashboard token
    pub async fn get_dashboard_token(&self, request: &DashboardTokenRequest) -> Result<DashboardTokenResponse> {
        self.post("/api/commercial/dashboard/token", request).await
    }

    /// Site group switch (PUT /commercial/site/group/switch)
    ///
    /// Switches site group settings.
    ///
    /// # Arguments
    ///
    /// * `request` - Group switch request
    pub async fn site_group_switch(&self, request: &GroupSwitchRequest) -> Result<()> {
        self.put("/commercial/site/group/switch", request).await
    }

    /// Get dashboard intercepts (GET /open/dashboard/intercepts)
    ///
    /// Retrieves dashboard intercepts.
    ///
    /// # Returns
    ///
    /// Returns dashboard intercepts
    pub async fn get_dashboard_intercepts(&self) -> Result<DashboardTrendResponse> {
        self.get("/open/dashboard/intercepts").await
    }

    /// Export attack logs (GET /commercial/record/export)
    ///
    /// Exports attack logs.
    ///
    /// # Returns
    ///
    /// Returns exported log file
    pub async fn export_attack_logs(&self, params: &HashMap<String, String>) -> Result<String> {
        self.get_with_query("/commercial/record/export", params).await
    }

    /// Get waiting room logs (GET /open/records/waiting)
    ///
    /// Retrieves waiting room logs.
    ///
    /// # Returns
    ///
    /// Returns waiting room logs
    pub async fn get_waiting_room_logs(&self) -> Result<WaitingRoomLogsResponse> {
        self.get("/open/records/waiting").await
    }
}