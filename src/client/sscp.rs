/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use sha2::{Digest, Sha256};

const FALLBACK_DEVICE_ID: &str =
    "fdaa587852001bad7bf1e81ed275b822d2bc812f4eeeab092da60b7066ab2bfa";

extern "C" {
    fn get_sscp_device_id(buf: *mut u8, len: i32) -> i32;
}

pub fn get_device_id() -> String {
    const BUFSIZE: i32 = 512;
    let mut buf: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];
    let id_len = unsafe { get_sscp_device_id(buf.as_mut_ptr(), BUFSIZE) };
    match id_len {
        0 => FALLBACK_DEVICE_ID.to_string(),
        -1 => panic!("Underlying Device ID is larger than {} bytes!", BUFSIZE),
        _ => {
            let id_len = id_len as usize;
            let digest = Sha256::digest(&buf[0..id_len]);
            format!("{:x}", digest)
        }
    }
}
