extern crate cc;

use std::fs::rename;
use std::path::Path;
use std::process::Command;

fn main() {
    match Command::new("flatc").args(&[
        "--rust",
        "--cpp",
        "-o", "src/",
        "../gadget.fbs",
    ]).output() {
        Ok(flatc) => {
            if !
                flatc.status.success() {
                panic!("\n\nFlatBuffers code generation failed.\n{}\n{}\n",
                       String::from_utf8_lossy(&flatc.stdout),
                       String::from_utf8_lossy(&flatc.stderr));
            }

            // Move C++ file.
            rename(
                Path::new("src").join("gadget_generated.h"),
                Path::new("..").join("cpp").join("gadget_generated.h"),
            ).expect("Failed to rename");

            // Fix an issue in generated code.
            // The lifetime 'a should be on the return value, not on &self.
            // Published at https://github.com/google/flatbuffers/pull/5140
            {
                let file = &Path::new("src").join("gadget_generated.rs");
                let code = std::fs::read_to_string(file).expect("could not read file");

                let re = regex::Regex::new(
                    r"pub fn (\w+)_as_(\w+)\(&'a self\) -> Option<(\w+)> \{"
                ).unwrap();
                let fixed = re.replace_all(
                    &code,
                    r"pub fn ${1}_as_${2}(&self) -> Option<${3}<'a>> {",
                ).to_string();

                let re2 = regex::Regex::new(
                    r"\(&self\) -> Option<flatbuffers::Vector<flatbuffers::ForwardsUOffset<"
                ).unwrap();
                let fixed2 = re2.replace_all(
                    &fixed,
                    r"(&self) -> Option<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<",
                ).to_string();

                std::fs::write(file, fixed2).expect("could not write file");
            }
        }
        Err(_) => {
            println!("cargo:warning=Install FlatBuffers (flatc) if you modify `gadget.fbs`. Code was not regenerated.");
        }
    }

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .file("../cpp/gadget.cpp")
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
