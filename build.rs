extern crate capnpc;

fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("src/")
        .file("src/circuit.capnp")
        .run().expect("schema compiler command");
}
