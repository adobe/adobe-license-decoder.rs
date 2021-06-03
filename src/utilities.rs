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
use eyre::{eyre, Result, WrapErr};
use serde_json::Value;
use std::collections::HashMap;

pub type JsonMap = HashMap<String, Value>;

pub fn u64decode(s: &str) -> Result<String> {
    let bytes = base64::decode_config(s, base64::URL_SAFE_NO_PAD)?;
    String::from_utf8(bytes).wrap_err("Illegal payload encoding")
}

pub fn u64encode(s: &str) -> Result<String> {
    Ok(base64::encode_config(s, base64::URL_SAFE_NO_PAD))
}

pub fn json_from_base64(s: &str) -> Result<JsonMap> {
    serde_json::from_str(&u64decode(s)?).wrap_err("Illegal payload data")
}

pub fn json_from_str(s: &str) -> Result<JsonMap> {
    serde_json::from_str(s).wrap_err("Illegal license data")
}

pub fn date_from_epoch_millis(timestamp: &str) -> Result<String> {
    let timestamp = timestamp
        .parse::<i64>()
        .wrap_err("Illegal license timestamp")?;
    let date = Local.timestamp(timestamp / 1000, 0);
    Ok(date.format("%Y-%m-%d").to_string())
}

pub fn json_from_file(path: &str) -> Result<JsonMap> {
    let file = std::fs::File::open(std::path::Path::new(path))
        .wrap_err("Can't read license file")?;
    serde_json::from_reader(&file).wrap_err("Can't parse license data")
}

pub fn shorten_oc_file_name(name: &str) -> Result<String> {
    let parts: Vec<&str> = name.split('-').collect();
    if parts.len() != 3 {
        Ok(name.to_string())
    } else {
        Ok(format!("{}-...-{}", parts[0], parts[2]))
    }
}

#[cfg(target_os = "macos")]
pub fn get_saved_credential(key: &str) -> Result<String> {
    let service = format!("Adobe App Info ({})", &key);
    let keyring = keyring::Keyring::new(&service, "App Info");
    keyring.get_password().map_err(|e| eyre!(e))
}

#[cfg(target_os = "windows")]
pub fn get_saved_credential(key: &str) -> Result<String> {
    let mut result = String::new();
    for i in 1..100 {
        let service = format!("Adobe App Info ({})(Part{})", key, i);
        let keyring = keyring::Keyring::new(&service, "App Info");
        let note = keyring.get_password_for_target(&service);
        if let Ok(note) = note {
            result.push_str(note.trim());
        } else {
            break;
        }
    }
    if result.is_empty() {
        Err(eyre!("No credential data found"))
    } else {
        Ok(result)
    }
}
