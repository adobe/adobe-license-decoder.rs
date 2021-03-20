/*
Copyright 2020 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
extern crate base64;
extern crate chrono;
extern crate shellexpand;

use chrono::prelude::*;
use eyre::{Result, WrapErr};
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{metadata, File};
use std::path::Path;

pub type JsonMap = HashMap<String, Value>;

pub struct FileInfo {
    pub pathname: String,
    pub filename: String,
    pub name: String,
    pub extension: String,
    pub is_directory: bool,
    pub mod_date: String,
}

impl FileInfo {
    pub fn from_path(path: &str) -> Result<FileInfo> {
        let path: String = shellexpand::full(path)?.into();
        let info = metadata(&path)?;
        let path_object = Path::new(&path);
        let is_directory = info.is_dir();
        let mod_date: DateTime<Local> = info.modified()?.into();
        Ok(FileInfo {
            pathname: path.to_string(),
            filename: path_object
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            name: path_object
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            extension: path_object
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            is_directory,
            mod_date: mod_date.format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        })
    }

    pub fn from_name_and_extension(name: &str, extension: &str) -> Result<FileInfo> {
        let filename = format!("{}.{}", name, extension);
        Ok(FileInfo {
            pathname: filename.to_string(),
            filename,
            name: name.into(),
            extension: extension.into(),
            is_directory: false,
            mod_date: "Unknown".into(),
        })
    }
}

pub fn u64decode(s: &str) -> Result<String> {
    let bytes = base64::decode_config(s, base64::URL_SAFE_NO_PAD)?;
    String::from_utf8(bytes).wrap_err("Illegal license encoding")
}

pub fn u64encode(s: &str) -> Result<String> {
    Ok(base64::encode_config(s, base64::URL_SAFE_NO_PAD))
}

pub fn json_from_base64(s: &str) -> Result<JsonMap> {
    serde_json::from_str(&u64decode(s)?).wrap_err("Illegal license data")
}

pub fn date_from_epoch_millis(timestamp: &str) -> Result<String> {
    let timestamp = timestamp
        .parse::<i64>()
        .wrap_err("Illegal license timestamp")?;
    let date = Local.timestamp(timestamp / 1000, 0);
    Ok(date.format("%Y-%m-%d").to_string())
}

pub fn json_from_file(info: &FileInfo) -> Result<JsonMap> {
    let file =
        File::open(Path::new(&info.pathname)).wrap_err("Can't read license file")?;
    Ok(serde_json::from_reader(&file).wrap_err("Can't parse license data")?)
}

pub fn shorten_oc_file_name(name: &str) -> Result<String> {
    let parts: Vec<&str> = name.split('-').collect();
    if parts.len() != 3 {
        Ok(name.to_string())
    } else {
        Ok(format!("{}-...-{}", parts[0], parts[2]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_info_from_path() {
        if let Ok(fi) = FileInfo::from_path("src") {
            assert!(fi.is_directory);
            assert!(fi.extension.is_empty());
        } else {
            panic!("Failed to create file info from 'src' directory.");
        }
        if let Ok(fi) = FileInfo::from_path("src/main.rs") {
            assert!(!fi.is_directory);
            assert!(fi.extension.eq_ignore_ascii_case("rs"));
            assert!(fi.name.eq_ignore_ascii_case("main"));
        } else {
            panic!("Failed to create file info from 'src/main.rs' file");
        }
        if let Ok(_) = FileInfo::from_path("no-such-directory") {
            panic!("Created file info for non-existent path");
        }
    }

    #[test]
    fn test_file_info_from_name_and_extension() {
        let fi = FileInfo::from_name_and_extension("foo", "bar");
        assert_eq!(fi.filename, "foo.bar");
        assert_eq!(fi.pathname, "foo.bar");
    }
}
