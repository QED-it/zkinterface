extern crate zkinterface;
use zkinterface::examples::*;

use std::io::stdout;
use std::error::Error;


// Example:
//
//     cargo run --bin example > ../examples/example.zkif
//
pub fn main() -> Result<(), Box<dyn Error>> {

    example_circuit().write_into(stdout())?;
    write_example_constraints(stdout())?;
    write_example_witness(stdout())?;

    Ok(())
}
