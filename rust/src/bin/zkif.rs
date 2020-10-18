extern crate zkinterface;

use zkinterface::{Result, cli::{cli, Options}};
use structopt::StructOpt;

fn main() -> Result<()> {
    cli(&Options::from_args())
}
