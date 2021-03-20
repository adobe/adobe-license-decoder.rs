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
use eyre::{Result, WrapErr};
use structopt::StructOpt;
use utilities::FileInfo;

fn main() -> Result<()> {
    let opt: Opt = Opt::from_args();
    let info = if let Some(path) = opt.path.as_ref() {
        FileInfo::from_path(path)
            .wrap_err_with(|| format!("Can't find directory: {}", path))?
    } else {
        FileInfo::from_path(DEFAULT_CONFIG_DIR)
            .wrap_err("There are no licenses installed on this computer")?
    };
    if info.is_directory {
        describe_directory(&info, opt.verbose)?;
    } else {
        describe_file(&info, opt.verbose)?;
    }
    Ok(())
}
