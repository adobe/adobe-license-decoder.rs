/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use structopt::StructOpt;

const APP_SUPPORT_DIR: &str = if cfg!(target_os = "macos") {
    "/Library/Application Support/Adobe/OperatingConfigs"
} else if cfg!(target_os = "windows") {
    "${ProgramData}/Adobe/OperatingConfigs"
} else {
    "This module can only run on MacOS or Windows"
};

#[derive(Debug, StructOpt)]
/// Adobe License Decoder
///
/// Decodes all the installed license files on the current machine.
/// If you specify a directory, it will decode all the license files
/// (ending in `.operatingconfig`) or preconditioning files
/// (named `ngl-preconditioning-data.json`) found in that directory.
/// If you specify a license or preconditioning file, it will
/// decode that file.
pub struct Opt {
    /// Output additional license data (e.g., census codes)
    #[structopt(short, long)]
    pub verbose: bool,

    /// path to directory or file to decode
    #[structopt(default_value = APP_SUPPORT_DIR)]
    pub path: String,
}
