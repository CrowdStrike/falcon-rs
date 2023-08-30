/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainPeriodGcpAccountV1 {
    #[serde(rename = "CreatedAt")]
    pub created_at: String,
    #[serde(rename = "DeletedAt")]
    pub deleted_at: String,
    #[serde(rename = "ID")]
    pub id: i32,
    #[serde(rename = "UpdatedAt")]
    pub updated_at: String,
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cloud_scopes", skip_serializing_if = "Option::is_none")]
    pub cloud_scopes: Option<Vec<crate::models::DomainPeriodCloudScope>>,
    #[serde(rename = "cspm_enabled")]
    pub cspm_enabled: bool,
    /// GCP Display Name
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "environment", skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,
    /// GCP folder ID
    #[serde(rename = "folder_id", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// GCP folder Name
    #[serde(rename = "folder_name", skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    /// Permissions status returned via API.
    #[serde(rename = "gcp_permissions_status")]
    pub gcp_permissions_status: Vec<crate::models::DomainPeriodPermission>,
    /// GCP organization ID
    #[serde(rename = "organization_id", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// GCP organization name
    #[serde(rename = "organization_name", skip_serializing_if = "Option::is_none")]
    pub organization_name: Option<String>,
    /// GCP Account ID for organization/folder/projects.
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    /// GCP Parent Type.
    #[serde(rename = "parent_type", skip_serializing_if = "Option::is_none")]
    pub parent_type: Option<String>,
    /// GCP Project ID
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    #[serde(
        rename = "service_account_client_email",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_client_email: Option<String>,
    #[serde(
        rename = "service_account_client_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_client_id: Option<String>,
    /// GCP service account ID
    #[serde(rename = "service_account_id", skip_serializing_if = "Option::is_none")]
    pub service_account_id: Option<i32>,
    #[serde(
        rename = "service_account_private_key_id",
        skip_serializing_if = "Option::is_none"
    )]
    pub service_account_private_key_id: Option<String>,
    /// Account registration status.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl DomainPeriodGcpAccountV1 {
    pub fn new(
        created_at: String,
        deleted_at: String,
        id: i32,
        updated_at: String,
        cid: String,
        cspm_enabled: bool,
        gcp_permissions_status: Vec<crate::models::DomainPeriodPermission>,
        parent_id: String,
    ) -> DomainPeriodGcpAccountV1 {
        DomainPeriodGcpAccountV1 {
            created_at,
            deleted_at,
            id,
            updated_at,
            cid,
            cloud_scopes: None,
            cspm_enabled,
            display_name: None,
            environment: None,
            folder_id: None,
            folder_name: None,
            gcp_permissions_status,
            organization_id: None,
            organization_name: None,
            parent_id,
            parent_type: None,
            project_id: None,
            service_account_client_email: None,
            service_account_client_id: None,
            service_account_id: None,
            service_account_private_key_id: None,
            status: None,
        }
    }
}
