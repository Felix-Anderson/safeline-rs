use crate::client::Client;
use crate::error::Result;
use crate::models::*;

impl Client {
    // ============================================================================
    // Website Management
    // ============================================================================

    /// List websites (GET /open/site)
    ///
    /// Retrieves a list of websites.
    ///
    /// # Returns
    ///
    /// Returns a list of websites
    pub async fn list_websites(&self) -> Result<WebsiteListResponse> {
        self.get("/open/site").await
    }

    /// Create website (POST /open/site)
    ///
    /// Creates a new website.
    ///
    /// # Arguments
    ///
    /// * `request` - Website creation request
    ///
    /// # Returns
    ///
    /// Returns the created website ID
    pub async fn create_website(&self, request: &WebsiteRequest) -> Result<CreateUserResponse> {
        self.post("/open/site", request).await
    }

    /// Get website detail (GET /open/site/{id})
    ///
    /// Retrieves detailed information about a specific website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns detailed website information
    pub async fn get_website_detail(&self, id: i32) -> Result<WebsiteDetailResponse> {
        self.get(&format!("/open/site/{}", id)).await
    }

    /// Update website (PUT /open/site/{id})
    ///
    /// Updates an existing website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Website update request
    pub async fn update_website(&self, id: i32, request: &WebsiteRequest) -> Result<()> {
        self.put(&format!("/open/site/{}", id), request).await
    }

    /// Delete websites (DELETE /open/site)
    ///
    /// Deletes one or more websites.
    ///
    /// # Arguments
    ///
    /// * `request` - Delete request with list of website IDs
    pub async fn delete_websites(&self, request: &DeleteWebsiteRequest) -> Result<()> {
        self.delete("/open/site", request).await
    }

    /// Put website basic info (PUT /open/site/{id}/basic_info)
    ///
    /// Updates basic information of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Basic info update request
    pub async fn put_website_basic_info(&self, id: i32, request: &PutWebsiteBasicInfoRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/basic_info", id), request).await
    }

    /// Put website defense (PUT /open/site/{id}/defense)
    ///
    /// Updates defense settings of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Defense update request
    pub async fn put_website_defense(&self, id: i32, request: &PutWebsiteDefenseRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/defense", id), request).await
    }

    /// Put website mode (PUT /open/site/mode)
    ///
    /// Updates the mode of websites.
    ///
    /// # Arguments
    ///
    /// * `request` - Mode update request
    pub async fn put_website_mode(&self, request: &PutWebsiteModeRequest) -> Result<()> {
        self.put("/open/site/mode", request).await
    }

    /// Website chaos (POST /open/site/{id}/chaos)
    ///
    /// Updates chaos settings of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Chaos update request
    pub async fn website_chaos(&self, id: i32, request: &WebsiteChaosRequest) -> Result<()> {
        self.post(&format!("/open/site/{}/chaos", id), request).await
    }

    /// Website challenge (POST /open/site/challenge)
    ///
    /// Updates challenge settings of a website.
    ///
    /// # Arguments
    ///
    /// * `request` - Challenge update request
    pub async fn website_challenge(&self, request: &WebsiteChallengeRequest) -> Result<()> {
        self.post("/open/site/challenge", request).await
    }

    /// Set website waiting room (POST /open/site/{id}/waiting_room)
    ///
    /// Updates waiting room settings of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Waiting room update request
    pub async fn set_website_waiting_room(&self, id: i32, request: &SetWebsiteWaitingRoomRequest) -> Result<()> {
        self.post(&format!("/open/site/{}/waiting_room", id), request).await
    }

    /// Website health check (POST /open/site/healthcheck)
    ///
    /// Performs a health check on a website.
    ///
    /// # Arguments
    ///
    /// * `request` - Health check request
    pub async fn website_health_check(&self, request: &WebsiteHealthCheckRequest) -> Result<()> {
        self.post("/open/site/healthcheck", request).await
    }

    // ============================================================================
    // Website Group Management
    // ============================================================================

    /// List website groups (GET /open/site/group)
    ///
    /// Retrieves a list of website groups.
    ///
    /// # Returns
    ///
    /// Returns a list of website groups
    pub async fn list_website_groups(&self) -> Result<WebsiteGroupResponse> {
        self.get("/open/site/group").await
    }

    /// Create website group (POST /open/site/group)
    ///
    /// Creates a new website group.
    ///
    /// # Arguments
    ///
    /// * `request` - Group creation request
    ///
    /// # Returns
    ///
    /// Returns the created group ID
    pub async fn create_website_group(&self, request: &CreateGroupRequest) -> Result<CreateUserResponse> {
        self.post("/open/site/group", request).await
    }

    /// Update website group (PUT /open/site/group/{id})
    ///
    /// Updates an existing website group.
    ///
    /// # Arguments
    ///
    /// * `id` - Group ID
    /// * `request` - Group update request
    pub async fn update_website_group(&self, id: i32, request: &UpdateGroupRequest) -> Result<()> {
        self.put(&format!("/open/site/group/{}", id), request).await
    }

    /// Delete website group (DELETE /open/site/group/{id})
    ///
    /// Deletes a website group.
    ///
    /// # Arguments
    ///
    /// * `id` - Group ID
    pub async fn delete_website_group(&self, id: i32) -> Result<()> {
        self.delete(&format!("/open/site/group/{}", id), &()).await
    }

    /// Sort website groups (PUT /open/site/group/{id}/sort)
    ///
    /// Sorts website groups.
    ///
    /// # Arguments
    ///
    /// * `id` - Group ID
    /// * `request` - Sort request
    pub async fn sort_website_groups(&self, id: i32, request: &SortGroupRequest) -> Result<()> {
        self.put(&format!("/open/site/group/{}/sort", id), request).await
    }

    /// Website group switch (PUT /open/site/group/switch)
    ///
    /// Toggles website group settings.
    ///
    /// # Arguments
    ///
    /// * `request` - Switch request
    pub async fn website_group_switch(&self, request: &GroupSwitchRequest) -> Result<()> {
        self.put("/open/site/group/switch", request).await
    }

    /// Update site group (PUT /open/site/{id}/group)
    ///
    /// Updates the group of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Update group request
    pub async fn update_site_group(&self, id: i32, request: &UpdateSiteGroupRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/group", id), request).await
    }

    /// Sort websites (PUT /open/site/{id}/sort)
    ///
    /// Sorts websites.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Sort request
    pub async fn sort_websites(&self, id: i32, request: &SortWebsiteRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/sort", id), request).await
    }

    // ============================================================================
    // Website Configuration
    // ============================================================================

    /// Get nginx config (GET /open/site/{id}/nginx_config)
    ///
    /// Retrieves the nginx configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns the nginx configuration
    pub async fn get_nginx_config(&self, id: i32) -> Result<NginxConfigGetResponse> {
        self.get(&format!("/open/site/{}/nginx_config", id)).await
    }

    /// Update nginx config (PUT /open/site/{id}/nginx_config)
    ///
    /// Updates the nginx configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Nginx config update request
    pub async fn update_nginx_config(&self, id: i32, request: &NginxConfigUpdateRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/nginx_config", id), request).await
    }

    /// Get website resources (GET /open/site/{id}/resources)
    ///
    /// Retrieves resources of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns website resources
    pub async fn get_website_resources(&self, id: i32) -> Result<WebsiteResource> {
        self.get(&format!("/open/site/{}/resources", id)).await
    }

    /// Update excludes (PUT /open/site/{id}/excludes)
    ///
    /// Updates the excludes configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Excludes update request
    pub async fn update_excludes(&self, id: i32, request: &UpdateExcludesRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/excludes", id), request).await
    }

    /// Remove resources (DELETE /open/site/{id}/resources)
    ///
    /// Removes resources from a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Remove request
    pub async fn remove_resources(&self, id: i32, request: &RemoveResourceRequest) -> Result<()> {
        self.delete(&format!("/open/site/{}/resources", id), request).await
    }

    /// Get website log (GET /open/site/{id}/log)
    ///
    /// Retrieves log files of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns log files
    pub async fn get_website_log(&self, id: i32) -> Result<Vec<ListLogItem>> {
        self.get(&format!("/open/site/{}/log", id)).await
    }

    /// Get website log detail (GET /open/site/{id}/log/detail)
    ///
    /// Retrieves detailed log information of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns detailed log information
    pub async fn get_website_log_detail(&self, id: i32) -> Result<String> {
        self.get(&format!("/open/site/{}/log/detail", id)).await
    }

    /// Download website log (GET /open/site/{id}/log/download)
    ///
    /// Downloads log files of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns log file content
    pub async fn download_website_log(&self, id: i32) -> Result<String> {
        self.get(&format!("/open/site/{}/log/download", id)).await
    }

    /// Get website log limit (GET /open/site/{id}/log/limit)
    ///
    /// Retrieves log limit configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns log limit configuration
    pub async fn get_website_log_limit(&self, id: i32) -> Result<i32> {
        self.get(&format!("/open/site/{}/log/limit", id)).await
    }

    /// Get website proxy (GET /open/site/{id}/proxy)
    ///
    /// Retrieves proxy configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns proxy configuration
    pub async fn get_website_proxy(&self, id: i32) -> Result<ProxyConfig> {
        self.get(&format!("/open/site/{}/proxy", id)).await
    }

    /// Get website semantics (GET /open/site/{id}/semantics)
    ///
    /// Retrieves semantics configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns semantics configuration
    pub async fn get_website_semantics(&self, id: i32) -> Result<SkynetModule> {
        self.get(&format!("/open/site/{}/semantics", id)).await
    }

    /// Get website static (GET /open/site/{id}/static)
    ///
    /// Retrieves static files of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns static files
    pub async fn get_website_static(&self, id: i32) -> Result<Vec<FileInfo>> {
        self.get(&format!("/open/site/{}/static", id)).await
    }

    /// Add static (POST /open/site/{id}/static)
    ///
    /// Adds static files to a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Add static request
    pub async fn add_static(&self, id: i32, request: &AddStaticRequest) -> Result<()> {
        self.post(&format!("/open/site/{}/static", id), request).await
    }

    /// Delete static (DELETE /open/site/{id}/static)
    ///
    /// Deletes static files from a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Delete static request
    pub async fn delete_static(&self, id: i32, request: &DeleteStaticRequest) -> Result<()> {
        self.delete(&format!("/open/site/{}/static", id), request).await
    }

    /// Rename static (PUT /open/site/{id}/static)
    ///
    /// Renames static files of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - Rename static request
    pub async fn rename_static(&self, id: i32, request: &RenameStaticRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/static", id), request).await
    }

    /// Get website ACL (GET /open/site/{id}/acl)
    ///
    /// Retrieves ACL configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns ACL configuration
    pub async fn get_website_acl(&self, id: i32) -> Result<Vec<ACLConfig>> {
        self.get(&format!("/open/site/{}/acl", id)).await
    }

    /// Set website ACL (PUT /open/site/{id}/acl)
    ///
    /// Sets ACL configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `request` - ACL configuration request
    pub async fn set_website_acl(&self, id: i32, request: &SetSiteACLRequest) -> Result<()> {
        self.put(&format!("/open/site/{}/acl", id), request).await
    }

    /// Delete website ACL rule (DELETE /open/site/{id}/acl/{rule_id})
    ///
    /// Deletes an ACL rule from a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    /// * `rule_id` - ACL rule ID
    pub async fn delete_website_acl_rule(&self, id: i32, rule_id: i32) -> Result<()> {
        self.delete(&format!("/open/site/{}/acl/{}", id, rule_id), &()).await
    }

    /// Get website waiting room (GET /open/site/:id/waiting)
    ///
    /// Retrieves waiting room configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns waiting room configuration
    pub async fn get_website_waiting_room(&self, id: i32) -> Result<WaitingRoom> {
        self.get(&format!("/open/site/{}/waiting", id)).await
    }

    /// Get website chaos (GET /open/site/{id}/chaos)
    ///
    /// Retrieves chaos configuration of a website.
    ///
    /// # Arguments
    ///
    /// * `id` - Website ID
    ///
    /// # Returns
    ///
    /// Returns chaos configuration
    pub async fn get_website_chaos(&self, id: i32) -> Result<Chaos> {
        self.get(&format!("/open/site/{}/chaos", id)).await
    }

    /// Get site mode (GET /open/site/mode)
    ///
    /// Retrieves the site mode.
    ///
    /// # Returns
    ///
    /// Returns site mode
    pub async fn get_site_mode(&self) -> Result<SiteMode> {
        self.get("/open/site/mode").await
    }
}