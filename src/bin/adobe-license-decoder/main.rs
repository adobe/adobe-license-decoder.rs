/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
mod cli;

use adobe_license_toolbox::client::descriptions::{describe_directory, describe_file};
use adobe_license_toolbox::client::types::FileInfo;
use cli::{Opt, DEFAULT_CONFIG_DIR};
use eyre::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    if let Ok(info) = FileInfo::from_path(&opt.path) {
        if info.is_directory {
            describe_directory(&info, opt.verbose)?;
        } else {
            describe_file(&info, opt.verbose)?;
        }
    } else {
        if opt.path.eq_ignore_ascii_case(DEFAULT_CONFIG_DIR) {
            eprintln!("Error: There are no licenses installed on this computer")
        } else {
            eprintln!("Error: No such directory: {}", &opt.path)
        }
        std::process::exit(1);
    };
    Ok(())
}
