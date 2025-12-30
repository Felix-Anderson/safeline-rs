use thiserror::Error;

/// SafeLine SDK Error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON serialization/deserialization failed: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("API error (code: {code:?}): {message}")]
    ApiError { code: Option<String>, message: String },

    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("Invalid response format: {0}")]
    InvalidResponse(String),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, Error>;

/// Common API error codes
pub mod error_codes {
    pub const SUCCESS: Option<&str> = None;
    // pub const INVALID_PARAMS: i32 = 1001;
    // pub const AUTH_FAILED: i32 = 1002;
    // pub const NOT_FOUND: i32 = 1003;
    pub const INTERNAL_ERROR: Option<&str> = Some("internal-error");
}

/// Check if response code indicates success
pub fn is_success(code: Option<&str>) -> bool {
    code == error_codes::SUCCESS
}

/// Check if response code indicates authentication error
pub fn is_auth_error(code: Option<&str>) -> bool {
    code == Some("auth-failed")
}

/// Check if response code indicates resource not found error
pub fn is_not_found_error(code: Option<&str>) -> bool {
    code == Some("not-found")
}
