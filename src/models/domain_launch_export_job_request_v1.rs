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
pub struct DomainLaunchExportJobRequestV1 {
    /// The entity type. This can be one of: [notification-exposed-data-record]
    #[serde(rename = "entity")]
    pub entity: String,
    /// The file type of the export. This can be one of: [json csv]
    #[serde(rename = "export_type")]
    pub export_type: String,
    /// FQL query to filter entities by. Possible filter properties depend on the entity type.
    #[serde(rename = "filter")]
    pub filter: String,
    /// If set to true (default), the field names in the exported file will resemble the table header in the UI (e.g. \"Hash type\"), otherwise the API level field names will be used (e.g. \"hash_type\")
    #[serde(rename = "human_readable")]
    pub human_readable: bool,
    /// Possible order by fields: created_timestamp, last_updated_timestamp. Ex: 'last_updated_timestamp|desc'
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

impl DomainLaunchExportJobRequestV1 {
    pub fn new(
        entity: String,
        export_type: String,
        filter: String,
        human_readable: bool,
    ) -> DomainLaunchExportJobRequestV1 {
        DomainLaunchExportJobRequestV1 {
            entity,
            export_type,
            filter,
            human_readable,
            sort: None,
        }
    }
}
