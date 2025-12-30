use crate::client::Client;
use crate::error::Result;
use crate::models::*;
use std::collections::HashMap;

impl Client {
    /// Create IP Group (POST /open/ipgroup)
    /// 
    /// Creates a new IP group with the specified configuration.
    /// 
    /// # Arguments
    /// 
    /// * `request` - IP group creation request
    /// 
    /// # Returns
    /// 
    /// Returns the ID of the created IP group
    pub async fn ip_group_create(&self, request: &IPGroupCreateRequest) -> Result<IPGroupCreateResponse> {
        self.post("/open/ipgroup", request).await
    }

    /// List IP Groups (GET /open/ipgroup)
    /// 
    /// Retrieves a list of IP groups.
    /// 
    /// # Arguments
    /// 
    /// * `request` - List request with optional top parameter
    /// 
    /// # Returns
    /// 
    /// Returns a list of IP groups with total count
    pub async fn ip_group_list(&self, request: &IPGroupListRequest) -> Result<IPGroupListResponse> {
        let mut params = HashMap::new();
        if let Some(top) = request.top {
            params.insert("top".to_string(), top.to_string());
        }
        
        self.get_with_query("/open/ipgroup", &params).await
    }

    /// Update IP Group (PUT /open/ipgroup)
    /// 
    /// Updates an existing IP group.
    /// 
    /// # Arguments
    /// 
    /// * `request` - IP group update request
    pub async fn ip_group_update(&self, request: &IPGroupUpdateRequest) -> Result<IPGroupUpdateResponse> {
        self.put("/open/ipgroup", request).await
    }

    /// Delete IP Groups (DELETE /open/ipgroup)
    /// 
    /// Deletes one or more IP groups.
    /// 
    /// # Arguments
    /// 
    /// * `request` - Delete request with list of IP group IDs
    pub async fn ip_group_delete(&self, request: &IPGroupDeleteRequest) -> Result<IPGroupDeleteResponse> {
        self.delete("/open/ipgroup", request).await
    }

    /// Get IP Group Detail (GET /open/ipgroup/detail)
    /// 
    /// Retrieves detailed information about a specific IP group.
    /// 
    /// # Arguments
    /// 
    /// * `request` - Detail request with IP group ID
    /// 
    /// # Returns
    /// 
    /// Returns detailed IP group information
    pub async fn ip_group_detail(&self, request: &IPGroupDetailRequest) -> Result<IPGroupDetailResponse> {
        let mut params = HashMap::new();
        params.insert("id".to_string(), request.id.to_string());
        
        self.get_with_query("/open/ipgroup/detail", &params).await
    }

    /// Add IPs to IP Groups (POST /open/ipgroup/append)
    /// 
    /// Adds IP addresses to one or more IP groups.
    /// 
    /// # Arguments
    /// 
    /// * `request` - Append request with IP group IDs and IP addresses
    pub async fn ip_group_append(&self, request: &IPGroupAppendRequest) -> Result<IPGroupAppendResponse> {
        self.post("/open/ipgroup/append", request).await
    }

    /// Get Search Engine Spider Group ID (GET /open/ipgroup/crawler)
    /// 
    /// Retrieves the ID of the search engine spider IP group.
    /// 
    /// # Returns
    /// 
    /// Returns the search engine spider group ID
    pub async fn ip_group_crawler(&self) -> Result<IPGroupCrawlerResponse> {
        self.get("/open/ipgroup/crawler").await
    }

    /// Update Search Engine Spider IPs (POST /open/ipgroup/crawler)
    /// 
    /// Updates the search engine spider IP group with latest spider IPs.
    pub async fn ip_group_crawler_update(&self) -> Result<IPGroupCrawlerUpdateResponse> {
        let request = IPGroupCrawlerUpdateRequest {};
        self.post("/open/ipgroup/crawler", &request).await
    }

    /// Get IPs by Link (GET /open/ipgroup/link)
    /// 
    /// Retrieves IP addresses associated with a specific link.
    /// 
    /// # Arguments
    /// 
    /// * `request` - Link request with URL
    /// 
    /// # Returns
    /// 
    /// Returns IP addresses found through the link
    pub async fn ip_group_link(&self, request: &IPGroupLinkRequest) -> Result<IPGroupLinkResponse> {
        let mut params = HashMap::new();
        params.insert("href".to_string(), request.href.clone());
        
        self.get_with_query("/open/ipgroup/link", &params).await
    }

    /// Create IP Group by Link (POST /open/ipgroup/link)
    /// 
    /// Creates a new IP group by extracting IPs from a URL.
    /// 
    /// # Arguments
    /// 
    /// * `request` - Create by link request with comment and URL
    pub async fn ip_group_create_by_link(&self, request: &IPGroupCreateByLinkRequest) -> Result<IPGroupCreateByLinkResponse> {
        let mut params = HashMap::new();
        params.insert("comment".to_string(), request.comment.clone());
        params.insert("url".to_string(), request.url.clone());
        
        self.get_with_query("/open/ipgroup/link", &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ip_group_operations() {
        let _client = Client::new("https://test.example.com:9443", "test-token");

        // Test create request
        let create_req = IPGroupCreateRequest {
            comment: Some("Test IP Group".to_string()),
            ips: vec!["192.168.1.1".to_string(), "10.0.0.1".to_string()],
            reference: Some("test-reference".to_string()),
        };

        // Note: These are just compilation tests since we don't have a real API endpoint
        let _create_req_serialized = serde_json::to_string(&create_req).unwrap();

        // Test list request
        let list_req = IPGroupListRequest { top: Some(10) };
        let _list_req_serialized = serde_json::to_string(&list_req).unwrap();

        // Test update request
        let update_req = IPGroupUpdateRequest {
            id: 1,
            builtin: Some(false),
            comment: Some("Updated IP Group".to_string()),
            ips: Some(vec!["192.168.1.1".to_string()]),
            reference: Some("updated-reference".to_string()),
        };
        let _update_req_serialized = serde_json::to_string(&update_req).unwrap();

        // Test delete request
        let delete_req = IPGroupDeleteRequest { ids: vec![1, 2, 3] };
        let _delete_req_serialized = serde_json::to_string(&delete_req).unwrap();

        // Test append request
        let append_req = IPGroupAppendRequest {
            ip_group_ids: vec![1],
            ips: vec!["172.16.0.1".to_string()],
        };
        let _append_req_serialized = serde_json::to_string(&append_req).unwrap();

        // Test detail request
        let detail_req = IPGroupDetailRequest { id: 1 };
        let _detail_req_serialized = serde_json::to_string(&detail_req).unwrap();

        // Test link request
        let link_req = IPGroupLinkRequest {
            href: "https://example.com".to_string(),
        };
        let _link_req_serialized = serde_json::to_string(&link_req).unwrap();

        // Test create by link request
        let create_by_link_req = IPGroupCreateByLinkRequest {
            comment: "Test from link".to_string(),
            url: "https://example.com".to_string(),
        };
        let _create_by_link_req_serialized = serde_json::to_string(&create_by_link_req).unwrap();
    }
}