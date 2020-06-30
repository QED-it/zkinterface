fn main() {
    #[cfg(feature = "fbs")] {
        use std::fs::rename;
        use std::path::Path;
        use std::process::Command;

        // Latest flatc version: 1.12.0
        match Command::new("flatc").args(&[
            "--rust",
            "--cpp", "--gen-object-api", "--gen-mutable", "--gen-compare",
            "--js",
            "-o", "src/",
            "../zkinterface.fbs",
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
                    Path::new("src").join("zkinterface_generated.h"),
                    Path::new("..").join("cpp").join("zkinterface_generated.h"),
                ).expect("Failed to rename");

                // Move JS file.
                rename(
                    Path::new("src").join("zkinterface_generated.js"),
                    Path::new("..").join("js").join("zkinterface_generated.js"),
                ).expect("Failed to rename");

                // Fix an issue in generated Rust code.
                // The lifetime 'a should be on the return value, not on &self.
                // Published at https://github.com/google/flatbuffers/pull/5140
                {
                    let file = &Path::new("src").join("zkinterface_generated.rs");
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
                println!("cargo:warning=Install FlatBuffers (flatc) if you modify `zkinterface.fbs`. Code was not regenerated.");
            }
        }
    }
}
