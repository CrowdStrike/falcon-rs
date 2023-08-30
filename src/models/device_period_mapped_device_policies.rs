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
pub struct DevicePeriodMappedDevicePolicies {
    #[serde(rename = "airlock", skip_serializing_if = "Option::is_none")]
    pub airlock: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "automox", skip_serializing_if = "Option::is_none")]
    pub automox: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(
        rename = "aws-verified-access",
        skip_serializing_if = "Option::is_none"
    )]
    pub aws_verified_access: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "data-protection", skip_serializing_if = "Option::is_none")]
    pub data_protection: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "device_control", skip_serializing_if = "Option::is_none")]
    pub device_control: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "fim", skip_serializing_if = "Option::is_none")]
    pub fim: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "firewall", skip_serializing_if = "Option::is_none")]
    pub firewall: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "global_config", skip_serializing_if = "Option::is_none")]
    pub global_config: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(
        rename = "identity-protection",
        skip_serializing_if = "Option::is_none"
    )]
    pub identity_protection: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "jumpcloud", skip_serializing_if = "Option::is_none")]
    pub jumpcloud: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(
        rename = "kubernetes-admission-control",
        skip_serializing_if = "Option::is_none"
    )]
    pub kubernetes_admission_control: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "mobile", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "netskope", skip_serializing_if = "Option::is_none")]
    pub netskope: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "prevention", skip_serializing_if = "Option::is_none")]
    pub prevention: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "remote_response", skip_serializing_if = "Option::is_none")]
    pub remote_response: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "sca", skip_serializing_if = "Option::is_none")]
    pub sca: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "sensor_update", skip_serializing_if = "Option::is_none")]
    pub sensor_update: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "system-tray", skip_serializing_if = "Option::is_none")]
    pub system_tray: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
    #[serde(rename = "ztl", skip_serializing_if = "Option::is_none")]
    pub ztl: Option<Box<crate::models::DevicePeriodDevicePolicy>>,
}

impl DevicePeriodMappedDevicePolicies {
    pub fn new() -> DevicePeriodMappedDevicePolicies {
        DevicePeriodMappedDevicePolicies {
            airlock: None,
            automox: None,
            aws_verified_access: None,
            data_protection: None,
            device_control: None,
            fim: None,
            firewall: None,
            global_config: None,
            identity_protection: None,
            jumpcloud: None,
            kubernetes_admission_control: None,
            mobile: None,
            netskope: None,
            prevention: None,
            remote_response: None,
            sca: None,
            sensor_update: None,
            system_tray: None,
            ztl: None,
        }
    }
}
