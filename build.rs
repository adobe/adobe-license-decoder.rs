/*
Copyright 2021 Adobe
All Rights Reserved.

NOTICE: Adobe permits you to use, modify, and distribute this file in
accordance with the terms of the Adobe license agreement accompanying
it.
*/
fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if target_os.eq_ignore_ascii_case("macos") {
        let library_path = format!("rsrc/libraries/{}-{}", &target_os, &target_arch);
        println!("cargo:rustc-link-search=native={}", &library_path);
        println!("cargo:rustc-link-lib=static={}", "sscp");
        println!("cargo:rustc-link-lib=dylib={}", "c++");
        // println!("cargo:rustc-link-lib=framework={}", "Cocoa");
        println!("cargo:rustc-link-lib=framework={}", "CoreFoundation");
        // println!("cargo:rustc-link-lib=framework={}", "CoreServices");
        // println!("cargo:rustc-link-lib=framework={}", "CoreVideo");
        println!("cargo:rustc-link-lib=framework={}", "IOKit");
        // println!("cargo:rustc-link-lib=framework={}", "Security");
        // println!("cargo:rustc-link-lib=framework={}", "SystemConfiguration");
        // println!("cargo:rustc-link-lib=framework={}", "Webkit");
    }
}
