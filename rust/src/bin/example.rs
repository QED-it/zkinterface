extern crate zkinterface;
use zkinterface::examples::*;

use std::io::stdout;
use std::error::Error;


// Example:
//
//     cargo run --bin example > example.zkif
//
pub fn main() -> Result<(), Box<Error>> {

    example_circuit().write(stdout())?;
    write_example_constraints(stdout())?;
    write_example_witness(stdout())?;

    Ok(())
}
