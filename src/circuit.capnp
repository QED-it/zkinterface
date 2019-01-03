@0xa55ec1c0b97af6aa;

## Example

struct Person {
  id @0 :UInt32;
  name @1 :Text;
  email @2 :Text;
  phones @3 :List(PhoneNumber);

  struct PhoneNumber {
    number @0 :Text;
    type @1 :Type;

    enum Type {
      mobile @0;
      home @1;
      work @2;
    }
  }

  employment :union {
    unemployed @4 :Void;
    employer @5 :Text;
    school @6 :Text;
    selfEmployed @7 :Void;
    # We assume that a person is only one of these.
  }
}

struct AddressBook {
  people @0 :List(Person);
}


## Circuit Messages

struct Variable {
    namespace @0 :UInt32;
    number @1 :UInt32;
}

struct Instance {
    namespace @0 :UInt32;
    connections @1 :List(Variable);
}

struct ConstraintRequest {
    instance @0 :Instance;
}