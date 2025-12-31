use crate::client::Client;
use crate::error::Result;
use crate::models::*;

impl Client {
    /// Get JA4 fingerprint (GET /open/ja4)
    ///
    /// Retrieves JA4 fingerprint information.
    ///
    /// # Returns
    ///
    /// Returns JA4 fingerprint response
    pub async fn get_ja4(&self) -> Result<Ja4Response> {
        self.get("/open/ja4").await
    }
}