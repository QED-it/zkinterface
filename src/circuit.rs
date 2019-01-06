use std::io::{Read, Write};
use capnp::serialize;
use circuit_capnp::{instance_request};
use circuit_capnp::struct_var::Which::Variables;


pub fn write_constraint_request<W>(w: &mut W) -> ::std::io::Result<()>
    where W: Write
{
    let mut message = ::capnp::message::Builder::new_default();
    {
        let req = message.init_root::<instance_request::Builder>();

        let parent_id = 1;
        let child_id = 2;

        let mut instance = req.init_instance();
        instance.set_free_variable_id(child_id);
        {
            let mut incoming = instance.reborrow().init_incoming_struct();
            {
                let mut vars = incoming.reborrow().init_variables(1);
                vars.set(0, parent_id + 1);
            }
        }
        {
            let mut outgoing = instance.reborrow().init_outgoing_struct();
            {
                let mut vars = outgoing.reborrow().init_variables(1);
                vars.set(1, parent_id + 2);
            }
        }
        {
            let mut params = instance.reborrow().init_parameters(2);
            {
                let mut param = params.reborrow().get(0);
                param.set_key("Name");
                param.init_value().set_as("Gadget1").unwrap();
            }
            {
                let mut param = params.reborrow().get(1);
                param.set_key("Field");
                param.init_value().set_as("BLS12").unwrap();
            }
        }
    }

    serialize::write_message(w, &message)
}


pub fn print_constraint_request<R>(mut r: R) -> ::capnp::Result<()>
    where R: Read
{
    let message_reader = serialize::read_message(&mut r,
                                                 ::capnp::message::ReaderOptions::new())?;
    let req = message_reader.get_root::<instance_request::Reader>()?;
    let instance = req.get_instance()?;

    println!("First var ID: {}", instance.get_free_variable_id());

    let incoming = match instance.get_incoming_struct()?.which()? {
      Variables(vars) => vars?,
        _ => panic!("Nested structs are not implemented."),
    };
    for var in incoming.iter() {
        println!("incoming {}", var);
    }

    let outgoing = match instance.get_outgoing_struct()?.which()? {
        Variables(vars) => vars?,
        _ => panic!("Nested structs are not implemented."),
    };
    for var in outgoing.iter() {
        println!("outgoing {}", var);
    }

    for param in instance.get_parameters()?.iter() {
        println!("param {} = {}", param.get_key()?, param.get_value().get_as::<&str>()?);
    }

    Ok(())
}


#[test]
fn test_cap_circuit() {
    let mut buf = vec![];
    write_constraint_request(&mut buf).unwrap();

    print_constraint_request(&buf[..]).unwrap();
}