/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use num_traits::identities::Zero;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivationBody {
    pub npd_id: String,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub npd_precedence: i32,
    pub asnp_template_id: String,
    pub app_details: AppDetails,
    pub device_details: DeviceDetails,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDetails {
    pub ngl_app_id: String,
    pub ngl_app_version: String,
    pub ngl_lib_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_asnp_id: Option<String>,
    pub locale: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDetails {
    pub device_id: String,
    pub os_name: String, // WINDOWS_32, WINDOWS_64, UWP, MAC, IOS, ANDROID
    pub os_version: String,
    pub current_date: String, // yyyy-MM-ddTHH:mm:ss.SSSZ
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub current_timestamp: u64, // epoch millis
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os_user_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_vdi_marker_exists: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_virtual_environment: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_os_user_account: Option<bool>,
}
