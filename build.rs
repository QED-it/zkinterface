extern crate cc;

use std::path::Path;
use std::fs::rename;
use std::process::Command;

fn main() {
    match Command::new("flatc").args(&[
        "--rust",
        "--cpp",
        "-o", "src/",
        "src/gadget.fbs",
    ]).output() {
        Ok(flatc) => {
            if !
                flatc.status.success() {
                panic!("\n\nFlatBuffers code generation failed.\n{}\n{}\n",
                       String::from_utf8_lossy(&flatc.stdout),
                       String::from_utf8_lossy(&flatc.stderr));
            }

            rename(
                Path::new("src").join("gadget_generated.h"),
                Path::new("cpp").join("gadget_generated.h"),
            ).expect("Failed to rename");
        }
        Err(_) => {
            println!("cargo:warning=Install FlatBuffers (flatc) if you modify `gadget.fbs`. Code was not regenerated.");
        }
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("cpp/gadget.cpp")
        .compile("cpp_gadget");
}


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
