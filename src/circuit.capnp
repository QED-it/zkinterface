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
# Variable ID unique within a constraint system.
# Zero is a reserved special value.

using FieldElementLE = Data;
# A field element as a unsigned integer in little-endian bytes.

struct Term {
    variableId  @1 :VariableId;
    # The ID of the variable.

    coefficient @0 :FieldElementLE;
    # A coefficient.
}
# A term in a R1CS row.
# Intended to be sent in sequences.

struct Constraint {
    # (A) * (B) = (C)
    a @0 :List(Term);
    b @1 :List(Term);
    c @2 :List(Term);
}
# A low-level R1CS constraint between variables.
# Targets the generic mechanisms that build circuits.
# Intended to be sent in sequences.

struct AssignedVariable {
    variableId @1 :VariableId;
    # The ID of the variable.

    value      @0 :FieldElementLE;
    # The value to assign.

}
# A low-level assignment to a variable.
# Targets the generic mechanisms that prepare proofs.
# Intended to be sent in sequences.

struct CustomKeyValue {
    key   @0 :Text;
    value @1 :Data;
}
# Generic key-value for miscellaneous attributes.

struct StructuredGadgetInterface {
    union {
        variables @0 :List(VariableId);
        # Allocated variables.

        structs   @1 :List(StructuredGadgetInterface);
        # Or recursive type.
    }

    type          @2 :Text;
    # Standard, conventional, or custom type name.
    # Allows a gadget to support multiple representation.
    # TODO: not necessary?

    name          @3 :Text;

    info          @4 :List(CustomKeyValue);

    #xInternal     @5 :AnyPointer;
    # A space reserved for implementations to track internal information
    # along with the struct. Not part of the protocol.
}
# A high-level structure of variables.
# Define the interface between a gadget and the rest of the circuit.
# In gadget composition, the parent provides these structures to its child.
# A gadget should document what structures it can accept.

struct GadgetInstance {

    gadgetName     @0 :Text;
    # Which gadget to instantiate.
    # Allows a library to provide multiple gadgets.

    parameters     @1 :List(CustomKeyValue);
    # Any parameter that may influence the instance behavior.
    # Parameters can be standard, conventional, or specific to a gadget.

    incomingStruct @2 :StructuredGadgetInterface;
    # Structure of variables that must be assigned by the calling parent.

    outgoingStruct @3 :StructuredGadgetInterface;
    # Structure of variables that must be assigned by the called gadget.
    # There may be no outgoing variables if the gadget represents a pure assertion.

    freeVariableId @4 :VariableId;
    # First free variable ID. The instance can allocate IDs greater or equal.

    #xInternal      @5 :AnyPointer;
    # A space reserved for implementations to track internal information
    # along with the instance. Not part of the protocol.
}
# An instance of a gadget as part of a circuit.


# == Messages for Instantiation ==

struct R1CSRequest {
    instance @0 :GadgetInstance;
    # All details necessary to construct the instance.
}
# Request to build an instance.

struct ConstraintsChunk {
    constraints @0 :List(Constraint);
    # Constraints to add.
}
# Report all constraints in one or more chunks.

struct R1CSResponse {
    freeVariableId @0 :VariableId;
    # A variable ID greater than all IDs allocated by the instance.

    info           @1 :List(CustomKeyValue);
    # Any info that may be useful to the calling parent.

    error          @2 :Text;
    # An error message. Null if no error.
}
# Response after the instantiation is complete.


# == Messages for Proving ==

struct AssignmentsRequest {
    instance            @0 :GadgetInstance;
    # All details necessary to construct the instance.
    # The same instance parameter must be provided as in the corresponding R1CSRequest.

    witness             @1 :List(CustomKeyValue);
    # Any info that may be useful to the gadget to compute its assignments.

    incomingAssignments @2 :List(AssignedVariable);
    # The values that the parent assigned to `instance.incomingStruct`.
}
# Request assignments computed from a witness.

struct AssignmentsChunk {
    assignments @0 :List(AssignedVariable);
    # Assignments computed by the gadgets.
}
# Report local and outgoing assignments in one or more chunks.

struct AssignmentsResponse {
    freeVariableId      @0 :VariableId;
    # A variable ID greater than all IDs allocated by the instance.

    info                @1 :List(CustomKeyValue);
    # Any info that may be useful to the calling parent.

    outgoingAssignments @2 :List(AssignedVariable);
    # The values that the gadget assigned to `instance.outgoingStruct`.
    # Intentionally redundant with AssignmentsChunk to allow handling
    # the outgoing variables separately from the bulk of local variables assignments.

    error               @3 :Text;
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
        makeR1CS        @0 :R1CSRequest;
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
    makeR1CS        @0 (params :R1CSRequest, caller :Parent) -> (res :R1CSResponse);
    makeAssignments @1 (params :AssignmentsRequest, caller :Parent) -> (res :AssignmentsResponse);
}
