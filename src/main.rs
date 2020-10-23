mod types;
mod utilities;

use std::env;
use std::process::exit;
use types::*;
use utilities::*;
use std::cmp::Ordering::Equal;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = match args.len() {
        1 => {
            if cfg!(target_os = "macos") {
                "/Library/Application Support/Adobe/OperatingConfigs"
            } else {
                "%ProgramFiles%/Adobe/OperatingConfigs"
            }
        }
        2 => &args[1],
        _ => {
            eprintln!("Too many arguments: {:?}", &args[1..]);
            eprintln!("Usage: frl-license-decoder [directory-or-preconditiong-file]");
            exit(1);
        }
    };
    let info = match FileInfo::from_path(path) {
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
            describe_operating_configs(&ocs);
        }
    } else if info.extension.eq_ignore_ascii_case("json") {
        let mut ocs = OperatingConfig::preconditioning_file_configs(&info);
        ocs.sort_by(|oc1, oc2|
            match oc1.app_id.cmp(&oc2.app_id) {
                Equal => oc1.install_datetime.cmp(&oc2.install_datetime),
                default => default,
        });
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
    println!("License files:");
    for (i, oc) in ocs.iter().enumerate() {
        println!("{}: {}", i + 1, &oc.filename);
        println!(
            "\tApp ID: {}, Certificate Group: {}",
            &oc.app_id,
            &oc.cert_group_id
        );
        println!("\tLicense type: {}", &oc.mode);
        println!("\tLicense expiry date: {}", &oc.expiry_date);
        println!("\tPrecedence: {}", &oc.precedence);
        println!("\tInstall date: {}", &oc.install_datetime);
    }
}

fn describe_preconditioning_data(ocs: &Vec<OperatingConfig>) {
    println!("Preconditioning Data:");
    for (i, oc) in ocs.iter().enumerate() {
        if i == 0 {
            println!("Package ID: {} ({})", &oc.npd_id, &oc.package_id);
            println!("License type: {}", &oc.mode);
            println!("License expiry date: {}", &oc.expiry_date);
            println!("Precedence: {}", &oc.precedence);
        }
        println!(
            "\t{}: App ID: {}, Certificate Group: {}",
            i + 1,
            &oc.app_id,
            &oc.cert_group_id
        );
    }
}
