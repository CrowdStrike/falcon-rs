/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

/// RequestsSetPolicyPrecedenceReqV1 : Sets the precedence order for policies of a given platform

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct RequestsSetPolicyPrecedenceReqV1 {
    /// The ids of all current prevention policies for the platform specified. The precedence will be set in the order the ids are specified
    #[serde(rename = "ids")]
    pub ids: Vec<String>,
    /// The name of the platform for which to set precedence
    #[serde(rename = "platform_name")]
    pub platform_name: PlatformName,
}

impl RequestsSetPolicyPrecedenceReqV1 {
    /// Sets the precedence order for policies of a given platform
    pub fn new(ids: Vec<String>, platform_name: PlatformName) -> RequestsSetPolicyPrecedenceReqV1 {
        RequestsSetPolicyPrecedenceReqV1 { ids, platform_name }
    }
}

/// The name of the platform for which to set precedence
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
