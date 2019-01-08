extern crate capnpc;
extern crate cc;

use std::path::Path;
use std::fs::copy;
use std::env;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("src/")
        .file("src/circuit.capnp")
        .run().expect("schema compiler command");

    // Copy to source to be committed.
    let out_dir = &env::var("OUT_DIR").unwrap();
    let new = Path::new(out_dir).join("circuit_capnp.rs");
    copy(new, "src/circuit_capnp.rs").unwrap();

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .file("cpp/gadget.cpp")
        .compile("cpp_gadget");


    /* Tentative compilation as a shared lib.

    let mut cmd = cc::Build::new()
        .cpp(true)
        .flag("-std=c++11")
        .flag("-fPIC")
        .flag("-shared")
        .get_compiler()
        .to_command();

    let so = Path::new(out_dir).join("libcpp_gadget.so");

    let output = cmd.args(&["cpp/gadget.cpp", "-o", so.to_str().unwrap()])
        .output().expect("Compilation failed");

    println!("cargo:warning=COMPILE: {:?}", cmd);
    println!("cargo:warning=COMPILE status: {}", output.status);
    println!("cargo:warning=COMPILE stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("cargo:warning=COMPILE stderr: {}", String::from_utf8_lossy(&output.stderr));

    println!("cargo:rustc-link-lib=cpp_gadget");
    println!("cargo:rustc-link-search=.");
    */
}
