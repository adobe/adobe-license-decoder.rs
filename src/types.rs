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
use eyre::{eyre, Result, WrapErr};
use std::io::Read;
use std::str::from_utf8;

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
    fn from_file_info(info: &FileInfo) -> Result<OperatingConfig> {
        let err = || eyre!("Invalid license file name: {}", info.name);
        let name_parts: Vec<&str> = info.name.split('-').collect();
        if name_parts.len() < 3 {
            return Err(eyre!("Invalid license file name format: {}", info.name));
        }
        let npd_id = name_parts[1].to_string();
        let precedence = Precedence::from(name_parts[2]).wrap_err_with(err)?;
        let app_part = u64decode(name_parts[0]).wrap_err_with(err)?;
        let package_id = u64decode(&npd_id).wrap_err_with(err)?;
        let app_info: Vec<&str> = app_part.split("{}").collect();
        if app_info.len() < 2 {
            return Err(eyre!("Invalid license file initial section: {}", info.name));
        }
        let app_id = app_info[0].to_string();
        let cert_group_id = app_info[1].to_string();
        Ok(OperatingConfig {
            filename: info.filename.to_string(),
            app_id,
            cert_group_id,
            npd_id,
            package_id,
            precedence,
            mode: Unknown(String::from("Unknown")),
            expiry_date: String::from("Unknown"),
            install_datetime: info.mod_date.to_string(),
        })
    }

    fn from_preconditioning_data(data: &JsonMap) -> Result<OperatingConfig> {
        let err = || eyre!("Invalid preconditioning data: bad license file name");
        let info = FileInfo::from_name_and_extension(
            data["name"].as_str().ok_or_else(err)?,
            data["extension"].as_str().ok_or_else(err)?,
        )?;
        let err = || eyre!("Invalid preconditioning data: bad license content");
        let mut result = OperatingConfig::from_file_info(&info)?;
        let content = data["content"].as_str().ok_or_else(err)?;
        let content = json_from_base64(content).wrap_err_with(err)?;
        result.update_from_license_data(&content)?;
        Ok(result)
    }

    pub fn from_license_file(info: &FileInfo) -> Result<OperatingConfig> {
        let mut result = OperatingConfig::from_file_info(&info)?;
        let data = json_from_file(&info)?;
        result.update_from_license_data(&data)?;
        Ok(result)
    }

    fn update_from_license_data(&mut self, data: &JsonMap) -> Result<()> {
        let err = || eyre!("License data is invalid: {}", self.filename);
        let payload = data["payload"].as_str().ok_or_else(err)?;
        let payload = json_from_base64(payload)?;
        let mode_string = payload["deploymentMode"].as_str().ok_or_else(err)?;
        self.mode = match mode_string {
            "NAMED_USER_EDUCATION_LAB" => Sdl,
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
                    .ok_or_else(err)?
                    .to_string();
                FrlLan(server)
            }
            "FRL_ISOLATED" => {
                let values = payload["asnpData"]["customerCertSignedValues"]["values"]
                    .as_str()
                    .ok_or_else(err)?;
                let values = json_from_base64(values)?;
                let codes: Vec<String> =
                    serde_json::from_value(values["challengeCodes"].clone())?;
                let code0 = codes.get(0).ok_or_else(err)?;
                if code0.len() > 18 {
                    FrlOffline
                } else {
                    let codes = codes
                        .iter()
                        .map(|code| {
                            if code.len() != 18 {
                                "invalid-census-code".to_string()
                            } else {
                                format!(
                                    "{}-{}-{}",
                                    &code[0..6],
                                    &code[6..12],
                                    &code[12..18]
                                )
                            }
                        })
                        .collect();
                    FrlIsolated(codes)
                }
            }
            s => Unknown(s.to_string()),
        };
        if let Some(expiry_timestamp) = payload["asnpData"]["adobeCertSignedValues"]
            ["values"]["licenseExpiryTimestamp"]
            .as_str()
        {
            self.expiry_date = date_from_epoch_millis(expiry_timestamp)?;
        } else if let Ok(expiry_timestamp) = self.get_cached_expiry() {
            self.expiry_date = date_from_epoch_millis(&expiry_timestamp)?;
        } else {
            self.expiry_date = "controlled by server".to_string();
        }
        Ok(())
    }

    fn from_preconditioning_json(data: &JsonMap) -> Result<Vec<OperatingConfig>> {
        let oc_vec: Vec<JsonMap> =
            serde_json::from_value(data["operatingConfigs"].clone())?;
        let mut result: Vec<OperatingConfig> = Vec::new();
        for oc_data in oc_vec {
            result.push(OperatingConfig::from_preconditioning_data(&oc_data)?)
        }
        result.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        Ok(result)
    }

    pub fn from_preconditioning_file(info: &FileInfo) -> Result<Vec<OperatingConfig>> {
        let data = json_from_file(&info)?;
        OperatingConfig::from_preconditioning_json(&data)
    }

    pub fn from_ccp_file(info: &FileInfo) -> Result<Vec<OperatingConfig>> {
        let bytes = std::fs::read(&info.pathname).wrap_err("Cannot read ccp file")?;
        // on Windows, this may be a zip file, and we need to extract
        // the PkgConfig.xml file from it
        let reader = std::io::Cursor::new(&bytes);
        let html = if let Ok(mut archive) = zip::ZipArchive::new(reader) {
            let mut file = archive
                .by_name("PkgConfig.xml")
                .map_err(|e| eyre!(e))
                .wrap_err("Can't find configuration data in ccp archive")?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)
                .wrap_err("Can't read configuration data from ccp archive")?;
            buffer
        } else {
            from_utf8(&bytes)
                .wrap_err("Invalid ccp file format")?
                .to_string()
        };
        let doc = visdom::Vis::load(&html)
            .map_err(|e| eyre!("{}", e))
            .wrap_err("Cannot parse ccp file")?;
        let data_node = doc.find("Preconditioning");
        let data = data_node.text();
        let data: JsonMap = serde_json::from_str(data)
            .wrap_err("Can't parse preconditioning data in ccp file")?;
        OperatingConfig::from_preconditioning_json(&data)
    }

    pub fn get_cached_expiry(&self) -> Result<String> {
        let err = || eyre!("Malformed license");
        let app_name = self.app_id.as_str();
        // adjust the cert name to end with 03 because apps always use that cert group
        let cert_name =
            format!("{}03", &self.cert_group_id[..self.cert_group_id.len() - 2]);
        let note_key = u64encode(&format!("{}{{}}{}", app_name, &cert_name))?;
        let note = get_saved_credential(&note_key)?;
        let json = json_from_str(&note)?;
        let asnp = json["asnp"].as_str().ok_or_else(err)?;
        let json = json_from_str(asnp)?;
        let payload = json["payload"].as_str().ok_or_else(err)?;
        let json = json_from_base64(payload)?;
        let legacy_profile = json["legacyProfile"].as_str().ok_or_else(err)?;
        let json = json_from_str(legacy_profile)?;
        let timestamp = json["effectiveEndTimestamp"].as_i64().ok_or_else(err)?;
        Ok(timestamp.to_string())
    }
}

#[cfg(target_os = "macos")]
fn get_saved_credential(key: &str) -> Result<String> {
    let service = format!("Adobe App Info ({})", &key);
    let keyring = keyring::Keyring::new(&service, "App Info");
    keyring.get_password().map_err(|e| eyre!(e))
}

#[cfg(target_os = "windows")]
fn get_saved_credential(key: &str) -> Result<String> {
    let mut result = String::new();
    for i in 1..100 {
        let service = format!("Adobe App Info ({})(Part{})", &key, i);
        let keyring = keyring::Keyring::new(&service, "App Info");
        if let Ok(note) = keyring.get_password() {
            result.push_str(note.trim());
        } else {
            break;
        }
    }
    if result.is_empty() {
        Err(eyre!("No credential data found"))
    } else {
        Ok(result)
    }
}

pub enum DeploymentMode {
    FrlOnline(String),
    FrlOffline,
    FrlIsolated(Vec<String>),
    FrlLan(String),
    Sdl,
    Unknown(String),
}

impl std::fmt::Display for DeploymentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FrlOnline(server) => format!("FRL Online (server: {})", server).fmt(f),
            FrlOffline => "FRL Offline".fmt(f),
            FrlIsolated(codes) => match codes.len() {
                1 => "FRL Isolated (1 census code)".fmt(f),
                n => format!("FRL Isolated ({} census codes)", n).fmt(f),
            },
            FrlLan(server) => format!("FRL LAN (server: {})", server).fmt(f),
            Sdl => "SDL".fmt(f),
            Unknown(s) => s.fmt(f),
        }
    }
}

pub enum Precedence {
    AcrobatStandard = 70,
    AcrobatPro = 100,
    CcSingleApp = 80,
    CcAllApps = 90,
}

impl std::fmt::Display for Precedence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AcrobatStandard => "70 (Acrobat Standard)".fmt(f),
            AcrobatPro => "100 (Acrobat Pro)".fmt(f),
            CcSingleApp => "80 (CC Single App)".fmt(f),
            CcAllApps => "90 (CC All Apps)".fmt(f),
        }
    }
}

impl Precedence {
    pub fn from(s: &str) -> Result<Precedence> {
        match s {
            "70" => Ok(AcrobatStandard),
            "100" => Ok(AcrobatPro),
            "80" => Ok(CcSingleApp),
            "90" => Ok(CcAllApps),
            _ => Err(eyre!("Precedence ({}) must be 70, 80, 90, or 100", s)),
        }
    }
}
