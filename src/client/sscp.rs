/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
use sha2::{Digest, Sha256};

// libsscp is a custom library supplied in various architectures
// and linked by the build.rs script.  It provides all the
// entries in this extern block.
mod libsscp {
    pub const FALLBACK_NGL_DEVICE_ID: &str =
        "fdaa587852001bad7bf1e81ed275b822d2bc812f4eeeab092da60b7066ab2bfa";

    extern "C" {
        pub fn get_ngl_hardware_id(buf: *mut u8, len: i32) -> i32;
    }
}

pub fn get_ngl_device_id() -> String {
    const BUFSIZE: i32 = 512;
    let mut buf: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];
    let id_len = unsafe { libsscp::get_ngl_hardware_id(buf.as_mut_ptr(), BUFSIZE) };
    match id_len {
        0 => libsscp::FALLBACK_NGL_DEVICE_ID.to_string(),
        -1 => panic!("Underlying NGL Device ID is larger than {} bytes!", BUFSIZE),
        _ => {
            let id_len = id_len as usize;
            let digest = Sha256::digest(&buf[0..id_len]);
            format!("{:x}", digest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_ngl_device_id;
    use super::libsscp;

    #[test]
    fn test_device_id_shape() {
        let ngl_id = get_ngl_device_id();
        assert_eq!(ngl_id.len(), 64, "NGL Device ID is not 64 characters");
        assert!(
            ngl_id.chars().all(|c| c.is_ascii_hexdigit()),
            "SSCP Device ID is not all hex digits"
        );
    }

    #[test]
    fn test_device_id_match_cases() {
        const BUFSIZE: i32 = 512;
        let mut buf: [u8; BUFSIZE as usize] = [0; BUFSIZE as usize];
        let ngl_id = get_ngl_device_id();
        let sscp_id = unsafe { libsscp::get_ngl_hardware_id(buf.as_mut_ptr(), BUFSIZE) };
        match sscp_id {
            0 => assert_eq!(
                ngl_id,
                libsscp::FALLBACK_NGL_DEVICE_ID,
                "Fallback ID was not returned but should have been."
            ),
            _ => assert_ne!(
                ngl_id,
                libsscp::FALLBACK_NGL_DEVICE_ID,
                "Fallback ID was returned but should not have been."
            ),
        }
        println!("The test machine NGL device ID is: {}", ngl_id.as_str());
    }
}
