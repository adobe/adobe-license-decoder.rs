/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use chrono::{DateTime, Local};
use serde_json::Value;
use std::collections::HashMap;
use url::Url;

#[derive(Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub struct ActivationBody {
    pub npd_id: String,
    #[serde(default, skip_serializing_if = "Zero::is_zero")]
    pub npd_precedence: i32,
    pub asnp_template_id: String,
    pub app_details: AppDetails,
    pub device_details: DeviceDetails,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
pub struct AppDetails {
    pub ngl_app_id: String,
    pub ngl_app_version: String,
    pub ngl_lib_version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_asnp_id: Option<String>,
    pub locale: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
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

#[derive(Default, Debug, Clone)]
/// The data values found in an FRL Online request.
///
/// Some of these are held in headers, some in URL parameters, and
/// some in the body.  Where they go and which are required depend
/// on whether the request is for activation or deactivation.
///
/// We add our own timestamp so we can sort by it, even if the request
/// does not have a device timestamp attached.
pub struct Request {
    pub kind: Kind,
    pub api_key: String,
    pub request_id: String,
    pub session_id: String,
    pub package_id: String,
    pub asnp_id: String,
    pub device_id: String,
    pub device_date: String,
    pub is_vdi: bool,
    pub is_virtual: bool,
    pub os_name: String,
    pub os_version: String,
    pub os_user_id: String,
    pub is_domain_user: bool,
    pub app_id: String,
    pub app_version: String,
    pub ngl_version: String,
    pub timestamp: String,
}

impl Request {
    /// Create a COPS request from a network request received by the proxy.
    pub fn from_network(parts: &Parts, body: &[u8]) -> Result<Request, BadRequest> {
        match parts.uri.path() {
            ACTIVATION_ENDPOINT => {
                if parts.method == Method::POST {
                    Request::from_activation(parts, body)
                } else {
                    Err(BadRequest::from("Activation method must be POST"))
                }
            }
            DEACTIVATION_ENDPOINT => {
                if parts.method == Method::DELETE {
                    Request::from_deactivation(parts)
                } else {
                    Err(BadRequest::from("Deactivation method must be DELETE"))
                }
            }
            path => {
                let message = format!("Unknown endpoint path: {}", path);
                Err(BadRequest::from(&message))
            }
        }
    }

    /// Create a network request which submits this COPS request to the given server.
    pub fn to_network(&self, scheme: &str, host: &str) -> hyper::Request<Body> {
        match self.kind {
            Kind::Activation => self.to_activation(scheme, host),
            Kind::Deactivation => self.to_deactivation(scheme, host),
        }
    }

    fn from_activation(parts: &Parts, body: &[u8]) -> Result<Request, BadRequest> {
        let mut req = Request {
            kind: Kind::Activation,
            timestamp: current_timestamp(),
            ..Default::default()
        };
        req.update_from_headers(parts)?;
        let map: HashMap<String, Value> =
            serde_json::from_slice(body).unwrap_or_default();
        if map.is_empty() {
            return Err(BadRequest::from("Malformed activation request body"));
        }
        if let Some(package_id) = map["npdId"].as_str() {
            req.package_id = package_id.to_string();
        } else {
            return Err(BadRequest::from("Missing npdId field in request."));
        }
        if let Some(asnp_id) = map["asnpTemplateId"].as_str() {
            req.asnp_id = asnp_id.to_string();
        } else {
            return Err(BadRequest::from("Missing asnpTemplateId field in request."));
        }
        if map.get("appDetails").is_none() {
            return Err(BadRequest::from("Missing appDetails object in request."));
        }
        let app_map: HashMap<String, Value> =
            serde_json::from_value(map["appDetails"].clone()).unwrap_or_default();
        if let Some(app_id) = app_map["nglAppId"].as_str() {
            req.app_id = app_id.to_string();
        } else {
            return Err(BadRequest::from("Missing nglAppId field in request."));
        }
        if let Some(app_version) = app_map["nglAppVersion"].as_str() {
            req.app_version = app_version.to_string();
        } else {
            return Err(BadRequest::from("Missing nglAppVersion field in request."));
        }
        if let Some(ngl_version) = app_map["nglLibVersion"].as_str() {
            req.ngl_version = ngl_version.to_string();
        } else {
            return Err(BadRequest::from("Missing nglLibVersion field in request."));
        }
        if map.get("deviceDetails").is_none() {
            return Err(BadRequest::from("Missing deviceDetails object in request."));
        }
        let device_map: HashMap<String, Value> =
            serde_json::from_value(map["deviceDetails"].clone()).unwrap_or_default();
        if let Some(device_date) = device_map["currentDate"].as_str() {
            req.device_date = device_date.to_string();
        } else {
            return Err(BadRequest::from("Missing currentDate field in request."));
        }
        if let Some(device_id) = device_map["deviceId"].as_str() {
            req.device_id = device_id.to_string();
        } else {
            return Err(BadRequest::from("Missing deviceId field in request."));
        }
        if let Some(os_user_id) = device_map["osUserId"].as_str() {
            req.os_user_id = os_user_id.to_string();
        } else {
            return Err(BadRequest::from("Missing osUserId field in request."));
        }
        if let Some(os_name) = device_map["osName"].as_str() {
            req.os_name = os_name.to_string();
        } else {
            return Err(BadRequest::from("Missing osName field in request."));
        }
        if let Some(os_version) = device_map["osVersion"].as_str() {
            req.os_version = os_version.to_string();
        } else {
            return Err(BadRequest::from("Missing osVersion field in request."));
        }
        if let Some(is_vdi) = device_map["enableVdiMarkerExists"].as_bool() {
            req.is_vdi = is_vdi;
        } else {
            req.is_vdi = false;
        }
        if let Some(is_domain_user) = device_map["isOsUserAccountInDomain"].as_bool() {
            req.is_domain_user = is_domain_user;
        } else {
            req.is_domain_user = false;
        }
        if let Some(is_virtual) = device_map["isVirtualEnvironment"].as_bool() {
            req.is_virtual = is_virtual;
        } else {
            req.is_virtual = false;
        }
        Ok(req)
    }

    fn from_deactivation(parts: &Parts) -> Result<Request, BadRequest> {
        let mut req = Request {
            kind: Kind::Deactivation,
            timestamp: current_timestamp(),
            ..Default::default()
        };
        req.update_from_headers(parts)?;
        let request_url = format!("http://placeholder{}", &parts.uri.to_string());
        let pairs: HashMap<String, String> = Url::parse(&request_url)
            .expect("Bad deactivation query string")
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        if let Some(npd_id) = pairs.get("npdId") {
            req.package_id = npd_id.clone();
        } else {
            return Err(BadRequest::from("Missing 'npdId' parameter"));
        }
        if let Some(device_id) = pairs.get("deviceId") {
            req.device_id = device_id.clone()
        } else {
            return Err(BadRequest::from("Missing 'deviceId' parameter"));
        }
        if let Some(os_user_id) = pairs.get("osUserId") {
            req.os_user_id = os_user_id.clone()
        } else {
            return Err(BadRequest::from("Missing 'osUserId' parameter"));
        }
        if let Some(is_vdi) = pairs.get("enableVdiMarkerExists") {
            req.is_vdi = is_vdi.eq_ignore_ascii_case("true")
        } else {
            req.is_vdi = false
        }
        Ok(req)
    }

    /// Convert a COPS activation request to its network form.
    fn to_activation(&self, scheme: &str, host: &str) -> hyper::Request<Body> {
        let body = serde_json::json!({
            "npdId" : &self.package_id,
            "asnpTemplateId" : &self.asnp_id,
            "appDetails" :  {
                "nglAppId" : &self.app_id,
                "nglAppVersion" : &self.app_version,
                "nglLibVersion" : &self.ngl_version
            },
            "deviceDetails" : {
                "currentDate" : &self.device_date,
                "deviceId" : &self.device_id,
                "enableVdiMarkerExists" : &self.is_vdi,
                "isOsUserAccountInDomain" : &self.is_domain_user,
                "isVirtualEnvironment" : &self.is_virtual,
                "osName" : &self.os_name,
                "osUserId" : &self.os_user_id,
                "osVersion" : &self.os_version
            }
        });
        let builder = hyper::Request::builder()
            .method("POST")
            .uri(format!("{}://{}{}", scheme, host, ACTIVATION_ENDPOINT).as_str())
            .header("host", host)
            .header("x-api-key", &self.api_key)
            .header("x-session-id", &self.session_id)
            .header("x-request-id", &self.request_id)
            .header("content-type", "application/json")
            .header("accept", "application/json")
            .header("user-agent", agent());
        builder
            .body(Body::from(body.to_string()))
            .expect("Error building activation request body")
    }

    /// Convert a COPS deactivation request to its network form.
    fn to_deactivation(&self, scheme: &str, host: &str) -> hyper::Request<Body> {
        let uri = format!(
            "{}://{}{}?npdId={}&deviceId={}&osUserId={}&enableVdiMarkerExists={}",
            scheme,
            host,
            DEACTIVATION_ENDPOINT,
            &self.package_id,
            &self.device_id,
            &self.os_user_id,
            self.is_vdi,
        );
        let builder = hyper::Request::builder()
            .method("DELETE")
            .uri(uri)
            .header("host", host)
            .header("x-api-key", &self.api_key)
            .header("x-request-id", &self.request_id)
            .header("accept", "application/json")
            .header("user-agent", agent());
        builder
            .body(Body::empty())
            .expect("Error building deactivation request body")
    }

    /// update a request with info from network headers
    fn update_from_headers(&mut self, parts: &Parts) -> Result<&mut Request, BadRequest> {
        for (k, v) in parts.headers.iter() {
            if let Ok(val) = v.to_str() {
                match k.as_str() {
                    "x-api-key" => self.api_key = val.to_string(),
                    "x-request-id" => self.request_id = val.to_string(),
                    "x-session-id" => self.session_id = val.to_string(),
                    _ => (),
                }
            }
        }
        match self.kind {
            Kind::Activation => {
                if self.api_key.is_empty()
                    || self.request_id.is_empty()
                    || self.session_id.is_empty()
                {
                    return Err(BadRequest::from("Missing required header field"));
                }
            }
            Kind::Deactivation => {
                if self.api_key.is_empty() || self.request_id.is_empty() {
                    return Err(BadRequest::from("Missing required header field"));
                }
            }
        }
        Ok(self)
    }
}

pub struct Response {
    pub kind: Kind,
    pub request_id: String,
    pub body: Vec<u8>,
    pub timestamp: String,
}

impl Response {
    pub fn from_network(request: &Request, body: &[u8]) -> Response {
        Response {
            kind: request.kind.clone(),
            request_id: request.request_id.clone(),
            body: Vec::from(body),
            timestamp: current_timestamp(),
        }
    }

    pub fn to_network(&self) -> HResponse<Body> {
        HResponse::builder()
            .status(200)
            .header("server", agent())
            .header("x-request-id", self.request_id.clone())
            .header("content-type", "application/json;charset=UTF-8")
            .body(Body::from(self.body.clone()))
            .unwrap()
    }
}

const ACTIVATION_ENDPOINT: &str = "/asnp/frl_connected/values/v2";
const DEACTIVATION_ENDPOINT: &str = "/asnp/frl_connected/v1";

#[derive(Debug, Clone)]
pub enum Kind {
    Activation,
    Deactivation,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Activation => "Activation".fmt(f),
            Kind::Deactivation => "Deactivation".fmt(f),
        }
    }
}

impl Default for Kind {
    fn default() -> Self {
        Kind::Activation
    }
}

#[derive(Debug, Clone)]
pub struct BadRequest {
    pub reason: String,
}

impl BadRequest {
    pub fn from(why: &str) -> BadRequest {
        BadRequest {
            reason: why.to_string(),
        }
    }
}

pub fn agent() -> String {
    format!(
        "FRL-Online-Proxy/{} ({}/{})",
        env!("CARGO_PKG_VERSION"),
        std::env::consts::OS,
        sys_info::os_release().as_deref().unwrap_or("Unknown")
    )
}

pub fn current_timestamp() -> String {
    let now: DateTime<Local> = Local::now();
    now.format("%Y-%m-%dT%H:%M:%S%.3f%z").to_string()
}
