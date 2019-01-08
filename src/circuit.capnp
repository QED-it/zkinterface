@0xa55ec1c0b97af6aa;

## Types

#struct OwnedVariableId {
#    index   @0 :UInt32;
    # A reference unique within the local scope of the owning gadget.
    # Zero is a reserved special value.

#    ownerId @1 :UInt32;
    # Globally unique ID of the instance that owns this variable.
    # Zero is a reserved special value.
#}
# Globally unique variable ID interpreted as a namespace.
# Going with the simpler concept below for now.

using VariableId = UInt64;
# Globally unique variable ID.
# Zero is a reserved special value.

struct Term {
    coefficientLE @0 :Data;
    # A coefficient in little-endian.

    variableId    @1 :VariableId;
    # The ID comes last to allow optimizations that omit it.
}
# A term in a R1CS row.
# Intended to be sent in sequences.

struct Constraint {
    # (A) * (B) = (C)
    rowA @0 :List(Term);
    rowB @1 :List(Term);
    rowC @2 :List(Term);
}
# A low-level R1CS constraint between variables.
# Targets the generic mechanisms that build circuits.
# Intended to be sent in sequences.

struct Assignment {
    valueLE    @0 :Data;
    # A value in little-endian.

    variableId @1 :VariableId;
    # The ID comes last to allow optimizations that omit it.
}
# A low-level assignment to a variable.
# Targets the generic mechanisms that prepare proofs.
# Intended to be sent in sequences.

struct KeyValue {
    key   @0 :Text;
    value @1 :AnyPointer;
}
# Generic key-value for miscellaneous attributes.

struct StructVar {
    union {
        variables @0 :List(VariableId);
        structs   @1 :List(StructVar);
    }
    type          @2 :Text;
    name          @3 :Text;
    info          @4 :List(KeyValue);

    xInternal     @5 :AnyPointer;
    # A space reserved for implementations to track internal information
    # along with the struct. Not part of the protocol.
}
# A high-level structure of variables.
# In gadget composition, the parent provides these structures to its child.
# A gadget should document what structures it can accept.

struct Instance {

    freeVariableId @0 :VariableId;
    # First free variable ID. The instance can allocate IDs greater or equal.

    incomingStruct @1 :StructVar;
    # Structure of variables that must be assigned by the calling parent.

    outgoingStruct @2 :StructVar;
    # Structure of variables that must be assigned by the called gadget.
    # There may be no outgoing variables if the gadget represents a pure assertion.

    parameters     @3 :List(KeyValue);
    # Any parameter that may influence the instance behavior.
    # Parameters can be standard, conventional, or specific to a gadget.

    xInternal      @4 :AnyPointer;
    # A space reserved for implementations to track internal information
    # along with the instance. Not part of the protocol.
}
# An instance of a gadget as part of a circuit.


# == Messages for Instantiation ==

struct InstanceRequest {
    instance         @0 :Instance;

    xChunkContext    @1 :AnyPointer;
    # Opaque data to pass to the chunk handler.

    xResponseContext @2 :AnyPointer;
    # Opaque data to pass to the return handler.
}
# Request to build an instance.

struct ConstraintsChunk {
    constraints   @0 :List(Constraint);
    # Constraints to add.

    xChunkContext @1 :AnyPointer;
    # The opaque data given in the request.
}
# Report all constraints in one or more chunks.

struct InstanceResponse {
    freeVariableId   @0 :VariableId;
    # A variable ID greater than all IDs allocated by the instance.

    info             @1 :List(KeyValue);
    # Any info that may be useful to the calling parent.

    xResponseContext @2 :AnyPointer;
    # The opaque data given in the request.

    error            @3 :Text;
    # An error message. Null if no error.
}
# Response after the instantiation is complete.


# == Messages for Proving ==

struct AssignmentsRequest {
    instance            @0 :Instance;
    # The same instance parameter must be provided as in the corresponding InstanceRequest.

    witness             @1 :List(KeyValue);
    # Any info that may be useful to the gadget to compute its assignments.

    incomingAssignments @2 :List(Assignment);
    # The values that the parent assigned to `instance.incomingStruct`.

    xChunkContext       @3 :AnyPointer;
    # Opaque data to pass to the chunk handler.

    xResponseContext    @4 :AnyPointer;
    # Opaque data to pass to the return handler.
}
# Request assignments computed from a witness.

struct AssignmentsChunk {
    assignments   @0 :List(Assignment);
    # Assignments computed by the gadgets.

    xChunkContext @1 :AnyPointer;
    # The opaque data given in the request.
}
# Report local and outgoing assignments in one or more chunks.

struct AssignmentsResponse {
    freeVariableId      @0 :VariableId;
    # A variable ID greater than all IDs allocated by the instance.

    info                @1 :List(KeyValue);
    # Any info that may be useful to the calling parent.

    outgoingAssignments @2 :List(Assignment);
    # The values that the gadget assigned to `instance.outgoingStruct`.
    # Intentionally redundant with AssignmentsChunk to allow handling
    # the outgoing variables separately from the bulk of local variables assignments.

    xResponseContext    @3 :AnyPointer;
    # The opaque data given in the request.

    error               @4 :Text;
    # An error message. Null if no error.
}
# Response after all assignments have been reported.


# = RPC Approach =
#
# == Calling with shared memory ==
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
# == Calling by passing messages ==
#
# - Caller sends a serialized request message.
# - Gadget sends one or more serialized chunks, possibly pipelined.
# - Gadget sends a serialized return message.
#
# == Motivation ==
#
# The entity handling chunks might not be the same as the one handling the return.
# In case of gadget composition, the parent can handle the return, while chunks
# are handled directly by some global environment.
# This avoid the needs to route chunks up the callstack.
#

struct GadgetRequest {
    union {
        makeInstance    @0 :InstanceRequest;
        makeAssignments @1 :AssignmentsRequest;
    }
}
# A polymorphic request to implement a basic RPC.

# Methods in Cap'n'proto RPC format.
# Although this might not be the most appropriate approach.

interface Parent {
    constrain @0 (chunk :ConstraintsChunk) -> (ok :Bool);
    assign    @1 (chunk :AssignmentsChunk) -> (ok :Bool);
}

interface Gadget {
    makeInstance    @0 (params :InstanceRequest, caller :Parent) -> (res :InstanceResponse);
    makeAssignments @1 (params :AssignmentsRequest, caller :Parent) -> (res :AssignmentsResponse);
}
