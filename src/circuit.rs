use std::str::from_utf8;
use std::iter::FromIterator;
use std::io::{Read, Write};
use capnp::serialize;
use circuit_capnp::{assignments_request, assignments_response};
use circuit_capnp::structured_gadget_interface::Which::Variables;


pub fn request_assignments<W>(w: &mut W) -> ::std::io::Result<()>
    where W: Write
{
    let mut message = ::capnp::message::Builder::new_default();
    {
        let req = message.init_root::<assignments_request::Builder>();

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
                param.set_value("Gadget1".as_bytes());
            }
            {
                let mut param = params.reborrow().get(1);
                param.set_key("Field");
                param.set_value("BLS12".as_bytes());
            }
        }
    }

    serialize::write_message(w, &message)
}

pub fn handle_assignments<R>(mut r: R) -> ::capnp::Result<()>
    where R: Read
{
    let request_buf = serialize::read_message(&mut r,
                                              ::capnp::message::ReaderOptions::new())?;
    let request: assignments_request::Reader = request_buf.get_root()?;

    let mut response_buf = ::capnp::message::Builder::new_default();
    let mut response: assignments_response::Builder = response_buf.init_root();

    // Process the instance parameters.
    {
        let instance = request.get_instance()?;

        // Allocate 100 local variables.
        response.set_free_variable_id(instance.get_free_variable_id() + 100);
        println!("Got free var ID: {}", instance.get_free_variable_id());
        println!("Return free var ID: {}", response.get_free_variable_id());

        let incoming = match instance.get_incoming_struct()?.which()? {
            Variables(vars) => vars?,
            _ => panic!("Nested structs are not implemented."),
        };
        println!("incoming vars: {:?}", Vec::from_iter(incoming.iter()));

        let outgoing = match instance.get_outgoing_struct()?.which()? {
            Variables(vars) => vars?,
            _ => panic!("Nested structs are not implemented."),
        };
        println!("outgoing vars: {:?}", Vec::from_iter(outgoing.iter()));

        for param in instance.get_parameters()?.iter() {
            println!("param {} = {}", param.get_key()?, from_utf8(param.get_value()?)?);
        }
    }

    Ok(())
}

#[test]
fn test_cap_circuit() {
    let mut buf = vec![];
    request_assignments(&mut buf).unwrap();

    handle_assignments(&buf[..]).unwrap();
}