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

// libsscp is a custom library supplied in various architectures
// and linked by the build.rs script.  It provides all the
// entries in this extern block.
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

#[cfg(test)]
mod tests {
    use super::get_device_id;
    use crate::client::sscp::FALLBACK_DEVICE_ID;

    #[test]
    fn test_device_id_shape() {
        let sscp_id = get_device_id();
        assert_eq!(sscp_id.len(), 64, "SSCP Device ID is not 64 characters");
        assert!(
            sscp_id.chars().all(|c| c.is_ascii_hexdigit()),
            "SSCP Device ID is not all hex digits"
        );
    }

    #[test]
    fn test_device_id_nomatch() {
        let sscp_id = get_device_id();
        assert_ne!(
            FALLBACK_DEVICE_ID,
            sscp_id.as_str(),
            "The test device has a fallback device ID"
        );
        println!("The test device ID is: {}", sscp_id.as_str());
    }
}
