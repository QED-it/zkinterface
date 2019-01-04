@0xa55ec1c0b97af6aa;

## Types

struct VariableId {

    ownerId   @0 :UInt32;
    # Globally unique ID of the gadget that owns this variable.

    reference @1 :UInt32;
    # A reference unique within the local scope of the owning gadget.
}
# Globally unique variable IDs.

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
# Low-level constraint between variables.
# Targets the generic mechanisms that build circuits.

struct Assignment {
    variableId @0 :VariableId;
    value      @1 :Data;
}
# Low-level assignment to a variable.
# Targets the generic mechanisms that prepare proofs.

struct Struct {
    union {
        variables @0 :List(VariableId);
        structs   @1 :List(Struct);
    }
    type          @2 :Text;
    name          @3 :Text;
    info          @4 :List(KeyValue);

    xInternal     @5 :AnyPointer;
    # A space reserved for implementations to track internal information
    # along with the struct. Not part of the protocol.
}
# A high-level structure of variables.
# Targets gadget composition.

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

struct KeyValue {
    key   @0 :Text;
    value @1 :AnyPointer;
}

struct Instance {
    idSpace        @0 :IdSpace;
    incomingStruct @1 :Struct;
    outgoingStruct @2 :Struct;
    params         @3 :List(KeyValue);

    xInternal      @4 :AnyPointer;
    # A space reserved for implementations to track internal information
    # along with the instance. Not part of the protocol.
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
#     - a return handler function.
#     - an opaque context for each handler to be given back when calling them.
# - Gadget reads the request.
# - Gadget prepares a chunk in its memory.
# - Gadget calls the chunk handler.
# - Callback reads or copy the chunk.
# - Gadget releases the chunk memory.
# - Multiple chunks may be passed by calling the chunk handler multiple times.
# - Gadget prepares return data in its memory.
# - Gadget calls the return handler.
# - Callback reads or copy the return data.
# - Gadget releases the return memory.
# - Caller releases the request memory.
#

# # Message passing without shared memory:
#
# - Caller sends a serialized request
#

# # Motivation
#
# The entity handling chunks might not be the same as the one handling the return.
# In case of gadget composition, the parent can handle the return, while chunks
# are handled directly by some global environment.
# This avoid the needs to route chunks up the callstack.
#

struct ConstraintsRequest {
    instance       @0 :Instance;

    xChunkContext  @1 :AnyPointer; # Opaque data to pass to the chunk handler.
    xReturnContext @2 :AnyPointer; # Opaque data to pass to the return handler.
}

struct ConstraintsChunk {
    constraints   @0 :List(Constraint);

    xChunkContext @1 :AnyPointer;
}

struct ConstraintsReturn {
    union {
        error      @0 :Text;
        return     :group {
            info   @1 :List(KeyValue);
        }
    }

    xReturnContext @2 :AnyPointer;
}


struct AssignmentsRequest {
    instance            @0 :Instance;
    incomingAssignments @1 :List(Assignment);
    witness             @2 :List(KeyValue);

    xChunkContext       @3 :AnyPointer; # Opaque data to pass to the chunk handler.
    xReturnContext      @4 :AnyPointer; # Opaque data to pass to the return handler.
}

struct AssignmentsChunk {
    assignments   @0 :List(Assignment);

    xChunkContext @1 :AnyPointer;
}

struct AssignmentsReturn {
    union {
        error                   @0 :Text;
        return                  :group {
            outgoingAssignments @1 :List(Assignment);
            info                @2 :List(KeyValue);
        }
    }

    xReturnContext              @3 :AnyPointer;
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
    makeConstraints @0 (params :ConstraintsRequest, caller :Caller) -> (res :ConstraintsReturn);
    makeAssignments @1 (params :AssignmentsRequest, caller :Caller) -> (res :AssignmentsReturn);
}
