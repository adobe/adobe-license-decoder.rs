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
/// When run without arguments, decodes all the installed license files
/// on the current machine.  You can instead
/// specify a directory to decode
/// (which can contain either `.operatingconfig` files or
/// an `ngl-preconditioning-data.json` file)
/// or a license file to decode (which should have either a `.operatingconfig`
/// or a `.json` extension).
pub struct Opt {
    /// Decode more of the license (aka "verbose")
    #[structopt(short)]
    pub verbose: bool,

    /// Directory to search for licenses
    #[structopt(default_value = APP_SUPPORT_DIR)]
    pub path: String,
}
