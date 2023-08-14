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
pub struct DomainExposedDataRecordSocialV1 {
    #[serde(rename = "aim_id")]
    pub aim_id: String,
    #[serde(rename = "facebook_id")]
    pub facebook_id: String,
    #[serde(rename = "icq_id")]
    pub icq_id: String,
    #[serde(rename = "instagram_id")]
    pub instagram_id: String,
    #[serde(rename = "msn_id")]
    pub msn_id: String,
    #[serde(rename = "skype_id")]
    pub skype_id: String,
    #[serde(rename = "twitter_id")]
    pub twitter_id: String,
    #[serde(rename = "vk_id")]
    pub vk_id: String,
}

impl DomainExposedDataRecordSocialV1 {
    pub fn new(
        aim_id: String,
        facebook_id: String,
        icq_id: String,
        instagram_id: String,
        msn_id: String,
        skype_id: String,
        twitter_id: String,
        vk_id: String,
    ) -> DomainExposedDataRecordSocialV1 {
        DomainExposedDataRecordSocialV1 {
            aim_id,
            facebook_id,
            icq_id,
            instagram_id,
            msn_id,
            skype_id,
            twitter_id,
            vk_id,
        }
    }
}
