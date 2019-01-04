@0xa55ec1c0b97af6aa;

## Types

struct VariableId {

    ownerId   @0 :UInt32;
    # Globally unique ID of the gadget that owns this variable.

    reference @1 :UInt32;
    # A reference unique within the local scope of the owning gadget.
}
# Globally unique variable IDs.


struct IdSpace {

    ownerId @0 :UInt32;
    # The unique id of a gadget.
    # The gadget own all variables that refer to this ownerId.

    owned   @1 :UInt32;
    # How many consecutive GadgetId's belong to this gadget, starting with its own id.
    # The owner may redistribute its ids to sub-gadgets.
    # The owner is responsible for the uniqueness of VariableId's
    # within the space of GadgetId's it owns.
}
# Used to delegate a part of the VariableId spaces to a gadget.
#
# Motivation: let gadgets allocate unique IDs without coordination.
#

struct Instance {
    struct Connection {
        union {
            incoming @0 :VariableId;
            outgoing @1 :VariableId;
        }
    }

    idSpace     @0 :IdSpace;
    connections @1 :List(Connection);
    params      @2 :AnyPointer;
}


struct Constraint {
    struct Term {
        variableId  @0 :VariableId;
        coefficient @1 :Data;
    }

    # (A) * (B) = (C)
    rowA @0 :List(Term);
    rowB @1 :List(Term);
    rowC @2 :List(Term);
}



struct Assignment {
    variableId @0 :VariableId;
    value      @1 :Data;
}


## Messages

# # Calling convention with shared memory and function calls:
#
# The communicating parties are assumed to use different memory management,
# even if they may run in the same process.
# The party that sends data is responsible for allocating, writing, and releasing memory.
# The party that receives data can only read it.
# It must not write because that would not be compatible with serialized communication.
# It must not store references; data to be used later must be copied.
#
# - Caller prepares a request in its memory
# - Caller calls the gadget with pointers to:
#     - the request.
#     - a chunk handler function.
#     - a response handler function.
#     - an opaque context for each handler to be given back when calling them.
# - Gadget reads the request.
# - Gadget prepares a chunk in its memory.
# - Gadget calls the chunk handler.
# - Callback reads or copy the chunk.
# - Gadget releases the chunk memory.
# - Multiple chunks may be passed by calling the chunk handler multiple times.
# - Gadget prepares a response in its memory.
# - Gadget calls the response handler.
# - Callback reads or copy the response.
# - Gadget releases the response memory.
# - Caller releases the request memory.
#

# # Message passing without shared memory:
#
# - Caller sends a serialized request
#

# # Motivation
#
# The entity handling chunks might not be the same as the one handling responses.
# In case of gadget composition, the parent can handle the response, while chunks
# are handled directly by some global environment.
# This avoid the needs to route chunks up the callstack.
#

struct ConstraintsRequest {
    instance @0 :Instance;
}

struct ConstraintsChunk {
    constraints @0 :List(Constraint);
}

struct ConstraintsResponse {
}


struct AssignmentsRequest {
    instance            @0 :Instance;
    incomingAssignments @1 :List(Assignment);
    witness             @2 :AnyPointer;
}

struct AssignmentsChunk {
    assignments @0 :List(Assignment);
}

struct AssignmentsResponse {
    union {
        error                   @0 :Text;
        response                :group {
            outgoingAssignments @1 :List(Assignment);
            info                @2 :AnyPointer;
        }
    }
}


struct GadgetRequest {
    union {
        makeConstraints @0 :ConstraintsRequest;
        makeAssignments @1 :AssignmentsRequest;
    }
}


## Methods

interface Caller {
    constrain @0 (chunk :ConstraintsChunk) -> (ok :Bool);
    assign    @1 (chunk :AssignmentsChunk) -> (ok :Bool);
}

interface Gadget {
    makeConstraints @0 (params :ConstraintsRequest, caller :Caller) -> (res :ConstraintsResponse);
    makeAssignments @1 (params :AssignmentsRequest, caller :Caller) -> (res :AssignmentsResponse);
}



## Example schema

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
