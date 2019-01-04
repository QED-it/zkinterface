use std::io::{Read, Write};
use capnp::serialize;
use circuit_capnp::{constraints_request};


pub fn write_constraint_request<W>(w: &mut W) -> ::std::io::Result<()>
    where W: Write
{
    let mut message = ::capnp::message::Builder::new_default();
    {
        let req = message.init_root::<constraints_request::Builder>();

        let parent_id = 1;
        let child_id = 2;

        let mut parent = req.init_instance();
        {
            let mut id_space = parent.reborrow().init_id_space();
            id_space.set_owner_id(child_id);
            id_space.set_owned(10);
        }
        {
            let mut incoming = parent.reborrow().init_incoming_vars(1);
            {
                let mut var = incoming.reborrow().get(0);
                var.set_owner_id(parent_id);
                var.set_reference(1);
            }
        }
        {
            let mut outgoing = parent.reborrow().init_outgoing_vars(1);
            {
                let mut var = outgoing.reborrow().get(0);
                var.set_owner_id(parent_id);
                var.set_reference(2);
            }
        }
        {
            let mut params = parent.reborrow().init_params(2);
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
    let req = message_reader.get_root::<constraints_request::Reader>()?;
    let instance = req.get_instance()?;

    let id_space = instance.get_id_space()?;
    println!("owner: {}", id_space.get_owner_id());
    println!("owned: {}", id_space.get_owned());

    for var in instance.get_incoming_vars()?.iter() {
        println!("incoming {}/{}", var.get_owner_id(), var.get_reference());
    }

    for var in instance.get_outgoing_vars()?.iter() {
        println!("outgoing {}/{}", var.get_owner_id(), var.get_reference());
    }

    for param in instance.get_params()?.iter() {
        println!("param {}={}", param.get_key()?, param.get_value().get_as::<&str>()?);
    }

    Ok(())
}


#[test]
fn test_cap_circuit() {
    let mut buf = vec![];
    write_constraint_request(&mut buf).unwrap();

    print_constraint_request(&buf[..]).unwrap();
}