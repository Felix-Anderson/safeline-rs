use crate::client::Client;
use crate::error::Result;
use crate::models::*;

impl Client {
    /// Get CSRF token (GET /open/auth/csrf)
    ///
    /// Retrieves a CSRF token for authentication.
    ///
    /// # Returns
    ///
    /// Returns a CSRF token
    pub async fn get_csrf_token(&self) -> Result<GetCsrfTokenResponse> {
        self.get("/open/auth/csrf").await
    }

    /// Login (POST /open/auth/login)
    ///
    /// Authenticates a user with username and password.
    ///
    /// # Arguments
    ///
    /// * `request` - Login request with username, password, and CSRF token
    ///
    /// # Returns
    ///
    /// Returns login response with JWT token
    pub async fn login(&self, request: &LoginRequest) -> Result<LoginResponse> {
        self.post("/open/auth/login", request).await
    }

    /// Two-factor authentication (POST /open/auth/tfa)
    ///
    /// Completes two-factor authentication.
    ///
    /// # Arguments
    ///
    /// * `request` - TFA request with code and CSRF token
    ///
    /// # Returns
    ///
    /// Returns TFA response with JWT token
    pub async fn tfa(&self, request: &TfaRequest) -> Result<TfaResponse> {
        self.post("/open/auth/tfa", request).await
    }

    /// Logout (POST /open/auth/logout)
    ///
    /// Logs out the current user.
    pub async fn logout(&self) -> Result<()> {
        self.post::<(), ()>("/open/auth/logout", &()).await
    }

    /// Get auth token (GET /open/auth/token)
    ///
    /// Retrieves the current authentication token.
    ///
    /// # Returns
    ///
    /// Returns the auth token
    pub async fn get_auth_token(&self) -> Result<GetAuthTokenResponse> {
        self.get("/open/auth/token").await
    }
}