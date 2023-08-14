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
pub struct DomainReportExecutionSummary {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "last_updated_on")]
    pub last_updated_on: String,
    #[serde(rename = "report_file_reference")]
    pub report_file_reference: String,
    #[serde(rename = "result_metadata")]
    pub result_metadata: Box<crate::models::DomainResultMetadata>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "status_msg")]
    pub status_msg: String,
}

impl DomainReportExecutionSummary {
    pub fn new(
        id: String,
        last_updated_on: String,
        report_file_reference: String,
        result_metadata: crate::models::DomainResultMetadata,
        status: String,
        status_msg: String,
    ) -> DomainReportExecutionSummary {
        DomainReportExecutionSummary {
            id,
            last_updated_on,
            report_file_reference,
            result_metadata: Box::new(result_metadata),
            status,
            status_msg,
        }
    }
}
