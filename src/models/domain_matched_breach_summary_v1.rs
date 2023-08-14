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
pub struct DomainMatchedBreachSummaryV1 {
    /// Community/colloquial exposed data event name.
    #[serde(rename = "community_name", skip_serializing_if = "Option::is_none")]
    pub community_name: Option<String>,
    /// The level of confidence regarding data veridicality. Options for likely authentic, confirmed authentic (default: unverified).
    #[serde(rename = "confidence_level", skip_serializing_if = "Option::is_none")]
    pub confidence_level: Option<String>,
    /// The description of the breach
    #[serde(rename = "description")]
    pub description: String,
    /// The date the exposed data event occurred.
    #[serde(rename = "event_date", skip_serializing_if = "Option::is_none")]
    pub event_date: Option<String>,
    /// CrowdStrike-generated unique exposed data event identifier.
    #[serde(rename = "event_id", skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// The date when the data was leaked online
    #[serde(rename = "exposure_date", skip_serializing_if = "Option::is_none")]
    pub exposure_date: Option<String>,
    /// The set of fields which were breached: 'email', 'password', 'login_id', 'phone', etc.
    #[serde(rename = "fields")]
    pub fields: Vec<String>,
    /// Metadata regarding the file(s) where exposed data records where found.
    #[serde(rename = "files", skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<crate::models::DomainFileDetailsV1>>,
    /// Where the exposed data event happened. (e.g. LinkedIn or linkedin[.]com)
    #[serde(rename = "impacted_domains", skip_serializing_if = "Option::is_none")]
    pub impacted_domains: Option<Vec<String>>,
    /// Where the exposed data event happened
    #[serde(rename = "impacted_ips", skip_serializing_if = "Option::is_none")]
    pub impacted_ips: Option<Vec<String>>,
    /// The name of the breach
    #[serde(rename = "name")]
    pub name: String,
    /// Exposed Data Event Threat Actor/Group: Moniker(s) or real name(s) of the individual/group who unveiled confidential data.
    #[serde(rename = "obtained_by", skip_serializing_if = "Option::is_none")]
    pub obtained_by: Option<String>,
    /// Where the leak was found.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DomainMatchedBreachSummaryV1 {
    pub fn new(
        description: String,
        fields: Vec<String>,
        name: String,
    ) -> DomainMatchedBreachSummaryV1 {
        DomainMatchedBreachSummaryV1 {
            community_name: None,
            confidence_level: None,
            description,
            event_date: None,
            event_id: None,
            exposure_date: None,
            fields,
            files: None,
            impacted_domains: None,
            impacted_ips: None,
            name,
            obtained_by: None,
            url: None,
        }
    }
}
