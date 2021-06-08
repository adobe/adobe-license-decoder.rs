/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/

pub mod descriptions;
pub mod ngl_shim;
pub mod sscp;
pub mod types;

#[cfg(test)]
mod tests {
    use super::{ngl_shim, sscp};

    #[test]
    fn test_device_id_shape() {
        let ngl_id = ngl_shim::get_device_id();
        assert!(ngl_id.is_ok(), "Failed to get NGL device ID");
        assert_eq!(
            ngl_id.as_ref().unwrap().len(),
            64,
            "NGL Device ID is not 64 characters"
        );
        assert!(
            ngl_id
                .as_ref()
                .unwrap()
                .chars()
                .all(|c| c.is_ascii_hexdigit()),
            "NGL Device ID is not all hex digits"
        );
        let sscp_id = sscp::get_device_id();
        assert_eq!(sscp_id.len(), 64, "SSCP Device ID is not 64 characters");
        assert!(
            sscp_id.chars().all(|c| c.is_ascii_hexdigit()),
            "SSCP Device ID is not all hex digits"
        );
    }

    #[test]
    fn test_device_id_match() {
        let ngl_id = ngl_shim::get_device_id();
        assert!(ngl_id.is_ok(), "Failed to get NGL device ID");
        let sscp_id = sscp::get_device_id();
        assert_eq!(
            ngl_id.unwrap().as_str(),
            sscp_id.as_str(),
            "NGL and SSCP device IDs don't match"
        );
        println!("The (matching) NGL/SSCP device ID is: {}", sscp_id.as_str());
    }
}
