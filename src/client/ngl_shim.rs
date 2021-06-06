/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use eyre::{eyre, Result, WrapErr};

extern "C" {
    fn get_ngl_device_id(buf: *mut u8, len: i32) -> i32;
}

pub fn get_device_id() -> Result<String> {
    let mut buf: [u8; 128] = [0; 128];
    let id_len = unsafe { get_ngl_device_id(buf.as_mut_ptr(), 128) };
    if id_len < 0 {
        Err(eyre!("Device ID invalid (larger than 128 bytes)"))
    } else {
        let id_len = id_len as usize;
        Ok(std::str::from_utf8(&buf[0..id_len])
            .wrap_err("Device ID invalid (not UTF8)")?
            .to_string())
    }
}
