/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2021-10-05T19:33:53Z
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct MalqueryStats {
    /// Number of clean samples
    #[serde(rename = "clean_count")]
    pub clean_count: i32,
    /// Number of malicious samples
    #[serde(rename = "malware_count")]
    pub malware_count: i32,
    /// Number of potentially unwanted samples such as adware
    #[serde(rename = "pua_count")]
    pub pua_count: i32,
    /// Total number of samples
    #[serde(rename = "total_count")]
    pub total_count: i32,
    /// Number of unknown (which could not be classified) samples
    #[serde(rename = "unknown_count")]
    pub unknown_count: i32,
}

impl MalqueryStats {
    pub fn new(clean_count: i32, malware_count: i32, pua_count: i32, total_count: i32, unknown_count: i32) -> MalqueryStats {
        MalqueryStats {
            clean_count,
            malware_count,
            pua_count,
            total_count,
            unknown_count,
        }
    }
}
