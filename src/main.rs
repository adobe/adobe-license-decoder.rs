/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
mod cli;
mod descriptions;
mod types;
mod utilities;

use cli::{Opt, DEFAULT_CONFIG_DIR};
use descriptions::{describe_directory, describe_file};
use eyre::Result;
use structopt::StructOpt;
use utilities::FileInfo;

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
