/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use crate::types::{DeploymentMode, OperatingConfig};
use crate::utilities::{shorten_oc_file_name, FileInfo};
use std::cmp::Ordering::Equal;

pub fn describe_directory(info: &FileInfo, verbose: bool) {
    let json_file = format!("{}/ngl-preconditioning-data.json", info.pathname);
    if let Ok(info) = FileInfo::from_path(&json_file) {
        let mut ocs = OperatingConfig::from_preconditioning_file(&info);
        ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        describe_preconditioning_data(&ocs, verbose);
    } else {
        let mut ocs: Vec<OperatingConfig> = Vec::new();
        let pattern = format!("{}/*.operatingconfig", info.pathname);
        for path in glob::glob(&pattern).unwrap() {
            if let Ok(info) = FileInfo::from_path(path.unwrap().to_str().unwrap()) {
                let oc = OperatingConfig::from_license_file(&info);
                ocs.push(oc)
            }
        }
        if ocs.is_empty() {
            panic!("No license files found in '{}'", info.pathname);
        } else {
            ocs.sort_by(|oc1, oc2| match oc1.npd_id.cmp(&oc2.npd_id) {
                Equal => oc1.app_id.cmp(&oc2.app_id),
                otherwise => otherwise,
            });
            describe_operating_configs(&ocs, verbose);
        }
    }
}

pub fn describe_file(info: &FileInfo, verbose: bool) {
    if info.extension.eq_ignore_ascii_case("json") {
        let mut ocs = OperatingConfig::from_preconditioning_file(&info);
        ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        describe_preconditioning_data(&ocs, verbose);
    } else if info.extension.eq_ignore_ascii_case("operatingconfig") {
        let oc = OperatingConfig::from_license_file(&info);
        describe_operating_configs(&vec![oc], verbose);
    } else {
        panic!("Not a license file: '{}'", info.pathname)
    }
}

fn describe_operating_configs(ocs: &[OperatingConfig], verbose: bool) {
    let mut current_npd_id = "";
    for (i, oc) in ocs.iter().enumerate() {
        if !current_npd_id.eq_ignore_ascii_case(&oc.npd_id) {
            current_npd_id = &oc.npd_id;
            println!("License files for npdId: {}:", &oc.npd_id);
            describe_package(oc, verbose);
            println!("Filenames (shown with '...' where the npdId appears):")
        }
        println!("{: >2}: {}", i + 1, shorten_oc_file_name(&oc.filename));
        describe_app(-1, &oc.app_id, &oc.cert_group_id, verbose);
        println!("    Install date: {}", &oc.install_datetime);
    }
}

fn describe_preconditioning_data(ocs: &[OperatingConfig], verbose: bool) {
    for (i, oc) in ocs.iter().enumerate() {
        if i == 0 {
            println!("Preconditioning data for npdId: {}", &oc.npd_id);
            describe_package(oc, verbose);
            println!("Application Licenses:")
        }
        describe_app(i as i32, &oc.app_id, &oc.cert_group_id, verbose);
    }
}

fn describe_package(oc: &OperatingConfig, verbose: bool) {
    if verbose {
        println!("    Package UUID: {}", &oc.package_id);
    }
    println!("    License type: {}", &oc.mode);
    if verbose {
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

fn describe_app(count: i32, app_id: &str, group_id: &str, verbose: bool) {
    println!(
        "{}App ID: {}{}",
        if count < 0 {
            String::from("    ")
        } else {
            format!("{: >2}: ", count + 1)
        },
        app_id,
        if verbose {
            format!(", Certificate Group: {}", group_id)
        } else {
            String::new()
        }
    );
}
