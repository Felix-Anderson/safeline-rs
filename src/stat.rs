use crate::client::Client;
use crate::error::Result;
use crate::models::*;

impl Client {
    /// Get dashboard user counts (GET /open/dashboard/user/counts)
    ///
    /// Retrieves dashboard user counts.
    ///
    /// # Returns
    ///
    /// Returns dashboard user counts
    pub async fn get_dashboard_user_counts(&self) -> Result<DashboardUserCountsResponse> {
        self.get("/open/dashboard/user/counts").await
    }

    /// Get dashboard trend (GET /open/dashboard/requests)
    ///
    /// Retrieves dashboard trend data.
    ///
    /// # Returns
    ///
    /// Returns dashboard trend data
    pub async fn get_dashboard_trend(&self) -> Result<DashboardTrendResponse> {
        self.get("/open/dashboard/requests").await
    }

    /// Get basic access stats (GET /stat/basic/access)
    ///
    /// Retrieves basic access statistics.
    ///
    /// # Returns
    ///
    /// Returns basic access statistics
    pub async fn get_basic_access(&self) -> Result<BasicAccessResponse> {
        self.get("/stat/basic/access").await
    }

    /// Get basic attack stats (GET /stat/basic/attack)
    ///
    /// Retrieves basic attack statistics.
    ///
    /// # Returns
    ///
    /// Returns basic attack statistics
    pub async fn get_basic_attack(&self) -> Result<BasicAttackResponse> {
        self.get("/stat/basic/attack").await
    }

    /// Get advance access stats (GET /stat/advance/access)
    ///
    /// Retrieves advance access statistics.
    ///
    /// # Returns
    ///
    /// Returns advance access statistics
    pub async fn get_advance_access(&self) -> Result<AdvanceAccessResponse> {
        self.get("/stat/advance/access").await
    }

    /// Get advance attack stats (GET /stat/advance/attack)
    ///
    /// Retrieves advance attack statistics.
    ///
    /// # Returns
    ///
    /// Returns advance attack statistics
    pub async fn get_advance_attack(&self) -> Result<AdvanceAttackResponse> {
        self.get("/stat/advance/attack").await
    }

    /// Get advance client stats (GET /stat/advance/client)
    ///
    /// Retrieves advance client statistics.
    ///
    /// # Returns
    ///
    /// Returns advance client statistics
    pub async fn get_advance_client(&self) -> Result<AdvanceClientResponse> {
        self.get("/stat/advance/client").await
    }

    /// Get advance error status code stats (GET /stat/advance/error_status_code)
    ///
    /// Retrieves advance error status code statistics.
    ///
    /// # Returns
    ///
    /// Returns advance error status code statistics
    pub async fn get_advance_error_status_code(&self) -> Result<AdvanceErrorStatusCodeResponse> {
        self.get("/stat/advance/error_status_code").await
    }

    /// Get QPS stats (GET /stat/qps)
    ///
    /// Retrieves QPS statistics.
    ///
    /// # Returns
    ///
    /// Returns QPS statistics
    pub async fn get_qps(&self) -> Result<QpsResponse> {
        self.get("/stat/qps").await
    }

    /// Get advance domain stats (GET /stat/advance/domain)
    ///
    /// Retrieves advance domain statistics.
    ///
    /// # Returns
    ///
    /// Returns advance domain statistics
    pub async fn get_advance_domain(&self) -> Result<AdvanceAccessResponse> {
        self.get("/stat/advance/domain").await
    }

    /// Get advance page stats (GET /stat/advance/page)
    ///
    /// Retrieves advance page statistics.
    ///
    /// # Returns
    ///
    /// Returns advance page statistics
    pub async fn get_advance_page(&self) -> Result<AdvanceAccessResponse> {
        self.get("/stat/advance/page").await
    }

    /// Get advance location stats (GET /stat/advance/location)
    ///
    /// Retrieves advance location statistics.
    ///
    /// # Returns
    ///
    /// Returns advance location statistics
    pub async fn get_advance_location(&self) -> Result<AdvanceAccessResponse> {
        self.get("/stat/advance/location").await
    }

    /// Get basic error status code stats (GET /stat/basic/error_status_code)
    ///
    /// Retrieves basic error status code statistics.
    ///
    /// # Returns
    ///
    /// Returns basic error status code statistics
    pub async fn get_basic_error_status_code(&self) -> Result<AdvanceErrorStatusCodeResponse> {
        self.get("/stat/basic/error_status_code").await
    }

    /// Get basic location stats (GET /stat/basic/location)
    ///
    /// Retrieves basic location statistics.
    ///
    /// # Returns
    ///
    /// Returns basic location statistics
    pub async fn get_basic_location(&self) -> Result<AdvanceAccessResponse> {
        self.get("/stat/basic/location").await
    }

    /// Get advance trend access stats (GET /stat/advance/trend/access)
    ///
    /// Retrieves advance trend access statistics.
    ///
    /// # Returns
    ///
    /// Returns advance trend access statistics
    pub async fn get_advance_trend_access(&self) -> Result<DashboardTrendResponse> {
        self.get("/stat/advance/trend/access").await
    }

    /// Get advance trend intercept stats (GET /stat/advance/trend/intercept)
    ///
    /// Retrieves advance trend intercept statistics.
    ///
    /// # Returns
    ///
    /// Returns advance trend intercept statistics
    pub async fn get_advance_trend_intercept(&self) -> Result<DashboardTrendResponse> {
        self.get("/stat/advance/trend/intercept").await
    }
}