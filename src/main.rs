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

use cli::Opt;
use descriptions::{describe_directory, describe_file};
use structopt::StructOpt;
use utilities::FileInfo;

fn main() {
    let opt = Opt::from_args();
    let info = match FileInfo::from_path(&opt.path) {
        Ok(val) => val,
        Err(e) => {
            panic!("{}: {}", e, &opt.path);
        }
    };
    if info.is_directory {
        describe_directory(&info, opt.verbose);
    } else {
        describe_file(&info, opt.verbose);
    }
}
