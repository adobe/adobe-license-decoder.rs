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
        // println!("cargo:rustc-env=MACOSX_DEPLOYMENT_TARGET=10.14");
        println!("cargo:rustc-link-search=native={}", "rsrc/libraries");
        println!("cargo:rustc-link-search=native={}", 
                 "/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/lib");
        println!(
            "cargo:rustc-link-search=framework={}",
            "/System/Library/Frameworks"
        );
        println!(
            "cargo:rustc-link-search=framework={}",
            "/System/Library/PrivateFrameworks"
        );
        println!(
            "cargo:rustc-link-lib=static={}",
            format!("sscp-{}", &target_arch)
        );
        println!(
            "cargo:rustc-link-lib=static={}",
            format!("awg-ngl-{}", &target_arch)
        );
        println!("cargo:rustc-link-lib={}", "c++");
        println!("cargo:rustc-link-lib=framework={}", "Cocoa");
        println!("cargo:rustc-link-lib=framework={}", "CoreFoundation");
        println!("cargo:rustc-link-lib=framework={}", "CoreServices");
        println!("cargo:rustc-link-lib=framework={}", "CoreVideo");
        println!("cargo:rustc-link-lib=framework={}", "IOKit");
        println!("cargo:rustc-link-lib=framework={}", "Security");
        println!("cargo:rustc-link-lib=framework={}", "SystemConfiguration");
        println!("cargo:rustc-link-lib=framework={}", "Webkit");
    }
}
