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
    let library_path = format!("rsrc/libraries/{}-{}", &target_os, &target_arch);
    println!("cargo:rustc-link-search=native={}", &library_path);
    if target_os.eq_ignore_ascii_case("macos") {
        println!("cargo:rustc-link-lib=static=sscp");
        println!("cargo:rustc-link-lib=dylib=c++");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=IOKit");
    } else if target_os.eq_ignore_ascii_case("windows") {
        println!("cargo:rustc-link-lib=static=sscp");
        println!("cargo:rustc-link-lib=dylib=winhttp");
        println!("cargo:rustc-link-lib=dylib=kernel32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=winspool");
        println!("cargo:rustc-link-lib=dylib=comdlg32");
        println!("cargo:rustc-link-lib=dylib=advapi32");
        println!("cargo:rustc-link-lib=dylib=shell32");
        println!("cargo:rustc-link-lib=dylib=ole32");
        println!("cargo:rustc-link-lib=dylib=oleaut32");
        println!("cargo:rustc-link-lib=dylib=uuid");
        println!("cargo:rustc-link-lib=dylib=odbc32");
        println!("cargo:rustc-link-lib=dylib=odbccp32");
        println!("cargo:rustc-link-lib=dylib=libcpmt");
    }
}
