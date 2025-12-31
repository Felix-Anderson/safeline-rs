use crate::client::Client;
use crate::error::Result;
use crate::models::*;
use std::collections::HashMap;

impl Client {
    /// List policies (GET /open/policy)
    ///
    /// Retrieves a list of policies.
    ///
    /// # Returns
    ///
    /// Returns a list of policies
    pub async fn list_policies(&self) -> Result<PolicyListResponse> {
        self.get("/open/policy").await
    }

    /// Create policy (POST /open/policy)
    ///
    /// Creates a new policy.
    ///
    /// # Arguments
    ///
    /// * `request` - Policy creation request
    ///
    /// # Returns
    ///
    /// Returns the created policy ID
    pub async fn create_policy(&self, request: &PolicyRequest) -> Result<CreateUserResponse> {
        self.post("/open/policy", request).await
    }

    /// Get policy detail (GET /open/policy/detail)
    ///
    /// Retrieves detailed information about a specific policy.
    ///
    /// # Arguments
    ///
    /// * `id` - Policy ID
    ///
    /// # Returns
    ///
    /// Returns detailed policy information
    pub async fn get_policy_detail(&self, id: i32) -> Result<PolicyDetailResponse> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), id.to_string());
        self.get_with_query("/open/policy/detail", &params).await
    }

    /// Update policy (PUT /open/policy)
    ///
    /// Updates an existing policy.
    ///
    /// # Arguments
    ///
    /// * `request` - Policy update request
    pub async fn update_policy(&self, request: &PolicyRequest) -> Result<()> {
        self.put("/open/policy", request).await
    }

    /// Delete policy (DELETE /open/policy)
    ///
    /// Deletes a policy.
    ///
    /// # Arguments
    ///
    /// * `request` - Delete policy request
    pub async fn delete_policy(&self, request: &DeletePolicyRequest) -> Result<()> {
        self.delete("/open/policy", request).await
    }

    /// Policy switch (PUT /open/policy/switch)
    ///
    /// Toggles a policy on or off.
    ///
    /// # Arguments
    ///
    /// * `request` - Policy switch request
    pub async fn policy_switch(&self, request: &PolicySwitchRequest) -> Result<()> {
        self.put("/open/policy/switch", request).await
    }
}