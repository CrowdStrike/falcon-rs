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
pub struct FalconxActor {
    #[serde(rename = "created_timestamp", skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "first_activity_timestamp", skip_serializing_if = "Option::is_none")]
    pub first_activity_timestamp: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "image_artifact_id", skip_serializing_if = "Option::is_none")]
    pub image_artifact_id: Option<String>,
    #[serde(rename = "known_as", skip_serializing_if = "Option::is_none")]
    pub known_as: Option<String>,
    #[serde(rename = "last_activity_timestamp", skip_serializing_if = "Option::is_none")]
    pub last_activity_timestamp: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "origins", skip_serializing_if = "Option::is_none")]
    pub origins: Option<Vec<crate::models::FalconxEntity>>,
    #[serde(rename = "short_description", skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "target_countries", skip_serializing_if = "Option::is_none")]
    pub target_countries: Option<Vec<crate::models::FalconxEntity>>,
    #[serde(rename = "target_industries", skip_serializing_if = "Option::is_none")]
    pub target_industries: Option<Vec<crate::models::FalconxEntity>>,
    #[serde(rename = "thumbnail_artifact_id", skip_serializing_if = "Option::is_none")]
    pub thumbnail_artifact_id: Option<String>,
}

impl FalconxActor {
    pub fn new() -> FalconxActor {
        FalconxActor {
            created_timestamp: None,
            description: None,
            first_activity_timestamp: None,
            id: None,
            image_artifact_id: None,
            known_as: None,
            last_activity_timestamp: None,
            name: None,
            origins: None,
            short_description: None,
            slug: None,
            target_countries: None,
            target_industries: None,
            thumbnail_artifact_id: None,
        }
    }
}
