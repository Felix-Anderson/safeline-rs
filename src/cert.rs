use crate::client::Client;
use crate::error::Result;
use crate::models::*;

impl Client {
    /// List certificates (GET /open/cert)
    ///
    /// Retrieves a list of certificates.
    ///
    /// # Returns
    ///
    /// Returns a list of certificates
    pub async fn list_certs(&self) -> Result<CertListResponse> {
        self.get("/open/cert").await
    }

    /// Get certificate detail (GET /open/cert/{id})
    ///
    /// Retrieves detailed information about a specific certificate.
    ///
    /// # Arguments
    ///
    /// * `id` - Certificate ID
    ///
    /// # Returns
    ///
    /// Returns detailed certificate information
    pub async fn get_cert_detail(&self, id: i32) -> Result<CertDetail> {
        self.get(&format!("/open/cert/{}", id)).await
    }
}