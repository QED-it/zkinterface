extern crate cmake;

use cmake::Config;
use std::env;

fn main() {
    let dst = Config::new("..").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    //println!("cargo:rustc-link-lib=static=zkif_gadgetlib");
    // Dependencies.
    //println!("cargo:rustc-link-search=native={}", "../libsnark/depends/libff/libff/");
    //println!("cargo:rustc-link-lib=ff");
    //println!("cargo:rustc-link-search=native={}", "/usr/local/lib/");
    //println!("cargo:rustc-link-lib=gmp");
    //println!("cargo:rustc-link-search=native={}", "../libsnark/libsnark/");
    //println!("cargo:rustc-link-lib=snark");

    // C++ stdlib
    let target = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++"); // CLang
    } else {
        println!("cargo:rustc-link-lib=dylib=stdc++"); // GCC
    }

    // Export for other Rust packages.
    let out_dir = std::env::var("OUT_DIR").unwrap();
    println!("cargo:include={}", out_dir);

    // To use the C++ part in another Rust project, include the environment variable DEP_ZKINTERFACE_INCLUDE
    // See https://doc.rust-lang.org/cargo/reference/build-scripts.html#the-links-manifest-key
    //
    // For instance, if you use the cc crate, add:
    //   .include(std::env::var("DEP_ZKINTERFACE_INCLUDE").unwrap())
}