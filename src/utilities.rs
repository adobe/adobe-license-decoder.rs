extern crate base64;
extern crate chrono;
extern crate shellexpand;

use chrono::prelude::*;
use std::error::Error;
use std::fs::{metadata, File};
use std::path::Path;
use std::collections::HashMap;

pub type JsonMap = HashMap<String, serde_json::Value>;

pub struct FileInfo {
    pub pathname: String,
    pub filename: String,
    pub name: String,
    pub extension: String,
    pub is_directory: bool,
    pub mod_date: String,
}

impl FileInfo {
    pub fn from_path(path: &str) -> Result<FileInfo, Box<dyn Error>> {
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

    pub fn from_name_and_extension(name: &str, extension: &str) -> FileInfo {
        let filename = format!("{}.{}", name, extension);
        FileInfo {
            pathname: filename.to_string(),
            filename,
            name: name.into(),
            extension: extension.into(),
            is_directory: false,
            mod_date: "Unknown".into(),
        }
    }
}

pub fn u64decode(s: &str) -> String {
    let bytes = base64::decode_config(s, base64::URL_SAFE_NO_PAD).unwrap();
    String::from_utf8(bytes).unwrap()
}

pub fn json_from_base64(s: &str) -> JsonMap {
    serde_json::from_str(&u64decode(s)).unwrap()
}

pub fn date_from_epoch_millis(timestamp: &str) -> String {
    let timestamp = timestamp.parse::<i64>().unwrap() / 1000;
    let date = Local.timestamp(timestamp, 0);
    date.format("%Y-%m-%d").to_string()
}

pub fn json_from_file(info: &FileInfo) -> JsonMap {
    let file = File::open(Path::new(&info.pathname)).unwrap();
    serde_json::from_reader(&file).unwrap()
}

pub fn shorten_oc_file_name(name: &str) -> String {
    let parts: Vec<&str> = name.split("-").collect();
    format!("{}-...-{}", parts[0], parts[2])
}