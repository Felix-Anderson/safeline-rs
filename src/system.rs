use crate::client::Client;
use crate::error::Result;
use crate::models::*;
use std::collections::HashMap;

impl Client {
    /// Get system about (GET /open/system)
    ///
    /// Retrieves system information.
    ///
    /// # Returns
    ///
    /// Returns system information
    pub async fn get_system_about(&self) -> Result<SystemAboutResponse> {
        self.get("/open/system").await
    }

    /// Get system edition (GET /open/system/edition)
    ///
    /// Retrieves system edition information.
    ///
    /// # Returns
    ///
    /// Returns system edition information
    pub async fn get_system_edition(&self) -> Result<SystemEditionResponse> {
        self.get("/open/system/edition").await
    }

    /// Get system arch (GET /open/system/arch)
    ///
    /// Retrieves system architecture information.
    ///
    /// # Returns
    ///
    /// Returns system architecture information
    pub async fn get_system_arch(&self) -> Result<String> {
        self.get("/open/system/arch").await
    }

    /// Get system authorize (GET /open/system/authorize)
    ///
    /// Retrieves system authorization information.
    ///
    /// # Returns
    ///
    /// Returns system authorization information
    pub async fn get_system_authorize(&self) -> Result<SystemAboutResponse> {
        self.get("/open/system/authorize").await
    }

    /// Get system key (GET /open/system/key)
    ///
    /// Retrieves system key information.
    ///
    /// # Returns
    ///
    /// Returns system key information
    pub async fn get_system_key(&self) -> Result<String> {
        self.get("/open/system/key").await
    }

    /// Get system login method (GET /business/system/login_method)
    ///
    /// Retrieves system login method configuration.
    ///
    /// # Returns
    ///
    /// Returns login method configuration
    pub async fn get_system_login_method(&self) -> Result<LoginMethod> {
        self.get("/business/system/login_method").await
    }

    /// Set system login method (PUT /business/system/login_method)
    ///
    /// Sets system login method configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Login method configuration request
    pub async fn set_system_login_method(&self, request: &LoginMethod) -> Result<()> {
        self.put("/business/system/login_method", request).await
    }

    /// Get system login type (GET /open/system/login_type)
    ///
    /// Retrieves system login type.
    ///
    /// # Returns
    ///
    /// Returns system login type
    pub async fn get_system_login_type(&self) -> Result<i32> {
        self.get("/open/system/login_type").await
    }

    /// Get system network proxy (GET /open/system/network_proxy)
    ///
    /// Retrieves system network proxy configuration.
    ///
    /// # Returns
    ///
    /// Returns network proxy configuration
    pub async fn get_system_network_proxy(&self) -> Result<NetworkProxy> {
        self.get("/open/system/network_proxy").await
    }

    /// Get system protocol (GET /open/system/protocol)
    ///
    /// Retrieves system protocol information.
    ///
    /// # Returns
    ///
    /// Returns system protocol information
    pub async fn get_system_protocol(&self) -> Result<String> {
        self.get("/open/system/protocol").await
    }

    /// Update system (PUT /open/system)
    ///
    /// Updates system configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - System update request
    pub async fn update_system(&self, request: &SystemUpdateRequest) -> Result<()> {
        self.put("/open/system", request).await
    }

    /// Get global log clean config (GET /open/global/log_clean)
    ///
    /// Retrieves global log cleanup configuration.
    ///
    /// # Returns
    ///
    /// Returns log cleanup configuration
    pub async fn get_global_log_clean(&self) -> Result<LogMaxDayConfig> {
        self.get("/open/global/log_clean").await
    }

    /// Set global log clean config (PUT /open/global/log_clean)
    ///
    /// Sets global log cleanup configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Log cleanup configuration request
    pub async fn set_global_log_clean(&self, request: &LogMaxDayConfig) -> Result<()> {
        self.put("/open/global/log_clean", request).await
    }

    /// Get global mode (GET /open/global/mode)
    ///
    /// Retrieves global mode configuration.
    ///
    /// # Returns
    ///
    /// Returns global mode configuration
    pub async fn get_global_mode(&self) -> Result<DetectorMode> {
        self.get("/open/global/mode").await
    }

    /// Set global mode (PUT /open/global/mode)
    ///
    /// Sets global mode configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Mode configuration request
    pub async fn set_global_mode(&self, request: &DetectorRequest) -> Result<()> {
        self.put("/open/global/mode", request).await
    }

    /// Get global proxy (GET /open/global/proxy)
    ///
    /// Retrieves global proxy configuration.
    ///
    /// # Returns
    ///
    /// Returns global proxy configuration
    pub async fn get_global_proxy(&self) -> Result<GlobalProxyConfig> {
        self.get("/open/global/proxy").await
    }

    /// Set global proxy (PUT /open/global/proxy)
    ///
    /// Sets global proxy configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Proxy configuration request
    pub async fn set_global_proxy(&self, request: &GlobalProxyConfig) -> Result<()> {
        self.put("/open/global/proxy", request).await
    }

    /// Get account config (GET /business/account)
    ///
    /// Retrieves account configuration.
    ///
    /// # Returns
    ///
    /// Returns account configuration
    pub async fn get_account_config(&self) -> Result<AccountConfig> {
        self.get("/business/account").await
    }

    /// Set account config (POST /business/account)
    ///
    /// Sets account configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Account configuration request
    pub async fn set_account_config(&self, request: &AccountConfig) -> Result<()> {
        self.post("/business/account", request).await
    }

    /// Get syslog config (GET /commercial/syslog)
    ///
    /// Retrieves syslog configuration.
    ///
    /// # Returns
    ///
    /// Returns syslog configuration
    pub async fn get_syslog_config(&self) -> Result<ReceiverConfig> {
        self.get("/commercial/syslog").await
    }

    /// Set syslog config (PUT /commercial/syslog)
    ///
    /// Sets syslog configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Syslog configuration request
    pub async fn set_syslog_config(&self, request: &ReceiverConfig) -> Result<()> {
        self.put("/commercial/syslog", request).await
    }

    /// Test syslog (POST /commercial/syslog/test)
    ///
    /// Tests syslog configuration.
    ///
    /// # Arguments
    ///
    /// * `request` - Syslog configuration request
    pub async fn test_syslog(&self, request: &ReceiverConfig) -> Result<()> {
        self.post("/commercial/syslog/test", request).await
    }

    /// Get downgrade version (GET /open/Commercial/downgrade)
    ///
    /// Retrieves downgrade version status.
    ///
    /// # Returns
    ///
    /// Returns downgrade version status
    pub async fn get_downgrade_version(&self) -> Result<GetDowngradeVersionResponse> {
        self.get("/open/Commercial/downgrade").await
    }

    /// Downgrade (PUT /open/Commercial/downgrade)
    ///
    /// Performs system downgrade.
    pub async fn downgrade(&self) -> Result<()> {
        self.put("/open/Commercial/downgrade", &()).await
    }

    /// Get code apply info (GET /open/system/code_apply_info)
    ///
    /// Retrieves code application information.
    ///
    /// # Returns
    ///
    /// Returns code application information
    pub async fn get_code_apply_info(&self) -> Result<CodeApplyInfoResponse> {
        self.get("/open/system/code_apply_info").await
    }

    /// Apply code (POST /open/system/code_apply)
    ///
    /// Applies a code.
    ///
    /// # Arguments
    ///
    /// * `request` - Code apply request
    pub async fn apply_code(&self, request: &CodeApplyRequest) -> Result<CodeApplyInfoResponse> {
        self.post("/open/system/code_apply", request).await
    }

    /// Get audit logs (GET /business/audit_log)
    ///
    /// Retrieves audit logs.
    ///
    /// # Arguments
    ///
    /// * `page` - Page number
    /// * `page_size` - Page size
    ///
    /// # Returns
    ///
    /// Returns audit logs
    pub async fn get_audit_logs(&self, page: i32, page_size: i32) -> Result<AuditLogListResponse> {
        let mut params = HashMap::new();
        params.insert("page".to_string(), page.to_string());
        params.insert("page_size".to_string(), page_size.to_string());
        self.get_with_query("/business/audit_log", &params).await
    }

    /// List users (GET /open/users)
    ///
    /// Retrieves a list of users.
    ///
    /// # Returns
    ///
    /// Returns a list of users
    pub async fn list_users(&self) -> Result<UserListResponse> {
        self.get("/open/users").await
    }

    /// Create user (POST /open/users)
    ///
    /// Creates a new user.
    ///
    /// # Arguments
    ///
    /// * `request` - User creation request
    ///
    /// # Returns
    ///
    /// Returns the created user ID
    pub async fn create_user(&self, request: &CreateUserRequest) -> Result<CreateUserResponse> {
        self.post("/open/users", request).await
    }

    /// Update user (PUT /open/users)
    ///
    /// Updates an existing user.
    ///
    /// # Arguments
    ///
    /// * `request` - User update request
    pub async fn update_user(&self, request: &UpdateUserRequest) -> Result<()> {
        self.put("/open/users", request).await
    }

    /// Delete user (DELETE /open/users)
    ///
    /// Deletes a user.
    ///
    /// # Arguments
    ///
    /// * `request` - Delete user request
    pub async fn delete_user(&self, request: &DeleteUserRequest) -> Result<()> {
        self.delete("/open/users", request).await
    }

    /// Unlock user (POST /business/users/{id}/unlock)
    ///
    /// Unlocks a user account.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    pub async fn unlock_user(&self, id: i32) -> Result<()> {
        self.post(&format!("/business/users/{}/unlock", id), &UnlockUserRequest {}).await
    }

    /// Get user TOTP (GET /open/users/{id}/totp)
    ///
    /// Retrieves user TOTP information.
    ///
    /// # Arguments
    ///
    /// * `id` - User ID
    ///
    /// # Returns
    ///
    /// Returns user TOTP information
    pub async fn get_user_totp(&self, id: i32) -> Result<GetUserTotpResponse> {
        self.get(&format!("/open/users/{}/totp", id)).await
    }
}