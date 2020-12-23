/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use self::DeploymentMode::*;
use self::Precedence::*;
use crate::utilities::*;

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
        let name_parts: Vec<&str> = info.name.split('-').collect();
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
            mode: Unknown(String::from("Unknown")),
            expiry_date: String::from("Unknown"),
            install_datetime: info.mod_date.to_string(),
        }
    }

    fn from_preconditioning_data(data: &JsonMap) -> OperatingConfig {
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

    #[allow(clippy::expect_fun_call)]
    fn update_from_license_data(&mut self, data: &JsonMap) {
        let panic_str =
            |msg: &str| format!("License {} is invalid: {}.", self.filename, msg);
        let payload = data["payload"]
            .as_str()
            .expect(&panic_str("no payload data"));
        let payload = json_from_base64(payload);
        let mode_string = payload["deploymentMode"]
            .as_str()
            .expect(&panic_str("no deployment mode"));
        self.mode = match mode_string {
            "SDL" => Sdl,
            "FRL_CONNECTED" => {
                let server = payload["profileServerUrl"]
                    .as_str()
                    .unwrap_or("http://lcs-cops.adobe.io")
                    .to_string();
                FrlOnline(server)
            }
            "FRL_LAN" => {
                let server = payload["profileServerUrl"]
                    .as_str()
                    .expect(&panic_str("no server specified"))
                    .to_string();
                FrlLAN(server)
            }
            "FRL_ISOLATED" => {
                let values = payload["asnpData"]["customerCertSignedValues"]["values"]
                    .as_str()
                    .expect(&panic_str("no customer binding found"));
                let values = json_from_base64(values);
                let codes: Vec<String> =
                    serde_json::from_value(values["challengeCodes"].clone())
                        .expect(&panic_str("no machine binding found"));
                let code0 = codes.get(0).expect(&panic_str("no census codes found"));
                if code0.len() > 18 {
                    FrlIsolated(Vec::new())
                } else {
                    FrlIsolated(codes)
                }
            }
            s => Unknown(String::from(s)),
        };
        if let Some(expiry_timestamp) = payload["asnpData"]["adobeCertSignedValues"]
            ["values"]["licenseExpiryTimestamp"]
            .as_str()
        {
            self.expiry_date = match self.mode {
                FrlIsolated(_) => date_from_epoch_millis(expiry_timestamp),
                _ => "controlled by server".to_string(),
            };
        } else {
            self.expiry_date = "controlled by server".into();
        }
    }

    pub fn preconditioning_file_configs(info: &FileInfo) -> Vec<OperatingConfig> {
        let data = json_from_file(&info);
        let oc_vec: Vec<JsonMap> =
            serde_json::from_value(data["operatingConfigs"].clone())
                .unwrap_or_else(|_| [].into());
        let mut result: Vec<OperatingConfig> = Vec::new();
        for oc_data in oc_vec {
            result.push(OperatingConfig::from_preconditioning_data(&oc_data))
        }
        result.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        result
    }
}

pub enum DeploymentMode {
    FrlOnline(String),
    FrlIsolated(Vec<String>),
    FrlLAN(String),
    Sdl,
    Unknown(String),
}

impl std::fmt::Display for DeploymentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrlOnline(server) => format!("FRL Online (server: {})", server).fmt(f),
            FrlIsolated(codes) => match codes.len() {
                0 => "FRL Offline".fmt(f),
                1 => "FRL Isolated (1 code)".fmt(f),
                n => format!("FRL Isolated ({} codes)", n).fmt(f),
            },
            FrlLAN(server) => format!("FRL LAN (server: {})", server).fmt(f),
            Sdl => "SDL".fmt(f),
            Unknown(s) => s.fmt(f),
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
