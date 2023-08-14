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
pub struct RequestsDeviceControlPolicySettingsV1 {
    /// Settings that apply to a USB Class
    #[serde(rename = "classes")]
    pub classes: Vec<crate::models::RequestsDeviceControlPolicyClassSettingsV1>,
    /// An array of exception IDs to delete from the policy
    #[serde(rename = "delete_exceptions")]
    pub delete_exceptions: Vec<String>,
    /// Does the end user receives a notification when the policy is violated
    #[serde(rename = "end_user_notification")]
    pub end_user_notification: EndUserNotification,
    /// How is this policy enforced
    #[serde(rename = "enforcement_mode")]
    pub enforcement_mode: EnforcementMode,
}

impl RequestsDeviceControlPolicySettingsV1 {
    pub fn new(
        classes: Vec<crate::models::RequestsDeviceControlPolicyClassSettingsV1>,
        delete_exceptions: Vec<String>,
        end_user_notification: EndUserNotification,
        enforcement_mode: EnforcementMode,
    ) -> RequestsDeviceControlPolicySettingsV1 {
        RequestsDeviceControlPolicySettingsV1 {
            classes,
            delete_exceptions,
            end_user_notification,
            enforcement_mode,
        }
    }
}

/// Does the end user receives a notification when the policy is violated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndUserNotification {
    #[serde(rename = "SILENT")]
    SILENT,
    #[serde(rename = "NOTIFY_USER")]
    NOTIFYUSER,
}

impl Default for EndUserNotification {
    fn default() -> EndUserNotification {
        Self::SILENT
    }
}
/// How is this policy enforced
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EnforcementMode {
    #[serde(rename = "MONITOR_ONLY")]
    ONLY,
    #[serde(rename = "MONITOR_ENFORCE")]
    ENFORCE,
}

impl Default for EnforcementMode {
    fn default() -> EnforcementMode {
        Self::ONLY
    }
}
