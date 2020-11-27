/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
mod types;
mod utilities;

use std::cmp::Ordering::Equal;
use std::env;
use std::process::exit;
use types::*;
use utilities::*;

const APP_SUPPORT_DIR: &str = if cfg!(target_os = "macos") {
    "/Library/Application Support"
} else if cfg!(target_os = "windows") {
    "${ProgramData}"
} else {
    "This module can only run on MacOS or Windows"
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.len() {
        1 => format!("{}/Adobe/OperatingConfigs", APP_SUPPORT_DIR),
        2 => args[1].to_string(),
        _ => {
            eprintln!("Too many arguments: {:?}", &args[1..]);
            eprintln!("Usage: frl-license-decoder [directory-or-preconditioning-file]");
            exit(1);
        }
    };
    let info = match FileInfo::from_path(&path) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}: {}", e, path);
            exit(1);
        }
    };
    if info.is_directory {
        let mut ocs: Vec<OperatingConfig> = Vec::new();
        let pattern = info.pathname + "/*.operatingconfig";
        for path in glob::glob(&pattern).unwrap() {
            if let Ok(info) = FileInfo::from_path(path.unwrap().to_str().unwrap()) {
                let oc = OperatingConfig::from_license_file(&info);
                ocs.push(oc)
            }
        }
        if ocs.len() == 0 {
            eprintln!("Error: No license files found in '{}'", info.filename);
            exit(1);
        } else {
            ocs.sort_by(|oc1, oc2| match oc1.npd_id.cmp(&oc2.npd_id) {
                Equal => oc1.app_id.cmp(&oc2.app_id),
                otherwise => otherwise,
            });
            describe_operating_configs(&ocs);
        }
    } else if info.extension.eq_ignore_ascii_case("json") {
        let mut ocs = OperatingConfig::preconditioning_file_configs(&info);
        ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        describe_preconditioning_data(&ocs);
    } else if info.extension.eq_ignore_ascii_case("operatingconfig") {
        let oc = OperatingConfig::from_license_file(&info);
        describe_operating_configs(&vec![oc])
    } else {
        eprintln!("Error: you can only decode package.json or .operatingconfig files.");
        exit(1);
    }
}

fn describe_operating_configs(ocs: &Vec<OperatingConfig>) {
    let mut current_npd_id = "";
    for (i, oc) in ocs.iter().enumerate() {
        if !current_npd_id.eq_ignore_ascii_case(&oc.npd_id) {
            current_npd_id = &oc.npd_id;
            println!("License files for npdId: {}:", &oc.npd_id);
            println!("    Package UUID: {}", &oc.package_id);
            println!("    License type: {}", &oc.mode);
            println!("    License expiry date: {}", &oc.expiry_date);
            println!("    Precedence: {}", &oc.precedence);
            println!("Filenames (shown with '...' where the npdId appears):")
        }
        println!("{: >2}: {}", i + 1, shorten_oc_file_name(&oc.filename));
        println!(
            "    App ID: {}, Certificate Group: {}",
            &oc.app_id, &oc.cert_group_id
        );
        println!("    Install date: {}", &oc.install_datetime);
    }
}

fn describe_preconditioning_data(ocs: &Vec<OperatingConfig>) {
    for (i, oc) in ocs.iter().enumerate() {
        if i == 0 {
            println!("Preconditioning data for npdId: {}", &oc.npd_id);
            println!("    Package UUID: {}", &oc.package_id);
            println!("    License type: {}", &oc.mode);
            println!("    License expiry date: {}", &oc.expiry_date);
            println!("    Precedence: {}", &oc.precedence);
            println!("Application Licenses (AppID, Certificate Group):")
        }
        println!("{: >2}: {}, {}", i + 1, &oc.app_id, &oc.cert_group_id);
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::FileInfo;
    use crate::APP_SUPPORT_DIR;

    #[test]
    fn test_os() {
        assert!(
            FileInfo::from_path(APP_SUPPORT_DIR).is_ok(),
            "Application Support path is not present"
        );
    }
}
