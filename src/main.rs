extern crate zkstandard;
use zkstandard::gadget_call;

pub fn main() {

    let args : Vec<String> = ::std::env::args().collect();
    if args.len() < 2 {
        println!("usage: $ {} [write | read]", args[0]);
    } else {
        match &*args[1] {
            "write" => gadget_call::write_address_book(&mut ::std::io::stdout()).unwrap(),
            "read" =>  gadget_call::print_address_book(&mut ::std::io::stdin()).unwrap(),
            _ => {println!("unrecognized argument") }
        }
    }

}