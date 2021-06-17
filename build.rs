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
        println!("cargo:rustc-link-lib=static=ngl-lib");
        println!("cargo:rustc-link-lib=static=winhttp");
        println!("cargo:rustc-link-lib=static=kernel32");
        println!("cargo:rustc-link-lib=static=user32");
        println!("cargo:rustc-link-lib=static=gdi32");
        println!("cargo:rustc-link-lib=static=winspool");
        println!("cargo:rustc-link-lib=static=comdlg32");
        println!("cargo:rustc-link-lib=static=advapi32");
        println!("cargo:rustc-link-lib=static=shell32");
        println!("cargo:rustc-link-lib=static=ole32");
        println!("cargo:rustc-link-lib=static=oleaut32");
        println!("cargo:rustc-link-lib=static=uuid");
        println!("cargo:rustc-link-lib=static=odbc32");
        println!("cargo:rustc-link-lib=static=odbccp32");
        println!("cargo:rustc-link-lib=static=libcpmt");
    }
}
