extern crate zkstandard;
use zkstandard::circuit;

pub fn main() {

    let args : Vec<String> = ::std::env::args().collect();
    if args.len() < 2 {
        println!("usage: $ {} [write | read]", args[0]);
    } else {
        match &*args[1] {
            "write" => circuit::write_address_book(&mut ::std::io::stdout()).unwrap(),
            "read" =>  circuit::print_address_book(&mut ::std::io::stdin()).unwrap(),
            _ => {println!("unrecognized argument") }
        }
    }

}