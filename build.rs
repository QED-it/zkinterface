extern crate capnpc;

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
}
