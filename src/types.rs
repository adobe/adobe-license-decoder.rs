use self::DeploymentMode::*;
use self::Precedence::*;
use crate::utilities::*;

use json::JsonValue;

pub struct OperatingConfig {
    pub filename: String,
    pub app_id: String,
    pub cert_group_id: String,
    pub npd_id: String,
    pub package_id: String,
    pub precedence: Precedence,
    pub mode: DeploymentMode,
    pub expiry_date: String,
    pub install_datetime: String,
}

impl OperatingConfig {
    fn from_file_info(info: &FileInfo) -> OperatingConfig {
        let name_parts: Vec<&str> = info.name.split("-").collect();
        let app_part = u64decode(&name_parts[0]);
        let app_info: Vec<&str> = app_part.split("{}").collect();
        let package_id = u64decode(&name_parts[1]);
        let precedence = Precedence::from(&name_parts[2]);
        OperatingConfig {
            filename: info.filename.to_string(),
            app_id: app_info[0].into(),
            cert_group_id: app_info[1].to_string(),
            npd_id: name_parts[1].to_string(),
            package_id,
            precedence,
            mode: DeploymentMode::from("Unknown"),
            expiry_date: String::from("Unknown"),
            install_datetime: info.mod_date.to_string(),
        }
    }

    fn from_preconditioning_data(data: &JsonValue) -> OperatingConfig {
        let info = FileInfo::from_name_and_extension(
            data["name"].as_str().unwrap(),
            data["extension"].as_str().unwrap(),
        );
        let mut result = OperatingConfig::from_file_info(&info);
        if let Some(content) = data["content"].as_str() {
            let content = json_from_base64(content);
            result.update_from_license_data(&content);
        }
        result
    }

    pub fn from_license_file(info: &FileInfo) -> OperatingConfig {
        let mut result = OperatingConfig::from_file_info(&info);
        let data = json_from_file(&info);
        result.update_from_license_data(&data);
        result
    }

    fn update_from_license_data(&mut self, data: &JsonValue) {
        if let Some(payload) = data["payload"].as_str() {
            let payload = json_from_base64(payload);
            if let Some(mode) = payload["deploymentMode"].as_str() {
                self.mode = DeploymentMode::from(mode);
            }
            if let Some(expiry_timestamp) = payload["asnpData"]["adobeCertSignedValues"]
                ["values"]["licenseExpiryTimestamp"]
                .as_str()
            {
                self.expiry_date = match self.mode {
                    FrlIsolated | FrlLAN => date_from_epoch_millis(expiry_timestamp),
                    _ => "controlled by server".into(),
                };
            } else {
                self.expiry_date = "controlled by server".into();
            }
        }
    }

    pub fn preconditioning_file_configs(info: &FileInfo) -> Vec<OperatingConfig> {
        let data = json_from_file(&info);
        let mut result = Vec::new();
        for oc_data in data["operatingConfigs"].members() {
            result.push(OperatingConfig::from_preconditioning_data(oc_data))
        }
        result
    }
}

pub enum DeploymentMode {
    FrlConnected,
    FrlIsolated,
    FrlLAN,
    Sdl,
    Unknown(String),
}

impl std::fmt::Display for DeploymentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrlConnected => "FRL Online/Connected".fmt(f),
            FrlIsolated => "FRL Offline/Isolated".fmt(f),
            FrlLAN => "FRL LAN".fmt(f),
            Sdl => "SDL".fmt(f),
            Unknown(s) => s.fmt(f),
        }
    }
}

impl DeploymentMode {
    pub fn from(s: &str) -> DeploymentMode {
        match s {
            "FRL_CONNECTED" => FrlConnected,
            "FRL_ISOLATED" => FrlIsolated,
            "FRL_LAN" => FrlLAN,
            "SDL" => Sdl,
            s => Unknown(String::from(s)),
        }
    }
}

pub enum Precedence {
    AcrobatStandard = 70,
    AcrobatPro = 100,
    CCSingleApp = 80,
    CCAllApps = 90,
}

impl std::fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AcrobatStandard => "70 (Acrobat Standard)".fmt(f),
            AcrobatPro => "100 (Acrobat Pro)".fmt(f),
            CCSingleApp => "80 (CC Single App)".fmt(f),
            CCAllApps => "90 (CC All Apps)".fmt(f),
        }
    }
}

impl Precedence {
    pub fn from(s: &str) -> Precedence {
        match s {
            "70" => AcrobatStandard,
            "100" => AcrobatPro,
            "80" => CCSingleApp,
            "90" => CCAllApps,
            _ => panic!("Precedence ({}) must be 70, 80, 90, or 100", s),
        }
    }
}
