/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: rolling
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ClientArchiveWithFilesV1 {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(rename = "mime_type")]
    pub mime_type: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "sha256", skip_serializing_if = "Option::is_none")]
    pub sha256: Option<String>,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "upload_timestamp")]
    pub upload_timestamp: String,
}

impl ClientArchiveWithFilesV1 {
    pub fn new(
        mime_type: String,
        name: String,
        size: i64,
        status: String,
        upload_timestamp: String,
    ) -> ClientArchiveWithFilesV1 {
        ClientArchiveWithFilesV1 {
            error: None,
            files: None,
            mime_type,
            name,
            sha256: None,
            size,
            status,
            upload_timestamp,
        }
    }
}
