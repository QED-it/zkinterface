extern crate cmake;

use cmake::Config;
use std::env;

fn main() {
    let path_var = "ZKINTERFACE_LIBSNARK_PATH";

    let lib_dir = if let Ok(lib_dir) = env::var(path_var) {
        lib_dir
    } else {
        let dst = Config::new("..").build();
        dst.display().to_string()
    };

    println!("cargo:rerun-if-env-changed={}", path_var);
    println!("cargo:rustc-link-search=native={}/lib", lib_dir);
    println!("cargo:rustc-link-lib=static=zkif_gadgetlib");
    // Dependencies.
    println!("cargo:rustc-link-lib=ff");
    println!("cargo:rustc-link-lib=gmp");
    //println!("cargo:rustc-link-lib=snark");

    // C++ stdlib
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++"); // CLang
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++"); // GCC
    }

    // Export for other Rust packages.
    println!("cargo:include={}/include", lib_dir);
    // To use the C++ part in another Rust project, include the environment variable DEP_LIBSNARK_RUST_INCLUDE
    // See https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
    //
    // For instance, if you use the cc crate, add:
    //   .include(std::env::var("DEP_LIBSNARK_RUST_INCLUDE").unwrap())
}