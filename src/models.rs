use serde::{Deserialize, Serialize};

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