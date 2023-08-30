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
pub struct DomainPeriodAzureRoleAssignment {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role_definition_id", skip_serializing_if = "Option::is_none")]
    pub role_definition_id: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "subscription_id", skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
}

impl DomainPeriodAzureRoleAssignment {
    pub fn new(status: String) -> DomainPeriodAzureRoleAssignment {
        DomainPeriodAzureRoleAssignment {
            name: None,
            role_definition_id: None,
            status,
            subscription_id: None,
        }
    }
}
