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
pub struct DomainPeriodUserGrants {
    #[serde(rename = "cid")]
    pub cid: String,
    #[serde(rename = "cid_group_id", skip_serializing_if = "Option::is_none")]
    pub cid_group_id: Option<String>,
    #[serde(rename = "cid_group_name", skip_serializing_if = "Option::is_none")]
    pub cid_group_name: Option<String>,
    #[serde(rename = "grant_type")]
    pub grant_type: String,
    #[serde(rename = "parent_cid", skip_serializing_if = "Option::is_none")]
    pub parent_cid: Option<String>,
    #[serde(rename = "role_id")]
    pub role_id: String,
    #[serde(rename = "role_name")]
    pub role_name: String,
    #[serde(rename = "user_group_id", skip_serializing_if = "Option::is_none")]
    pub user_group_id: Option<String>,
    #[serde(rename = "user_group_name", skip_serializing_if = "Option::is_none")]
    pub user_group_name: Option<String>,
    #[serde(rename = "uuid")]
    pub uuid: String,
}

impl DomainPeriodUserGrants {
    pub fn new(
        cid: String,
        grant_type: String,
        role_id: String,
        role_name: String,
        uuid: String,
    ) -> DomainPeriodUserGrants {
        DomainPeriodUserGrants {
            cid,
            cid_group_id: None,
            cid_group_name: None,
            grant_type,
            parent_cid: None,
            role_id,
            role_name,
            user_group_id: None,
            user_group_name: None,
            uuid,
        }
    }
}
