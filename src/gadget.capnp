@0xa55ec1c0b97af6aa;

## Types

using VariableId = UInt64;
# Variable ID unique within a constraint system.
# Zero is a reserved special value.

struct FieldElementRepresentation {
    name @0 :Text;
    # The well-known name of the representation.

    size @1 :UInt32;
    # The size of the representation of an element in bytes.
}
# Description of the representation or encoding of field elements.
# If omitted, use a default representation:
# name = "little-endian"
# size = 32 bytes

struct Terms {
    variableIds    @0 :List(VariableId);

    coefficients   @1 :Data;
    # Contiguous coefficient representations
    # in the same order as variableIds.
}
# Terms in a R1CS vector.

struct Constraint {
    # (A) * (B) = (C)
    a @0 :Terms;
    b @1 :Terms;
    c @2 :Terms;
}
# A low-level R1CS constraint between variables.
# Targets the generic mechanisms that build circuits.
# Intended to be sent in sequences.

struct AssignedVariables {
    variableIds @0 :List(VariableId);

    elements    @1 :Data;
    # Contiguous element representations
    # in the same order as variableIds.
}
# Low-level assignments to variables.
# Targets the generic mechanisms that prepare proofs.

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

    name          @2 :Text;

    info          @3 :List(CustomKeyValue);
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
}
# An instance of a gadget as part of a circuit.


# == Messages for Instantiation ==

struct R1CSRequest {
    instance @0 :GadgetInstance;
    # All details necessary to construct the instance.
    # The same instance parameter must be provided in the corresponding AssignmentsRequest.
}
# Request to build an instance.

struct R1CSChunk {
    constraints    @0 :List(Constraint);
    # Constraints to add.

    representation @1 :FieldElementRepresentation;
    # The representation used for the constraints.
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

    incomingAssignments @2 :AssignedVariables;
    # The values that the parent assigned to `instance.incomingStruct`.

    representation      @3 :FieldElementRepresentation;
    # The representation used for the incomingAssignments.
}
# Request assignments computed from a witness.

struct AssignmentsChunk {
    assignedVariables @0 :AssignedVariables;
    # Assignments computed by the gadgets.

    representation    @1 :FieldElementRepresentation;
    # The representation used for the assignments.
}
# Report local and outgoing assignments in one or more chunks.

struct AssignmentsResponse {
    freeVariableId      @0 :VariableId;
    # A variable ID greater than all IDs allocated by the instance.

    info                @1 :List(CustomKeyValue);
    # Any info that may be useful to the calling parent.

    outgoingAssignments @2 :AssignedVariables;
    # The values that the gadget assigned to `instance.outgoingStruct`.
    # Intentionally redundant with AssignmentsChunk to allow handling
    # the outgoing variables separately from the bulk of local variables assignments.

    representation      @3 :FieldElementRepresentation;
    # The representation used for the outgoingAssignments.

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
        makeR1CS        @0 :R1CSRequest;
        makeAssignments @1 :AssignmentsRequest;
    }
}
# A polymorphic request to implement a basic RPC.

# Methods in Cap'n'proto RPC format.
# Although this might not be the most appropriate approach.

interface Parent {
    constrain @0 (chunk :R1CSChunk) -> (ok :Bool);
    assign    @1 (chunk :AssignmentsChunk) -> (ok :Bool);
}

interface Gadget {
    makeR1CS        @0 (params :R1CSRequest, caller :Parent) -> (res :R1CSResponse);
    makeAssignments @1 (params :AssignmentsRequest, caller :Parent) -> (res :AssignmentsResponse);
}
