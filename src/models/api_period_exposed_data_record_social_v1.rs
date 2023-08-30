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
pub struct ApiPeriodExposedDataRecordSocialV1 {
    #[serde(rename = "aim_id", skip_serializing_if = "Option::is_none")]
    pub aim_id: Option<String>,
    #[serde(rename = "facebook_id", skip_serializing_if = "Option::is_none")]
    pub facebook_id: Option<String>,
    #[serde(rename = "icq_id", skip_serializing_if = "Option::is_none")]
    pub icq_id: Option<String>,
    #[serde(rename = "instagram_id", skip_serializing_if = "Option::is_none")]
    pub instagram_id: Option<String>,
    #[serde(rename = "msn_id", skip_serializing_if = "Option::is_none")]
    pub msn_id: Option<String>,
    #[serde(rename = "skype_id", skip_serializing_if = "Option::is_none")]
    pub skype_id: Option<String>,
    #[serde(rename = "twitter_id", skip_serializing_if = "Option::is_none")]
    pub twitter_id: Option<String>,
    #[serde(rename = "vk_id", skip_serializing_if = "Option::is_none")]
    pub vk_id: Option<String>,
    #[serde(rename = "vk_token", skip_serializing_if = "Option::is_none")]
    pub vk_token: Option<String>,
}

impl ApiPeriodExposedDataRecordSocialV1 {
    pub fn new() -> ApiPeriodExposedDataRecordSocialV1 {
        ApiPeriodExposedDataRecordSocialV1 {
            aim_id: None,
            facebook_id: None,
            icq_id: None,
            instagram_id: None,
            msn_id: None,
            skype_id: None,
            twitter_id: None,
            vk_id: None,
            vk_token: None,
        }
    }
}
