use crate::client::Client;
use crate::error::Result;
use crate::models::*;
use std::collections::HashMap;

impl Client {
    /// Get ACL logs (GET /commercial/record/export)
    ///
    /// Retrieves ACL logs.
    ///
    /// # Returns
    ///
    /// Returns ACL logs
    pub async fn get_acl_logs(&self, request: &ACLSearchRequest) -> Result<ACLLogsResponse> {
        let mut params = HashMap::new();
        if let Some(begin) = request.begin {
            params.insert("begin".to_string(), begin.to_string());
        }
        if let Some(end) = request.end {
            params.insert("end".to_string(), end.to_string());
        }
        if let Some(ref ip) = request.ip {
            params.insert("ip".to_string(), ip.clone());
        }
        if let Some(site) = request.site {
            params.insert("site".to_string(), site.to_string());
        }
        self.get_with_query("/commercial/record/export", &params).await
    }

    /// ACL relieve (PUT /open/acl/relieve)
    ///
    /// Relieves ACL blocking.
    ///
    /// # Arguments
    ///
    /// * `request` - ACL relieve request
    pub async fn acl_relieve(&self, request: &ACLRelieveRequest) -> Result<()> {
        self.put("/open/acl/relieve", request).await
    }
}