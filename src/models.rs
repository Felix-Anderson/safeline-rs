use serde::{Deserialize, Serialize};

// ============================================================================
// IP Group Models
// ============================================================================

/// IP Group structure based on API documentation ipg.IPGroupVO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroup {
    pub id: Option<i32>,
    pub builtin: Option<bool>,
    pub comment: Option<String>,
    pub ips: Option<Vec<String>>,
    pub reference: Option<String>,
    pub total: Option<i32>,
    pub updated_at: Option<String>,
}

/// IP Group list request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupListRequest {
    pub top: Option<i32>,
}

/// IP Group list response based on API documentation api.GetIPGroupRes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupListResponse {
    pub nodes: Vec<IPGroup>,
    pub total: i32,
}

/// IP Group create request based on API documentation ipg.IPGroupVO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCreateRequest {
    pub comment: Option<String>,
    pub ips: Vec<String>,
    pub reference: Option<String>,
}

/// IP Group create response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCreateResponse {
    pub data: i32,
}

/// IP Group update request based on API documentation ipg.IPGroupVO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupUpdateRequest {
    pub id: i32,
    pub builtin: Option<bool>,
    pub comment: Option<String>,
    pub ips: Option<Vec<String>>,
    pub reference: Option<String>,
}

/// IP Group update response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupUpdateResponse {}

/// IP Group delete request based on API documentation api.DeleteIPGroupReq
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupDeleteRequest {
    pub ids: Vec<i32>,
}

/// IP Group delete response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupDeleteResponse {}

/// IP Group detail request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupDetailRequest {
    pub id: i32,
}

/// IP Group detail response based on API documentation api.GetIPGroupDetailRes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupDetailResponse {
    pub data: IPGroup,
}

/// IP Group append request based on API documentation api.PostAppendIPGroupRequest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupAppendRequest {
    pub ip_group_ids: Vec<i32>,
    pub ips: Vec<String>,
}

/// IP Group append response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupAppendResponse {}

/// Search engine spider group response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCrawlerResponse {
    pub data: i32,
}

/// Search engine spider update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCrawlerUpdateRequest {}

/// Search engine spider update response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCrawlerUpdateResponse {}

/// IP Group by link request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupLinkRequest {
    pub href: String,
}

/// IP Group by link response based on API documentation api.GetIPGroupByLinkRes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupLinkResponse {
    pub data: IPGroupLinkData,
}

/// IP Group link data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupLinkData {
    pub ips: Vec<String>,
}

/// IP Group create by link request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCreateByLinkRequest {
    pub comment: String,
    pub url: String,
}

/// IP Group create by link response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPGroupCreateByLinkResponse {}

// ============================================================================
// Auth Models
// ============================================================================

/// CSRF token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCsrfTokenResponse {
    pub data: GetCsrfTokenData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCsrfTokenData {
    pub csrf_token: String,
}

/// Login request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub csrf_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test: Option<bool>,
}

/// Login response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub id: i32,
    pub jwt: String,
    pub redirect: Option<String>,
    pub tfa_enabled: bool,
    pub tfa_binded: bool,
    pub tfa_bind_url: Option<String>,
}

/// Two-factor authentication request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfaRequest {
    pub code: String,
    pub csrf_token: String,
    pub timestamp: i64,
}

/// Two-factor authentication response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfaResponse {
    pub data: TfaData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfaData {
    pub jwt: String,
}

/// Get auth token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthTokenRequest {}

/// Get auth token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthTokenResponse {
    pub data: String,
}

// ============================================================================
// Site Models
// ============================================================================

/// Website request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub server_names: Vec<String>,
    pub ports: Vec<String>,
    pub upstreams: Vec<String>,
    pub group_id: i32,
    pub comment: Option<String>,
    pub email: Option<String>,
    pub cert_id: Option<i32>,
    pub load_balance: Option<String>,
    pub redirect_status_code: Option<i32>,
    pub health_check: Option<HealthCheck>,
    pub stat_enabled: Option<bool>,
    pub static_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub hosts: Vec<String>,
    pub upstreams: Vec<String>,
}

/// Website response item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteItem {
    pub id: i32,
    pub server_names: Vec<String>,
    pub ports: Vec<String>,
    pub upstreams: Vec<String>,
    pub group_id: i32,
    pub comment: Option<String>,
    pub email: Option<String>,
    pub cert_id: Option<i32>,
    pub cert_filename: Option<String>,
    pub key_filename: Option<String>,
    pub cert_type: Option<i32>,
    pub load_balance: Option<String>,
    pub redirect_status_code: Option<i32>,
    pub health_check: Option<HealthCheck>,
    pub stat_enabled: Option<bool>,
    pub static_default: Option<bool>,
    pub index: Option<i32>,
    pub r#type: Option<i32>,
    pub is_enabled: bool,
    pub mode: i32,
    pub title: String,
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: String,
    pub updated_at: String,
    pub acl_enabled: bool,
    pub acl_response_html_path: Option<String>,
    pub acl_response_status_code: Option<i32>,
    pub denied_value: Option<i32>,
    pub req_value: Option<i32>,
    pub forbidden_html_path: Option<String>,
    pub forbidden_status_code: Option<i32>,
    pub bad_gateway_html_path: Option<String>,
    pub bad_gateway_status_code: Option<i32>,
    pub gateway_timeout_html_path: Option<String>,
    pub gateway_timeout_status_code: Option<i32>,
    pub offline_html_path: Option<String>,
    pub offline_status_code: Option<i32>,
    pub not_found_html_path: Option<String>,
    pub not_found_status_code: Option<i32>,
    pub access_log_limit: Option<i32>,
    pub error_log_limit: Option<i32>,
    pub exclude_content_type: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub auth_defense_id: Option<i32>,
    pub challenge_id: Option<i32>,
    pub chaos_id: Option<i32>,
    pub chaos_is_enabled: Option<bool>,
    pub portal: Option<bool>,
    pub portal_redirect: Option<bool>,
    pub sp_enabled: Option<bool>,
    pub semantics: Option<bool>,
    pub custom_location: Option<String>,
    pub cc_bot: Option<bool>,
    pub tamper_refresh: Option<i32>,
    pub tamper_refresh_state: Option<i32>,
    pub health_state: Option<HealthState>,
    pub init: Option<bool>,
    pub wr_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthState {
    pub state: String,
    pub error: Option<String>,
}

/// Website list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteListResponse {
    pub data: Vec<WebsiteItem>,
    pub total: i32,
}

/// Website detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteDetailResponse {
    pub data: WebsiteDetailData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteDetailData {
    pub id: i32,
    pub server_names: Vec<String>,
    pub ports: Vec<String>,
    pub upstreams: Vec<String>,
    pub group_id: i32,
    pub comment: Option<String>,
    pub email: Option<String>,
    pub cert_id: Option<i32>,
    pub cert_filename: Option<String>,
    pub key_filename: Option<String>,
    pub cert_type: Option<i32>,
    pub load_balance: Option<String>,
    pub redirect_status_code: Option<i32>,
    pub health_check: Option<HealthCheck>,
    pub stat_enabled: Option<bool>,
    pub static_default: Option<bool>,
    pub index: Option<i32>,
    pub r#type: Option<i32>,
    pub is_enabled: bool,
    pub mode: i32,
    pub title: String,
    pub icon: Option<String>,
    pub position: i32,
    pub created_at: String,
    pub updated_at: String,
    pub acl_enabled: bool,
    pub acl_response_html_path: Option<String>,
    pub acl_response_status_code: Option<i32>,
    pub denied_value: Option<i32>,
    pub req_value: Option<i32>,
    pub forbidden_html_path: Option<String>,
    pub forbidden_status_code: Option<i32>,
    pub bad_gateway_html_path: Option<String>,
    pub bad_gateway_status_code: Option<i32>,
    pub gateway_timeout_html_path: Option<String>,
    pub gateway_timeout_status_code: Option<i32>,
    pub offline_html_path: Option<String>,
    pub offline_status_code: Option<i32>,
    pub not_found_html_path: Option<String>,
    pub not_found_status_code: Option<i32>,
    pub access_log_limit: Option<i32>,
    pub error_log_limit: Option<i32>,
    pub exclude_content_type: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
    pub auth_defense_id: Option<i32>,
    pub challenge_id: Option<i32>,
    pub chaos_id: Option<i32>,
    pub chaos_is_enabled: Option<bool>,
    pub portal: Option<bool>,
    pub portal_redirect: Option<bool>,
    pub sp_enabled: Option<bool>,
    pub semantics: Option<bool>,
    pub custom_location: Option<String>,
    pub cc_bot: Option<bool>,
    pub tamper_refresh: Option<i32>,
    pub tamper_refresh_state: Option<i32>,
    pub health_state: Option<HealthState>,
    pub init: Option<bool>,
    pub wr_id: Option<i32>,
}

/// Delete website request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebsiteRequest {
    pub ids: Vec<i32>,
}

/// Put website basic info request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutWebsiteBasicInfoRequest {
    pub comment: Option<String>,
    pub group_id: i32,
    pub icon: Option<String>,
}

/// Put website defense request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutWebsiteDefenseRequest {
    pub id: i32,
    pub enable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_redirect: Option<bool>,
}

/// Put website mode request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutWebsiteModeRequest {
    pub ids: Vec<i32>,
    pub mode: i32,
}

/// Website chaos request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteChaosRequest {
    pub is_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_encryption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_fast_decryption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js_encryption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub js_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_encryption: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_text: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub img_watermark: Option<bool>,
}

/// Website challenge request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteChallengeRequest {
    pub id: i32,
    pub enable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i32>,
}

/// Set website waiting room request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetWebsiteWaitingRoomRequest {
    pub is_enabled: bool,
    pub max_concurrent: i32,
    pub session_timeout: i32,
}

/// Website group request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteGroupRequest {
    pub id: Option<i32>,
    pub name: String,
}

/// Website group response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteGroupResponse {
    pub data: Vec<WebsiteGroupItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteGroupItem {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub site_count: i32,
}

/// Create group request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
}

/// Update group request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateGroupRequest {
    pub name: String,
}

/// Update site group request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSiteGroupRequest {
    pub group_id: i32,
}

/// Delete group request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupRequest {
    pub id: i32,
}

/// Sort group request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortGroupRequest {
    pub position: Vec<i32>,
}

/// Sort website request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SortWebsiteRequest {
    pub position: Vec<i32>,
}

/// Group switch request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupSwitchRequest {
    pub enable: bool,
}

/// Nginx config get response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NginxConfigGetResponse {
    pub data: NginxConfigData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NginxConfigData {
    pub config: String,
    pub custom_location: String,
}

/// Nginx config update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NginxConfigUpdateRequest {
    pub custom_location: String,
}

/// Website health check request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteHealthCheckRequest {
    pub hosts: Vec<String>,
    pub upstreams: Vec<String>,
}

/// Create website response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebsiteResponse {
    pub data: i32,
}

/// Create website group response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWebsiteGroupResponse {
    pub data: i32,
}

/// Website group switch response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteGroupSwitchResponse {
    pub data: bool,
}

// ============================================================================
// Policy Models
// ============================================================================

/// Policy request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<bool>,
}

/// Policy response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyResponse {
    pub id: i32,
    pub name: String,
    pub action: i32,
    pub level: i32,
    pub pattern: Option<String>,
    pub expire: Option<i32>,
    pub is_enabled: bool,
    pub log: bool,
    pub builtin: bool,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_rule: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_rule: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub white_rule: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub captcha_rule: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source_ids: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negate: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_total: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pass_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_count: Option<i32>,
}

/// Policy list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyListResponse {
    pub data: Vec<PolicyResponse>,
    pub total: i32,
}

/// Delete policy request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePolicyRequest {
    pub id: i32,
}

/// Policy switch request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicySwitchRequest {
    pub id: i32,
    pub is_enabled: bool,
}

/// Policy detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyDetailResponse {
    pub data: PolicyResponse,
}

// ============================================================================
// ACL Models
// ============================================================================

/// ACL config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLConfig {
    pub id: i32,
    pub site_id: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub built_in: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_min: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<MatchCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

/// ACL relieve request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLRelieveRequest {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
}

/// ACL logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLLogsResponse {
    pub data: Vec<ACLLog>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLLog {
    pub id: i32,
    pub ip: String,
    pub site_id: i32,
    pub action: String,
    pub result: String,
    pub status: String,
    pub period: i32,
    pub count: i32,
    pub denied_count: i32,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_before: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_server_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_title: Option<String>,
}

/// Set site ACL request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSiteACLRequest {
    pub rules: Vec<ACLConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_global: Option<bool>,
}

/// ACL search request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLSearchRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site: Option<i32>,
}

// ============================================================================
// Record Models
// ============================================================================

/// Detect log
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectLog {
    pub id: i32,
    pub event_id: String,
    pub site_uuid: String,
    pub website: String,
    pub src_ip: String,
    pub src_port: i32,
    pub dst_ip: String,
    pub dst_port: i32,
    pub host: String,
    pub method: String,
    pub url_path: String,
    pub protocol: String,
    pub module: String,
    pub attack_type: String,
    pub risk_level: String,
    pub action: String,
    pub reason: String,
    pub rule_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_rule_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id_list: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub socket_ip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsp_header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsp_body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decode_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja4_fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lat: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lng: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_name: Option<String>,
}

/// Detect log list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectLogListResponse {
    pub data: Vec<DetectLog>,
    pub total: i32,
}

/// Record event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordEvent {
    pub id: i32,
    pub ip: String,
    pub host: String,
    pub protocol: String,
    pub dst_port: i32,
    pub start_at: i64,
    pub end_at: i64,
    pub pass_count: i32,
    pub deny_count: i32,
    pub finished: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// Record event list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordEventListResponse {
    #[serde(alias = "nodes")]
    pub data: Vec<RecordEvent>,
    pub total: i32,
}

// ============================================================================
// Certificate Models
// ============================================================================

/// Certificate list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertListItem {
    pub id: i32,
    pub domains: Vec<String>,
    pub issuer: String,
    pub valid_before: i64,
    pub expired: bool,
    pub trusted: bool,
    pub self_signature: bool,
    pub revoked: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_sites: Option<Vec<i32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acme_message: Option<String>,
}

/// Certificate list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertListResponse {
    pub nodes: Vec<CertListItem>,
    pub total: i32,
}

/// Certificate detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertDetail {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<CertManual>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acme: Option<CertAcme>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertManual {
    pub crt: String,
    pub key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertAcme {
    pub domains: Vec<String>,
    pub email: String,
}

// ============================================================================
// JA4 Models
// ============================================================================

/// JA4 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ja4Response {
    pub id: i32,
    pub name: String,
    pub count: i32,
    pub updated_at: String,
}

// ============================================================================
// Stat Models
// ============================================================================

/// Dashboard user counts response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserCountsResponse {
    pub data: DashboardUserCountsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardUserCountsData {
    pub ip: i32,
    pub pv: i32,
    pub uv: i32,
    pub today_ip: i32,
    pub today_pv: i32,
    pub today_uv: i32,
}

/// Dashboard trend response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTrendResponse {
    pub nodes: Vec<StatNode>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatNode {
    pub label: String,
    pub value: i64,
}

/// Basic access response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAccessResponse {
    pub data: BasicAccessData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAccessData {
    pub access: Vec<StatNode>,
}

/// Basic attack response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAttackResponse {
    pub data: BasicAttackData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicAttackData {
    pub attack_ip: Vec<StatNode>,
    pub intercept: Vec<StatNode>,
}

/// Advance access response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceAccessResponse {
    pub data: AdvanceAccessData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceAccessData {
    pub access: Vec<StatNode>,
    pub ip: Vec<StatNode>,
    pub session: Vec<StatNode>,
}

/// Advance attack response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceAttackResponse {
    pub data: AdvanceAttackData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceAttackData {
    pub attack_ip: Vec<StatNode>,
    pub intercept: Vec<StatNode>,
}

/// Advance client response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceClientResponse {
    pub data: AdvanceClientData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceClientData {
    pub browser: Vec<StatNode>,
    pub os: Vec<StatNode>,
}

/// Advance error status code response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceErrorStatusCodeResponse {
    pub data: AdvanceErrorStatusCodeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdvanceErrorStatusCodeData {
    pub error_4xx: i32,
    pub error_5xx: i32,
    pub total: i32,
}

/// QPS response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QpsResponse {
    pub data: QpsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QpsData {
    pub nodes: Vec<StatNode>,
}

// ============================================================================
// System Models
// ============================================================================

/// System about response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAboutResponse {
    pub data: SystemAboutData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemAboutData {
    pub version: String,
    pub time: String,
    pub machine_id: String,
    pub license: Option<SystemLicense>,
    pub cert_id: Option<i32>,
    pub password_expire_day: Option<i32>,
    pub slave: bool,
    pub staging: bool,
    pub outdated: bool,
    pub deprecated: bool,
    pub oversea: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLicense {
    pub valid: bool,
}

/// System edition response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEditionResponse {
    pub data: SystemEditionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemEditionData {
    pub state: String,
    pub version: String,
}

/// Log max day config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMaxDayConfig {
    pub max_day: i32,
    pub max_stat_day: i32,
}

/// System update request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemUpdateRequest {
    pub cert_id: i32,
}

/// Code apply info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeApplyInfoResponse {
    pub data: CodeApplyInfoData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeApplyInfoData {
    pub state: String,
    pub user_id: i32,
    pub org_id: String,
    pub org_name: String,
    pub role: String,
    pub version: String,
    pub pwd_updated_at: String,
    pub expired_at: String,
    pub timeout: i32,
    pub river_url: String,
}

/// Code apply request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeApplyRequest {
    pub code: String,
}

// ============================================================================
// Alarm Models
// ============================================================================

/// Alarm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alarm {
    pub id: i32,
    pub created_at: String,
    pub updated_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur_platform: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<AlarmPlatform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<AlarmEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy: Option<NetworkProxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmPlatform {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ding_talk: Option<DingTalkConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord: Option<DiscordConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feishu: Option<FeishuConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qiye_weixin: Option<QiyeWeixinConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telegram: Option<TelegramConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DingTalkConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeishuConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sign: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QiyeWeixinConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub token: String,
    pub chat_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acl: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub black_white: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenge: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_defense: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tamper: Option<AlarmConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting_room: Option<AlarmConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfig {
    pub enabled: bool,
    pub alarm_period: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkProxy {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

/// Put alarm config request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutAlarmConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cur_platform: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<AlarmPlatform>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<AlarmEvent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy: Option<NetworkProxy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_ids: Option<Vec<i32>>,
}

/// Alarm config response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfigResponse {
    pub data: AlarmConfigResponseData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlarmConfigResponseData {
    pub id: i32,
}

// ============================================================================
// Portal Models
// ============================================================================

/// Portal get response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalGetResponse {
    pub data: PortalData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalData {
    pub domain: String,
    pub enable: bool,
    pub ports: Vec<i32>,
    pub cert_id: i32,
    pub auth_source_ids: Vec<i32>,
    pub tfa_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_sources: Option<Vec<PortalAuthSource>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalAuthSource {
    pub id: i32,
    pub r#type: i32,
}

/// Portal proxy config request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalProxyConfigRequest {
    pub force_https: bool,
}

// ============================================================================
// Report Models
// ============================================================================

/// Report list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportListItem {
    pub id: i32,
    pub name: String,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub created_at: String,
}

/// Report list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportListResponse {
    pub data: Vec<ReportListItem>,
    pub total: i32,
}

/// Report create request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCreateRequest {
    pub name: String,
    pub begin_time: i64,
    pub end_time: i64,
}

/// Report get response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportGetResponse {
    pub data: ReportData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportData {
    pub id: i32,
    pub name: String,
    pub begin_time: i64,
    pub end_time: i64,
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<ReportDataData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<ReportDataCount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attack: Option<ReportDataAttack>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geo: Option<ReportDataGeo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trend: Option<ReportDataTrend>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataCount {
    pub request: i32,
    pub intercept: i32,
    pub ip: i32,
    pub uv: i32,
    pub website: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataAttack {
    pub src_ip: Vec<ReportDataAttackItem>,
    pub r#type: Vec<ReportDataAttackItem>,
    pub website: Vec<ReportDataAttackItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataAttackItem {
    pub key: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataGeo {
    pub request_country: Vec<ReportDataGeoItem>,
    pub request_province: Vec<ReportDataGeoItem>,
    pub intercept_country: Vec<ReportDataGeoItem>,
    pub intercept_province: Vec<ReportDataGeoItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataGeoItem {
    pub location: String,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataTrend {
    pub request: Vec<ReportDataTrendItem>,
    pub intercept: Vec<ReportDataTrendItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDataTrendItem {
    pub trend_time: i64,
    pub count: i32,
}

// ============================================================================
// Anti Tamper Models
// ============================================================================

/// Anti tamper create request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiTamperCreateRequest {
    pub resource_ids: Vec<i32>,
}

/// Anti tamper list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiTamperListResponse {
    pub data: Vec<AntiTamperListData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_state: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiTamperListData {
    pub id: i32,
    pub site_id: i32,
    pub path: String,
    pub changed: bool,
    pub cache_time: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

/// Anti tamper detail response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiTamperDetailResponse {
    pub data: AntiTamperDetailData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiTamperDetailData {
    pub id: i32,
    pub site_id: i32,
    pub path: String,
    pub content_type: String,
    pub cache_sha256: String,
    pub cache_time: i64,
    pub change_sha256: String,
    pub change_time: i64,
    pub changed: bool,
    pub created_at: String,
    pub updated_at: String,
}

// ============================================================================
// Auth Defense Models
// ============================================================================

/// Auth defense user list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserListResponse {
    pub data: Vec<AuthDefenseUserListItem>,
    pub total: i32,
    pub limit: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserListItem {
    pub id: i32,
    pub username: String,
    pub tfa_enabled: bool,
    pub tfa_binded: bool,
    pub last_login: Option<i64>,
    pub binds: Vec<AuthDefenseUserBind>,
    pub websites: Vec<AuthDefenseUserSite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserBind {
    pub source_type: i32,
    pub third_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserSite {
    pub site_id: i32,
    pub status: i32,
}

/// Auth defense get user response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseGetUserResponse {
    pub data: AuthDefenseGetUserData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseGetUserData {
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tfa_enabled: bool,
    pub last_login: Option<i64>,
    pub binds: Vec<AuthDefenseUserBind>,
    pub websites: Vec<AuthDefenseUserSite>,
}

/// Auth defense create user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseCreateUserRequest {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
}

/// Auth defense update user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUpdateUserRequest {
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub websites: Option<Vec<AuthDefenseUserSite>>,
}

/// Auth defense delete user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseDeleteUserRequest {
    pub ids: Vec<i32>,
}

/// Auth defense source list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceListResponse {
    pub data: Vec<AuthDefenseSourceListItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceListItem {
    pub id: i32,
    pub r#type: i32,
}

/// Auth defense create source request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseCreateSourceRequest {
    pub title: String,
    pub r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<PolicyAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy: Option<NetworkProxy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap: Option<PolicyAuthLDAP>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas: Option<PolicyAuthCAS>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2: Option<PolicyAuthOauth2>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAuthLDAP {
    pub url: String,
    pub bind_dn: String,
    pub bind_pass: String,
    pub base_dn: String,
    pub filter: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAuthCAS {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAuthOauth2 {
    pub provider: String,
    pub app_id: String,
    pub app_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

/// Auth defense update source request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUpdateSourceRequest {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<PolicyAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy: Option<NetworkProxy>,
}

/// Auth defense source user list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceUserListResponse {
    pub data: Vec<AuthDefenseSourceUserListItem>,
    pub total: i32,
    pub user_total: i32,
    pub approved_total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceUserListItem {
    pub id: i32,
    pub uname: String,
    pub status: i32,
    pub last_login: Option<i64>,
}

/// Auth defense put source user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefensePutSourceUserRequest {
    pub status: i32,
}

/// Auth defense unbind user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUnbindUserRequest {
    pub r#type: i32,
}

/// Auth defense user review list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserReviewListResponse {
    pub data: Vec<AuthDefenseUserReviewListItem>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserReviewListItem {
    pub id: i32,
    pub username: String,
    pub site_id: i32,
    pub site_title: String,
    pub site_icon: String,
    pub site_server_names: Vec<String>,
    pub site_comment: String,
    pub status: i32,
    pub request_time: i64,
    pub auth_user_id: i32,
}

/// Auth defense user review request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserReviewRequest {
    pub status: i32,
}

/// Auth defense merge user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseMergeUserRequest {
    pub main: i32,
    pub merges: Vec<i32>,
    pub dry_run: bool,
}

/// Auth defense merge user response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseMergeUserResponse {
    pub data: AuthDefenseGetUserData,
}

/// Auth defense logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseLogsResponse {
    pub data: Vec<AuthDefenseLog>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseLog {
    pub id: i32,
    pub site_id: i32,
    pub site_title: String,
    pub site_icon: String,
    pub site_server_names: Vec<String>,
    pub site_comment: String,
    pub ip: String,
    pub username: Option<String>,
    pub user_id: Option<i32>,
    pub status: String,
    pub source_type: Option<i32>,
    pub third_id: Option<String>,
    pub result: String,
    pub policy_id: Option<i32>,
    pub deny_count: i32,
    pub pass_count: i32,
    pub trigger_count: i32,
    pub dur_sec: i32,
    pub started_at: i64,
    pub ended_at: i64,
    pub rule_id: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub uuid: String,
    pub event_id: String,
}

// ============================================================================
// Challenge Models
// ============================================================================

/// Challenge config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeConfig {
    pub server: i32,
}

/// Challenge logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeLogsResponse {
    pub data: Vec<ChallengeLog>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeLog {
    pub id: i32,
    pub site_id: i32,
    pub site_title: String,
    pub site_icon: String,
    pub site_server_names: Vec<String>,
    pub site_comment: String,
    pub ip: String,
    pub rule_id: Option<i32>,
    pub pass_count: i32,
    pub trigger_count: i32,
    pub dur_sec: i32,
    pub started_at: i64,
    pub ended_at: i64,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub event_id: String,
}

// ============================================================================
// Cloud Models
// ============================================================================

/// Cloud policies item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudPoliciesItem {
    pub id: i32,
    pub policy_name: String,
    pub org_name: String,
    pub tags: Vec<String>,
    pub compatible: bool,
    pub auth_rule: i32,
    pub black_rule: i32,
    pub white_rule: i32,
    pub captcha_rule: i32,
    pub created_at: String,
    pub updated_at: String,
    pub added: bool,
}

/// Cloud policies subscribe request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudPoliciesSubscribeRequest {
    pub id: i32,
}

// ============================================================================
// Detector Models
// ============================================================================

/// Detector mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorMode {
    pub mode: i32,
}

/// Detector request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorRequest {
    pub mode: i32,
    pub ts: i64,
}

/// Skynet info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkynetInfo {
    pub id: i32,
    pub name: String,
    pub attack_type: String,
    pub mode: i32,
    pub risk_level: String,
}

// ============================================================================
// Intelligence Models
// ============================================================================

/// Intelligence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intelligence {
    pub ip_group_id: i32,
    pub policy_rule_id: i32,
    pub share_enabled: bool,
    pub use_commercial_lib: bool,
    pub updated_at: String,
}

/// Put threat info request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutThreatInfoRequest {
    pub share_enabled: bool,
}

/// Put threat lib request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutThreatLibRequest {
    pub use_commercial_lib: bool,
}

// ============================================================================
// MCP Models
// ============================================================================

/// MCP get response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpGetResponse {
    pub data: McpData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpData {
    pub server: String,
    pub secret: String,
}

/// MCP set request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpSetRequest {
    pub server: String,
    pub secret: String,
}

// ============================================================================
// Security Posture Models
// ============================================================================

/// Security posture statistics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureStatisticsResponse {
    pub data: SecurityPostureStatisticsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureStatisticsData {
    pub attack_allow: i32,
    pub attack_deny: i32,
    pub acl_hit: i32,
    pub black_hit: i32,
    pub white_hit: i32,
    pub auth_allow: i32,
    pub auth_deny: i32,
    pub challenge_allow: i32,
    pub challenge_deny: i32,
    pub waiting_hit: i32,
    pub anti_tamper: i32,
}

/// Security posture realtime response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureRealtimeResponse {
    pub data: SecurityPostureRealtimeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureRealtimeData {
    pub event: Vec<SecurityPostureRealtimeEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureRealtimeEvent {
    pub time_stamp: i64,
    pub event_type: String,
    pub site_id: i32,
    pub site_name: String,
    pub event: SecurityPostureEvent,
    pub attack_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureEvent {
    pub ip: String,
    pub host: String,
    pub port: i32,
    pub start: i64,
    pub end: i64,
}

/// Security posture trends response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureTrendsResponse {
    pub data: SecurityPostureTrendsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureTrendsData {
    pub pass: i32,
    pub top: i32,
    pub item: Vec<StatNode>,
}

/// Set site security posture request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetSiteSecurityPostureRequest {
    pub site_id: i32,
    pub enabled: bool,
}

// ============================================================================
// Skynet Models
// ============================================================================

/// Get global semantics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGlobalSemanticsResponse {
    pub data: GetGlobalSemanticsData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGlobalSemanticsData {
    pub semantics: Vec<String>,
}

/// Semantics config params
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticsConfigParams {
    pub semantics: Vec<String>,
    pub use_global: bool,
}

/// Get skynet rule response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSkynetRuleResponse {
    pub data: GetSkynetRuleData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSkynetRuleData {
    pub rules: Vec<SkynetInfo>,
}

/// Put skynet rule request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutSkynetRuleRequest {
    pub id: i32,
    pub mode: i32,
    pub global: bool,
}

/// Put skynet rule switch request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutSkynetRuleSwitchRequest {
    pub enable: bool,
}

// ============================================================================
// User Models
// ============================================================================

/// User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub role: String,
    pub tfa_enabled: bool,
    pub tfa_binded: bool,
    pub password_enabled: bool,
    pub last_login: Option<i64>,
    pub pwd_updated_at: Option<i64>,
    pub lock_until: Option<i64>,
}

/// User list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListResponse {
    pub data: Vec<User>,
}

/// Create user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
}

/// Create user response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserResponse {
    pub data: CreateUserData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserData {
    pub id: i32,
}

/// Update user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tfa_enabled: Option<bool>,
}

/// Delete user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteUserRequest {
    pub id: i32,
}

/// Unlock user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnlockUserRequest {}

/// Get user totp request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTotpRequest {}

/// Get user totp response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTotpResponse {
    pub data: GetUserTotpData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserTotpData {
    pub secret: String,
    pub qr_code: String,
}

// ============================================================================
// Misc Models
// ============================================================================

/// Manager info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagerInfo {
    pub manager_info: String,
    pub page_manager_info: String,
    pub common_color: CommonColor,
    pub error_color: ErrorColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonColor {
    pub primary_color: String,
    pub light_primary_color: String,
    pub font_color: String,
    pub light_font_color: String,
    pub success_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorColor {
    pub warning_color: String,
    pub warning_font_color: String,
    pub warning_light_font_color: String,
}

/// Put manager info request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutManagerInfoRequest {
    pub manager_info: String,
    pub page_manager_info: String,
    pub common_color: CommonColor,
    pub error_color: ErrorColor,
}

/// Get manager info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetManagerInfoResponse {
    pub data: ManagerInfo,
}

/// Get special page response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpecialPageResponse {
    pub data: GetSpecialPageData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSpecialPageData {
    pub content: String,
}

/// Put special page request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutSpecialPageRequest {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recover: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_html: Option<String>,
}

/// List special page item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpecialPageItem {
    pub r#type: String,
    pub custom: bool,
}

/// List special page response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListSpecialPageResponse {
    pub data: Vec<ListSpecialPageItem>,
}

/// Get downgrade version response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDowngradeVersionResponse {
    pub data: GetDowngradeVersionData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDowngradeVersionData {
    pub status: String,
}

/// Dashboard token request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTokenRequest {
    pub expired_at: i64,
}

/// Dashboard token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardTokenResponse {
    pub data: String,
}

/// Account config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub login_expire: i32,
    pub password_length: i32,
    pub password_complex: bool,
    pub password_expire_day: i32,
    pub expired_unit: i32,
    pub lock_config: AccountConfigLock,
    pub access_whitelist: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfigLock {
    pub login_failed: i32,
    pub lock_interval: i32,
    pub lock_interval_unit: i32,
}

/// Get frontend style response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFrontendStyleResponse {
    pub data: FrontendStyleData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontendStyleData {
    pub title: String,
    pub icon: String,
}

/// Put frontend style request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutFrontendStyleRequest {
    pub title: String,
    pub icon: String,
}

/// Audit log list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListItem {
    pub id: i32,
    pub username: String,
    pub ip: String,
    pub content: String,
    pub created_at: String,
}

/// Audit log list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListResponse {
    pub data: Vec<AuditLogListItem>,
    pub total: i32,
}

/// Portal style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalStyle {
    pub title: String,
    pub icon: String,
    pub theme: String,
    pub app_arrange: String,
}

/// Receiver config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiverConfig {
    pub r#type: String,
    pub protocol: String,
    pub host: String,
    pub port: i32,
    pub label: String,
    pub log_types: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfc: Option<String>,
}

/// Login method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginMethod {
    pub r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cas: Option<CASConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth: Option<OauthConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CASConfig {
    pub url: String,
    pub version: String,
    pub role_key: String,
    pub mapping: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OauthConfig {
    pub url: String,
    pub client_id: String,
    pub client_secret: String,
    pub scope: String,
    pub role_key: String,
    pub mapping: String,
}

/// Global proxy config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalProxyConfig {
    pub ip_source: i32,
    pub ipv6: bool,
    pub ssl_protocols: Vec<String>,
    pub ssl_ciphers: Vec<String>,
}

/// Website resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebsiteResource {
    pub nodes: Vec<Resource>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: i32,
    pub site_id: i32,
    pub method: String,
    pub path: String,
    pub content_type: String,
    pub content_length: i32,
    pub status_code: i32,
    pub response_time: i32,
    pub req_today: i32,
    pub req_header: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Update excludes request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExcludesRequest {
    pub prefixes: Vec<String>,
    pub content_types: Vec<String>,
}

/// Remove resource request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveResourceRequest {
    pub ids: Vec<i32>,
}

/// Waiting room logs response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitingRoomLogsResponse {
    pub data: Vec<WaitingRoomLog>,
    pub total: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitingRoomLog {
    pub site_id: i32,
    pub site_title: String,
    pub site_icon: String,
    pub site_server_names: Vec<String>,
    pub site_comment: String,
    pub rule_id: i32,
    pub started_at: i64,
    pub ended_at: i64,
    pub dur_sec: i32,
    pub cur_waiting: i32,
    pub max_concurrent: i32,
    pub top_waiting: i32,
    pub total_waiting: i32,
    pub avg_wait_sec: i32,
    pub bounce_rate: i32,
}

/// Waiting room
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitingRoom {
    pub id: i32,
    pub name: String,
    pub website_id: i32,
    pub is_enabled: bool,
    pub max_concurrent: i32,
    pub max_waiting: i32,
    pub session_timeout: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// Chaos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chaos {
    pub id: i32,
    pub name: String,
    pub website_id: i32,
    pub is_enabled: bool,
    pub html_encryption: bool,
    pub html_fast_decryption: bool,
    pub js_encryption: bool,
    pub js_path: String,
    pub img_encryption: bool,
    pub img_text: bool,
    pub img_watermark: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Site excludes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteExcludes {
    pub prefixes: Vec<String>,
    pub content_types: Vec<String>,
}

/// Skynet module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkynetModule {
    pub id: i32,
    pub site_id: i32,
    pub semantics: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Proxy config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub id: i32,
    pub site_id: i32,
    pub host: String,
    pub ip_source: i32,
    pub ip_value: String,
    pub ipv6: bool,
    pub force_https: bool,
    pub http_1_0: bool,
    pub http2: bool,
    pub http3: bool,
    pub gzip: bool,
    pub br: bool,
    pub sse: bool,
    pub hsts: bool,
    pub hsts_max_age: i32,
    pub hsts_preload: bool,
    pub hsts_sub: bool,
    pub ssl_protocols: Vec<String>,
    pub ssl_ciphers: Vec<String>,
    pub ntlm: bool,
    pub reset_xff: bool,
    pub xfh: bool,
    pub xfp: bool,
    pub default_server: bool,
    pub global: bool,
    pub http_headers: Vec<HttpHeader>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpHeader {
    pub key: String,
    pub op: String,
    pub val: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

/// Proxy item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyItemString {
    pub global: bool,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyItemBool {
    pub global: bool,
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyItemHttpHeaders {
    pub global: bool,
    pub value: Vec<HttpHeader>,
}

/// Load balance config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadBalanceConfig {
    pub balance_type: String,
}

/// Portal config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalConfig {
    pub domain: String,
    pub port: i32,
    pub ports: Vec<i32>,
    pub cert_id: i32,
    pub enable: bool,
    pub auth_source_ids: Vec<i32>,
    pub tfa_enabled: bool,
}

/// Rule pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RulePattern {
    pub k: String,
    pub op: String,
    pub v: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_k: Option<String>,
}

/// Rule condition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub condition: String,
    pub strategy: Vec<RulePattern>,
}

/// Set share behaviour request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetShareBehaviourRequest {
    pub enable: bool,
}

/// Set share fingerprint request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetShareFingerprintRequest {
    pub enable: bool,
}

/// File info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub dir: String,
    pub size: i32,
    pub content_type: String,
    pub modify_time: i64,
    pub content: String,
}

/// Add static request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddStaticRequest {
    pub path: String,
    pub dir: bool,
    pub page: bool,
    pub zip: bool,
}

/// Delete static request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStaticRequest {
    pub path: String,
}

/// Rename static request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameStaticRequest {
    pub old_path: String,
    pub new_path: String,
    pub copy: bool,
}

/// List log item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListLogItem {
    pub filename: String,
    pub size: i32,
}

/// Site mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteMode {
    pub mode: i32,
}

/// Site health check request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteHealthCheckRequest {
    pub hosts: Vec<String>,
    pub upstreams: Vec<String>,
}

/// Site health state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteHealthState {
    pub state: String,
    pub error: Option<String>,
}

/// List group item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupItem {
    pub id: i32,
    pub name: String,
    pub position: i32,
    pub site_count: i32,
}

/// List group response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupResponse {
    pub data: Vec<ListGroupItem>,
}

/// Auth defense source
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSource {
    pub id: i32,
    pub title: String,
    pub r#type: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<PolicyAuth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_proxy: Option<NetworkProxy>,
    pub comment: Option<String>,
    pub user: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

/// Auth defense user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUser {
    pub id: i32,
    pub username: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub tfa_enabled: bool,
    pub last_login: Option<i64>,
    pub binds: Vec<AuthDefenseUserBind>,
    pub websites: Vec<AuthDefenseUserSite>,
}

/// Get auth defense logs v2 item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDefenseLogsV2Item {
    pub id: i32,
    pub site_id: i32,
    pub site_title: String,
    pub site_icon: String,
    pub site_server_names: Vec<String>,
    pub site_comment: String,
    pub ip: String,
    pub username: Option<String>,
    pub user_id: Option<i32>,
    pub status: String,
    pub source_type: Option<i32>,
    pub third_id: Option<String>,
    pub result: String,
    pub policy_id: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub province: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub uuid: String,
    pub event_id: String,
}

/// Get auth defense logs v2 response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAuthDefenseLogsV2Response {
    pub data: Vec<GetAuthDefenseLogsV2Item>,
    pub total: i32,
}

/// Auth defense group list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseGroupListItem {
    pub id: i32,
    pub r#type: i32,
}

/// Auth defense group list response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseGroupListResponse {
    pub data: Vec<AuthDefenseGroupListItem>,
}

/// Semantic config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticConfig {}

/// Password config
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordConfig {
    pub title: String,
}

/// Time unit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeUnit {}

/// Match operator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchOperator {}

/// ACL config action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLConfigAction {}

/// ACL config type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ACLConfigType {}

/// Policy action
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyAction {}

/// Policy review
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyReview {}

/// Policy user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyUser {
    pub username: String,
    pub password: String,
}

/// Auth defense source association
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceAssociation {}

/// Auth defense source type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceType {}

/// Auth defense source user status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseSourceUserStatus {}

/// Auth defense user review status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserReviewStatus {}

/// Auth defense user site status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseUserSiteStatus {}

/// Auth defense verify status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthDefenseVerifyStatus {}

/// Password char
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordChar {}

/// Constants challenge server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChallengeServer {}

/// Consts user role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {}

/// Detect mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectMode {}

/// License state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseState {}

/// River edition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiverEdition {}

/// Security posture trends query type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPostureTrendsQueryType {}
