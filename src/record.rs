use crate::client::Client;
use crate::error::Result;
use crate::models::*;

impl Client {
    /// List records (GET /open/records)
    ///
    /// Retrieves a list of records.
    ///
    /// # Returns
    ///
    /// Returns a list of records
    pub async fn list_records(&self) -> Result<RecordEventListResponse> {
        self.get("/open/records").await
    }

    /// Get record detail (GET /open/record/:id)
    ///
    /// Retrieves detailed information about a specific record.
    ///
    /// # Arguments
    ///
    /// * `id` - Record ID
    ///
    /// # Returns
    ///
    /// Returns detailed record information
    pub async fn get_record_detail(&self, id: i32) -> Result<RecordEvent> {
        self.get(&format!("/open/record/{}", id)).await
    }

    /// Get rule detect log (GET /open/record/rule/:id)
    ///
    /// Retrieves rule detect log for a specific record.
    ///
    /// # Arguments
    ///
    /// * `id` - Record ID
    ///
    /// # Returns
    ///
    /// Returns rule detect log
    pub async fn get_rule_detect_log(&self, id: i32) -> Result<DetectLog> {
        self.get(&format!("/open/record/rule/{}", id)).await
    }

    /// List ACL records (GET /open/records/acl)
    ///
    /// Retrieves ACL records.
    ///
    /// # Returns
    ///
    /// Returns ACL records
    pub async fn list_acl_records(&self) -> Result<RecordEventListResponse> {
        self.get("/open/records/acl").await
    }

    /// List auth defense records (GET /open/records/auth_defense)
    ///
    /// Retrieves auth defense records.
    ///
    /// # Returns
    ///
    /// Returns auth defense records
    pub async fn list_auth_defense_records(&self) -> Result<RecordEventListResponse> {
        self.get("/open/records/auth_defense").await
    }

    /// List challenge records (GET /open/records/challenge)
    ///
    /// Retrieves challenge records.
    ///
    /// # Returns
    ///
    /// Returns challenge records
    pub async fn list_challenge_records(&self) -> Result<RecordEventListResponse> {
        self.get("/open/records/challenge").await
    }

    /// List rule records (GET /open/records/rule)
    ///
    /// Retrieves rule records.
    ///
    /// # Returns
    ///
    /// Returns rule records
    pub async fn list_rule_records(&self) -> Result<RecordEventListResponse> {
        self.get("/open/records/rule").await
    }

    /// List waiting records (GET /open/records/waiting)
    ///
    /// Retrieves waiting room records.
    ///
    /// # Returns
    ///
    /// Returns waiting room records
    pub async fn list_waiting_records(&self) -> Result<RecordEventListResponse> {
        self.get("/open/records/waiting").await
    }

    /// Get detect log list (GET /open/events)
    ///
    /// Retrieves a list of detect logs.
    ///
    /// # Returns
    ///
    /// Returns a list of detect logs
    pub async fn get_detect_log_list(&self) -> Result<DetectLogListResponse> {
        self.get("/open/events").await
    }

    /// Get rule detect log list (GET /open/events/rule)
    ///
    /// Retrieves a list of rule detect logs.
    ///
    /// # Returns
    ///
    /// Returns a list of rule detect logs
    pub async fn get_rule_detect_log_list(&self) -> Result<DetectLogListResponse> {
        self.get("/open/events/rule").await
    }
}