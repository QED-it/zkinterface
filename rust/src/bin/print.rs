extern crate zkinterface;
use zkinterface::reading::Messages;

use std::io::{stdin, Read};
use std::error::Error;


// Example:
//
//     cargo run --bin print < example.zkif
//
pub fn main() -> Result<(), Box<Error>> {
    let mut messages = Messages::new(1);

    let mut buffer = vec![];
    stdin().read_to_end(&mut buffer)?;
    messages.push_message(buffer)?;

    println!("{:?}", messages);

    Ok(())
}
