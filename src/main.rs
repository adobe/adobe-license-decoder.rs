/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
mod cli;
mod types;
mod utilities;

use std::cmp::Ordering::Equal;
use std::process::exit;
use structopt::StructOpt;
use types::*;
use utilities::*;

fn main() {
    let opt = cli::Opt::from_args();
    let info = match FileInfo::from_path(&opt.path) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}: {}", e, &opt.path);
            exit(1);
        }
    };
    if info.is_directory {
        let json_file = format!("{}/ngl-preconditioning-data.json", info.pathname);
        if let Ok(info) = FileInfo::from_path(&json_file) {
            let mut ocs = OperatingConfig::preconditioning_file_configs(&info);
            ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
            describe_preconditioning_data(&ocs, opt.verbose);
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
                eprintln!("No license files found in '{}'", info.pathname);
                exit(1);
            } else {
                ocs.sort_by(|oc1, oc2| match oc1.npd_id.cmp(&oc2.npd_id) {
                    Equal => oc1.app_id.cmp(&oc2.app_id),
                    otherwise => otherwise,
                });
                describe_operating_configs(&ocs, opt.verbose);
            }
        }
    } else if info.extension.eq_ignore_ascii_case("json") {
        let mut ocs = OperatingConfig::preconditioning_file_configs(&info);
        ocs.sort_by(|oc1, oc2| oc1.app_id.cmp(&oc2.app_id));
        describe_preconditioning_data(&ocs, opt.verbose);
    } else if info.extension.eq_ignore_ascii_case("operatingconfig") {
        let oc = OperatingConfig::from_license_file(&info);
        describe_operating_configs(&vec![oc], opt.verbose)
    } else {
        eprintln!("Error: you can only decode package.json or .operatingconfig files.");
        exit(1);
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
