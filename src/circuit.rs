use std::io::{Read, Write};
use capnp::serialize;
use circuit_capnp::{address_book, person, constraints_request};


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
            let mut connections = parent.reborrow().init_connections(2);
            {
                let conn = connections.reborrow().get(0);
                let mut incoming = conn.init_incoming();
                incoming.set_owner_id(parent_id);
                incoming.set_reference(1);
            }
            {
                let conn = connections.reborrow().get(1);
                let mut outgoing = conn.init_outgoing();
                outgoing.set_owner_id(parent_id);
                outgoing.set_reference(1);
            }
        }
    }

    serialize::write_message(w, &message)
}


pub fn write_address_book<W>(w: &mut W) -> ::std::io::Result<()>
    where W: Write
{
    let mut message = ::capnp::message::Builder::new_default();
    {
        let address_book = message.init_root::<address_book::Builder>();

        let mut people = address_book.init_people(2);

        {
            let mut alice = people.reborrow().get(0);
            alice.set_id(123);
            alice.set_name("AAAAAAAAAAAAAAAAAAAAAAA");
            alice.set_email("alice@example.com");
            {
                let mut alice_phones = alice.reborrow().init_phones(1);
                alice_phones.reborrow().get(0).set_number("555-1212");
                alice_phones.reborrow().get(0).set_type(person::phone_number::Type::Mobile);
            }
            alice.get_employment().set_school("MIT");
        }

        {
            let mut bob = people.get(1);
            bob.set_id(456);
            bob.set_name("Bob");
            bob.set_email("bob@example.com");
            {
                let mut bob_phones = bob.reborrow().init_phones(2);
                bob_phones.reborrow().get(0).set_number("555-4567");
                bob_phones.reborrow().get(0).set_type(person::phone_number::Type::Home);
                bob_phones.reborrow().get(1).set_number("555-7654");
                bob_phones.reborrow().get(1).set_type(person::phone_number::Type::Work);
            }
            bob.get_employment().set_unemployed(());
        }
    }

    serialize::write_message(w, &message)
}

pub fn print_address_book<R>(mut r: R) -> ::capnp::Result<()>
    where R: Read
{
    let message_reader = serialize::read_message(&mut r,
                                                 ::capnp::message::ReaderOptions::new())?;
    let address_book = message_reader.get_root::<address_book::Reader>()?;

    for person in address_book.get_people()?.iter() {
        println!("{}: {}", person.get_name()?, person.get_email()?);
        for phone in person.get_phones()?.iter() {
            let type_name = match phone.get_type() {
                Ok(person::phone_number::Type::Mobile) => "mobile",
                Ok(person::phone_number::Type::Home) => "home",
                Ok(person::phone_number::Type::Work) => "work",
                Err(::capnp::NotInSchema(_)) => "UNKNOWN",
            };
            println!("  {} phone: {}", type_name, phone.get_number()?);
        }
        match person.get_employment().which() {
            Ok(person::employment::Unemployed(())) => {
                println!("  unemployed");
            }
            Ok(person::employment::Employer(employer)) => {
                println!("  employer: {}", employer?);
            }
            Ok(person::employment::School(school)) => {
                println!("  student at: {}", school?);
            }
            Ok(person::employment::SelfEmployed(())) => {
                println!("  self-employed");
            }
            Err(::capnp::NotInSchema(_)) => {}
        }
    }
    Ok(())
}

#[test]
fn test_cap() {
    let mut buf = vec![];
    write_address_book(&mut buf).unwrap();

    buf[112] = 66;
    print_address_book(&buf[..]).unwrap();
}