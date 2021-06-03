/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use eyre::{eyre, Result, WrapErr};

extern "C" {
    fn get_device_id(buf: *mut u8, len: i32) -> i32;
}

pub fn get_ngl_device_id() -> Result<String> {
    let mut buf: [u8; 128] = [0; 128];
    let id_len = unsafe { get_device_id(buf.as_mut_ptr(), 128) };
    if id_len < 0 {
        Err(eyre!("Device ID invalid (larger than 128 bytes)"))
    } else {
        let id_len = id_len as usize;
        Ok(std::str::from_utf8(&buf[0..id_len])
            .wrap_err("Device ID invalid (not UTF8)")?
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::get_ngl_device_id;

    #[test]
    fn test_device_id() {
        let id = get_ngl_device_id();
        assert!(id.is_ok(), "Failed to retrieve device ID");
        // println!("NGL device ID of test machine is: {}", id.as_ref().unwrap());
        assert_eq!(
            id.as_ref().unwrap().len(),
            64,
            "Device ID is not 64 characters"
        );
        assert!(
            id.as_ref().unwrap().chars().all(|c| c.is_ascii_hexdigit()),
            "Device ID is not all hex digits"
        );
    }
}
