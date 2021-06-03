/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use super::types::{DeploymentMode, FileInfo, OperatingConfig};
use crate::utilities::{date_from_epoch_millis, shorten_oc_file_name};
use eyre::{eyre, Result};
use std::cmp::Ordering::Equal;

pub fn describe_directory(info: &FileInfo, verbose: i32) -> Result<()> {
    let json_file = format!("{}/ngl-preconditioning-data.json", info.pathname);
    if let Ok(info) = FileInfo::from_path(&json_file) {
        return describe_file(&info, verbose);
    }
    let pattern = format!("{}/*.ccp", info.pathname);
    for path in glob::glob(&pattern).unwrap() {
        if let Ok(info) = FileInfo::from_path(path.unwrap().to_str().unwrap()) {
            return describe_file(&info, verbose);
        }
    }
    let pattern = format!("{}/*.operatingconfig", info.pathname);
    let mut ocs: Vec<OperatingConfig> = Vec::new();
    for path in glob::glob(&pattern).unwrap() {
        if let Ok(info) = FileInfo::from_path(path.unwrap().to_str().unwrap()) {
            let oc = OperatingConfig::from_license_file(&info)?;
            ocs.push(oc)
        }
    }
    if ocs.is_empty() {
        Err(eyre!(
            "No license files found in directory: {}",
            info.pathname
        ))
    } else {
        ocs.sort_by(|oc1, oc2| match oc1.npd_id.cmp(&oc2.npd_id) {
            Equal => oc1.app_id.cmp(&oc2.app_id),
            otherwise => otherwise,
        });
        describe_operating_configs(&ocs, verbose)
    }
}

pub fn describe_file(info: &FileInfo, verbose: i32) -> Result<()> {
    if info.extension.eq_ignore_ascii_case("json") {
        let mut ocs = OperatingConfig::from_preconditioning_file(info)?;
        ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        describe_preconditioning_data(&ocs, verbose);
        Ok(())
    } else if info.extension.eq_ignore_ascii_case("ccp") {
        let mut ocs = OperatingConfig::from_ccp_file(info)?;
        ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        describe_preconditioning_data(&ocs, verbose);
        Ok(())
    } else if info.extension.eq_ignore_ascii_case("operatingconfig") {
        let oc = OperatingConfig::from_license_file(&info)?;
        describe_operating_configs(&vec![oc], verbose)
    } else {
        Err(eyre!("Not a license file: {}", info.pathname))
    }
}

fn describe_operating_configs(ocs: &[OperatingConfig], verbose: i32) -> Result<()> {
    let mut current_npd_id = "";
    for (i, oc) in ocs.iter().enumerate() {
        if !current_npd_id.eq_ignore_ascii_case(&oc.npd_id) {
            current_npd_id = &oc.npd_id;
            println!("License files for npdId: {}:", &oc.npd_id);
            describe_package(oc, verbose);
            println!("Filenames (shown with '...' where the npdId appears):")
        }
        println!("{: >2}: {}", i + 1, shorten_oc_file_name(&oc.filename)?);
        describe_app(-1, &oc.app_id, &oc.cert_group_id, verbose);
        println!("    Install date: {}", &oc.install_datetime);
        // if -vv is given, check for locally cached licenses
        if verbose > 1 {
            if let Ok(date) = oc.get_cached_expiry() {
                println!(
                    "    Cached activation expires: {}",
                    date_from_epoch_millis(&date)?
                )
            } else {
                println!("    No cached activation")
            }
        }
    }
    Ok(())
}

fn describe_preconditioning_data(ocs: &[OperatingConfig], verbose: i32) {
    for (i, oc) in ocs.iter().enumerate() {
        if i == 0 {
            println!("Preconditioning data for npdId: {}", &oc.npd_id);
            describe_package(oc, verbose);
            println!("Application Licenses:")
        }
        describe_app(i as i32, &oc.app_id, &oc.cert_group_id, verbose);
    }
}

fn describe_package(oc: &OperatingConfig, verbose: i32) {
    if verbose > 0 {
        println!("    Package UUID: {}", &oc.package_id);
    }
    println!("    License type: {}", &oc.mode);
    if verbose > 0 {
        if let DeploymentMode::FrlIsolated(codes) = &oc.mode {
            if codes.len() == 1 {
                println!("    Census code: {}", codes[0]);
            } else {
                println!("    Census codes: {}", codes.join(", "));
            }
        }
    }
    println!("    License expiry date: {}", &oc.expiry_date);
    println!("    Precedence: {}", &oc.precedence);
}

fn describe_app(count: i32, app_id: &str, group_id: &str, verbose: i32) {
    println!(
        "{}App ID: {}{}",
        if count < 0 {
            String::from("    ")
        } else {
            format!("{: >2}: ", count + 1)
        },
        app_id,
        if verbose > 0 {
            format!(", Certificate Group: {}", group_id)
        } else {
            String::new()
        }
    );
}
