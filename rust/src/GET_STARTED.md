# Getting Started with zkInterface

This user guide is aimed at implementors of zero-knowledge systems and details how to integrate their systems using zkInterface. For an introduction and more details, see the `zkInterface.pdf` specification in this repository.

This guide uses the zkInterface supporting library for the Rust programming language, and its companion command-line interface (CLI). It focuses on the circuit format supported from zkInterface 2.0.0. For R1CS, see related content in this repository.

zkInterface is a method to communicate a zero-knowledge statement from a statement generator to a proving system. In this guide, we first generate example statements, and we then consume them. These steps can serve as a starting point for a new implementation in a statement generator, or in a proving system, respectively.

## Information Flow

To communicate a statement, three types of information are transmitted:

- A description of computation as a circuit of gates connected through wires.

- A witness used as input to the circuit by the prover side of the proving system.

- Metadata providing additional instructions to the proving system.

The exact structure of this information is specified in a Flatbuffers schema called `zkinterface.fbs` in this repository, along with inline documentation. See the respective structures: GateSystem, Witness, CircuitHeader.

In this guide, the structures are stored in intermediary files for ease and clarity. However, streaming implementations without storage are also supported.

## First step: getting familiar with existing tools

### Install

    git clone https://github.com/QED-it/zkinterface.git
    cd zkinterface
    git checkout gates
    cargo install --path .
    
    zkif help

This will print a list of available commands (your mileage may vary depending on your environment).


### A producer: example generator

The command below generates an example statement. It stores it into files in the working directory (customizable). The profile AC (Arithmetic Circuit) was selected.

    zkif --profile=AC example

    …
    Written ./statement.zkif
    Written ./witness.zkif


### A consumer: validator and simulator

The following command validates that the statement is properly formatted in compliance with the selected profile (Arithmetic Circuit).

It also acts as a simulator in place of a proving system, and reports whether a prover could convince a verifier. That is, it performs the computation described by the circuit and checks whether the witness satisfies the circuit.

    zkif --profile=AC simulate
    
    …
    Loading file ./witness.zkif
    Loading file ./statement.zkif
    
    The statement is COMPLIANT with the profile!
    The statement is TRUE!


### A consumer: human-readable printer

The command below reads the statement and prints a textual representation of it, including colored highlights for readability.

    zkif --profile=AC explain

    …
    Loading file ./witness.zkif
    Loading file ./statement.zkif
    

    FreeVariableId 10
    FieldMaximum 100
    Gate Constant
    Gate InstanceVar
    Gate Witness
    Gate AssertZero
    Gate Add
    Gate Mul
    Profile arithmetic_circuit
    SetInstanceVar wire_4 = 25
    
    wire_1	<- Constant 100
    wire_2	<- Witness
    wire_3	<- Witness
    wire_4	<- InstanceVar
    wire_5	<- wire_2 * wire_2
    wire_6	<- wire_3 * wire_3
    wire_7	<- wire_5 + wire_6
    wire_8	<- wire_1 * wire_4
    wire_9	<- wire_7 + wire_8
    AssertZero wire_9
    
    wire_2	= 3
    wire_3	= 4


### A producer: converter from R1CS

This repository includes a converter that reads a statement encoded in the R1CS profile and produces an equivalent statement in the arithmetic circuit profile. It is available as a Rust function called `r1cs_to_gates(…)`, with example usage in `test_r1cs_to_gates()`. It is not yet wrapped as a CLI command but can easily be made so.


## Second step: implementing a new integration

### Example code.

An easy way to start a new integration is to explore the source code of the library, which is itself called from the CLI commands. The entry points are the functions called `main_…` in the file `src/bin/zkif.rs`.  Additional example code can be found in the `test_…` functions in the directory `src/gates/`.

### Basic API

All information to be transmitted between systems is in data structures formally specified by the Flatbuffers schema. The simplest Rust API available is a straight one-to-one mapping of these structures. In essence:

    pub struct GateSystemOwned {
        pub gates: Vec<GateOwned>,
    }

    type WireId = u64;
    
    pub enum GateOwned {
        Constant(WireId, Vec<u8>),
        InstanceVar(WireId),
        Witness(WireId),
        AssertZero(WireId),
        Add(WireId, WireId, WireId),
        Mul(WireId, WireId, WireId),
    }

A producer can create a `GateSystemOwned` structure and populate its `gates` vector with a number of `GateOwned`, in compliance with the specification.

A consumer can iterate over `GateSystemOwned.gates` and act on the different gate types using, e.g., a `match` construct.

Implementations should expect to produce or receive not one but a stream of these structures in order to process very large statements with limited memory.


### Builder API

An additional circuit builder API is suggested. It may assist with common tasks that arises when building a circuit. The following features are proposed:
- Allocation of unique wire IDs. See `struct Builder`.
- De-duplication of repeated gates. See `struct CachingBuilder`.
- Removal of identity gates. See `struct OptimizingBuilder`.

This API is experimental and expected to evolve or be abandoned depending on user needs and contributions.


### Low-level serialization

It is not necessary to use the above APIs to integrate zkInterface. Any implementation of Flatbuffers can be used directly instead (a custom implementation is doable because the encoding is simple, but that would be a last resort). See https://google.github.io/flatbuffers/ for existing tools. This is the recommended approach for systems written in languages other than Rust.
