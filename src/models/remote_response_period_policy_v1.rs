/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and examples, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation).     To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`.    Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

/// RemoteResponsePeriodPolicyV1 : A fully formed RTR policy

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RemoteResponsePeriodPolicyV1 {
    /// The customer id associated with the policy
    #[serde(rename = "cid")]
    pub cid: String,
    /// The email of the user which created the policy
    #[serde(rename = "created_by")]
    pub created_by: String,
    /// The time at which the policy was created
    #[serde(rename = "created_timestamp")]
    pub created_timestamp: String,
    /// The description of a policy. Use this field to provide a high level summary of what this policy enforces
    #[serde(rename = "description")]
    pub description: String,
    /// If a policy is enabled it will be used during the course of policy evaluation
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// The groups that are currently attached to the policy
    #[serde(rename = "groups")]
    pub groups: Vec<crate::models::HostGroupsPeriodHostGroupV1>,
    /// The unique id of the policy
    #[serde(rename = "id")]
    pub id: String,
    /// The email of the user which last modified the policy
    #[serde(rename = "modified_by")]
    pub modified_by: String,
    /// The time at which the policy was last modified
    #[serde(rename = "modified_timestamp")]
    pub modified_timestamp: String,
    /// The human readable name of the policy
    #[serde(rename = "name")]
    pub name: String,
    /// The name of the platform
    #[serde(rename = "platform_name")]
    pub platform_name: PlatformName,
    /// A category of settings in a RTR policy
    #[serde(rename = "settings")]
    pub settings: Vec<crate::models::PreventionPeriodCategoryRespV1>,
}

impl RemoteResponsePeriodPolicyV1 {
    /// A fully formed RTR policy
    pub fn new(
        cid: String,
        created_by: String,
        created_timestamp: String,
        description: String,
        enabled: bool,
        groups: Vec<crate::models::HostGroupsPeriodHostGroupV1>,
        id: String,
        modified_by: String,
        modified_timestamp: String,
        name: String,
        platform_name: PlatformName,
        settings: Vec<crate::models::PreventionPeriodCategoryRespV1>,
    ) -> RemoteResponsePeriodPolicyV1 {
        RemoteResponsePeriodPolicyV1 {
            cid,
            created_by,
            created_timestamp,
            description,
            enabled,
            groups,
            id,
            modified_by,
            modified_timestamp,
            name,
            platform_name,
            settings,
        }
    }
}

/// The name of the platform
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PlatformName {
    #[serde(rename = "Windows")]
    Windows,
    #[serde(rename = "Mac")]
    Mac,
    #[serde(rename = "Linux")]
    Linux,
}

impl Default for PlatformName {
    fn default() -> PlatformName {
        Self::Windows
    }
}
