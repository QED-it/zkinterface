use bellman_zkif::zkif_backend::{Messages, zkif_backend};
use std::io;
use std::io::Read;
use std::env;

// Example:
//
//     cat src/test/circuit_r1cs.zkif src/test/r1cs.zkif       | cargo run --release
//     cat src/test/circuit_witness.zkif src/test/witness.zkif | cargo run --release
//
pub fn main() -> Result<(), Box<std::error::Error>> {
    let mut messages = Messages::new(1);

    let mut buffer = vec![];
    io::stdin().read_to_end(&mut buffer)?;
    messages.push_message(buffer)?;

    zkif_backend(&messages, &env::current_dir()?)?;

    Ok(())
}
