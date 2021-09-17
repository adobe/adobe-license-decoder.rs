/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConfig {
    pub oc_spec_version: String,
    pub signatures: Vec<SignatureSpecifier>,
    #[serde(with = "crate::utilities::base64_encoded_json")]
    pub payload: OcPayload,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OcPayload {
    pub id: String,
    pub npd_id: String,
    pub ngl_app_id: String,
    pub npd_precedence: i32,
    pub asnp_data: AsnpData,
    pub profile_server_url: String,
    pub profile_request_payload_params: Option<ProfileRequestPayloadParams>,
    pub deployment_mode: String,
    pub branding: BrandingData,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AsnpData {
    pub template_id: String,
    pub customer_cert_headers: Vec<SignatureSpecifier>,
    pub adobe_cert_signed_values: Option<AdobeSignedValues>,
    pub customer_cert_signed_values: Option<CustomerSignedValues>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrandingData {
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileRequestPayloadParams {
    pub device_params: Vec<String>,
    pub app_params: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdobeSignedValues {
    pub signatures: AdobeSignatures,
    // #[serde(with = "crate::utilities::base64_encoded_json")]
    pub values: AdobeValues,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSignedValues {
    pub signatures: CustomerSignatures,
    #[serde(with = "crate::utilities::base64_encoded_json")]
    pub values: CustomerValues,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdobeSignatures {
    pub signature1: String,
    pub signature2: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdobeValues {
    pub license_expiry_timestamp: String,
    pub enigma_data: String,
    pub grace_time: String,
    pub profile_status: String,
    pub effective_end_timestamp: String,
    pub license_expiry_warning_start_timestamp: String,
    pub ngl_lib_refresh_interval: String,
    pub license_id: String,
    pub licensed_features: String,
    pub app_refresh_interval: String,
    pub app_entitlement_status: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerSignatures {
    pub customer_signature2: String,
    pub customer_signature1: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomerValues {
    pub npd_id: String,
    pub asnp_id: String,
    pub creation_timestamp: u64,
    pub cache_lifetime: u64,
    pub response_type: String,
    pub cache_expiry_warning_control: CacheExpiryWarningControl,
    pub challenge_codes: Vec<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheExpiryWarningControl {
    pub warning_start_timestamp: u64,
    pub warning_interval: u64,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureSpecifier {
    #[serde(with = "crate::utilities::base64_encoded_json")]
    pub header: SignatureHeaderData,
    pub signature: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignatureHeaderData {
    pub content_signature_alg: String,
    pub trusted_cert_fingerprint_alg: String,
    pub trusted_cert_fingerprint_index: i32,
    pub certificate_details: Vec<CertificateDetails>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CertificateDetails {
    pub id: String,
    pub subject_name: String,
    pub hex_serial_number: String,
    pub sha1_hash: String,
    pub sequence: i32,
    pub download_path: String,
}

#[cfg(test)]
mod tests {
    use super::OperatingConfig;
    use crate::client::types::{FileInfo, OperatingConfig as ManualOperatingConfig};

    #[test]
    fn test_online() {
        let path = "rsrc/OperatingConfigs/UGhvdG9zaG9wMXt9MjAxODA3MjAwNA-ODU0YjU5OGQtOTE1Ni00NDZiLWFlZDYtMGQ1ZGM2ZmVhZDBi-80.operatingconfig";
        let info = FileInfo::from_path(path).expect("Can't find online test data");
        let json =
            std::fs::read_to_string(&info.pathname).expect("Can't read online data file");
        let oc1: OperatingConfig =
            serde_json::from_str(&json).expect("Can't parse online data");
        let oc2 = ManualOperatingConfig::from_license_file(&info)
            .expect("Can't manually extract config");
        assert_eq!(oc1.payload.npd_id, oc2.npd_id, "npdIds do not match");
        assert_eq!(oc1.payload.ngl_app_id, oc2.app_id, "appIds do not match");
        println!("test_online complete!");
    }

    #[test]
    fn test_isolated() {
        let path = "rsrc/OperatingConfigs/SWxsdXN0cmF0b3Ixe30yMDE4MDcyMDA0-MmE0N2E4M2UtNjFmNS00NmM2LWE0N2ItOGE0Njc2MTliOTI5-80.operatingconfig";
        let info = FileInfo::from_path(path).expect("Can't find isolated test data");
        let json = std::fs::read_to_string(&info.pathname)
            .expect("Can't read isolated data file");
        let oc1: OperatingConfig =
            serde_json::from_str(&json).expect("Can't parse isolated data");
        let oc2 = ManualOperatingConfig::from_license_file(&info)
            .expect("Can't manually extract config");
        assert_eq!(oc1.payload.npd_id, oc2.npd_id, "npdIds do not match");
        assert_eq!(oc1.payload.ngl_app_id, oc2.app_id, "appIds do not match");
    }
}
