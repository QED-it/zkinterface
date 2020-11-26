# zkInterface, a standard tool for zero-knowledge interoperability

zkInterface is a standard tool for zero-knowledge interoperability between different ZK DSLs, gadget libraries, and proving systems.
The zkInterface project was born in the [ZKProof](https://zkproof.org/) community.

See a [live demo](https://qed-it.github.io/zkinterface-wasm-demo/).

See the [specification](zkInterface.pdf) and the [serialization format](zkinterface.fbs).

See the Rust implementation: [![docs](https://docs.rs/zkinterface/badge.svg)](https://docs.rs/zkinterface/)

Integration instructions:
- [Dalek Bulletproofs](#Dalek-Bulletproofs)
- [Bellman](#Bellman)
- [libSNARK](#libSNARK)
- [Gadgets](#Gadgets-in-Rust)

## Introduction

![alt text](https://qedit.s3.eu-central-1.amazonaws.com/pictures/zkinterface.png)

*zkInterface* is specification and associated tools for enabling interoperability between implementations of general-purpose zero-knowledge proof systems. It aims to facilitate interoperability between zero knowledge proof implementations, at the level of the low-constraint systems that represent the statements to be proven. Such constraint systems are generated by _frontends_ (e.g., by compilation from higher-level specifications), and are consumed by cryptographic _backends_ which generate and verify the proofs. The goal is to enable decoupling of frontends from backends, allowing application writers to choose the frontend most convenient for their functional and development needs and combine it with the backend that best matches their performance and security needs.

The standard specifies the protocol for communicating constraint systems, for communicating variable assignments (for production of proofs), and for constructing constraint systems out of smaller building blocks (_gadgets_). These are specified using language-agnostic calling conventions and formats, to enable interoperability between different authors, frameworks and languages.

A simple special case is monolithic representation of a whole constraint system and its variable assignments. However, there are a need for more richer and more nuanced forms of interoperability:

* Precisely-specified statement semantics, variable representation and variable mapping
* Witness reduction, from high-level witnesses to variable assignments
* Gadgets interoperability, allowing components of constraint systems to be packaged in reusable and interoperable form
* Procedural interoperability, allowing execution of complex code to facilitate the above

## Current Status

<!-- What we have done, what we supports, and add the table that we have under Implementations -->

### Implementations
__Frontends:__

|                                                           | Circuit Type | Export Circuits | Import Circuits |
| --------------------------------------------------------- | -------------- | --------------- | --------------- |
| [ZoKrates](https://github.com/QED-it/ZoKrates/blob/zkinterface/zokrates_core/src/proof_system/zkinterface.rs) | R1CS | Yes | No |
| [Libsnark](https://github.com/QED-it/zkinterface/tree/master/cpp) | R1CS | Yes | No |
| [Mir r1cs](https://github.com/mir-protocol/r1cs-zkinterface) | R1CS | Yes | No |
| [PySNARK](https://github.com/meilof/pysnark) | R1CS | Yes | No |
| [Bellman](https://github.com/QED-it/zkinterface-bellman) | R1CS | Yes | Yes |

__Backends:__

|                                                           | Proving System |
| --------------------------------------------------------- | -------------- |
| [Bellman](https://github.com/QED-it/zkinterface-bellman) | Groth16            |
| [Dalek](https://github.com/QED-it/bulletproofs/blob/zkinterface/src/r1cs/zkinterface_backend.rs) | Bulletproofs |
| [Libsnark](https://github.com/QED-it/zkinterface/tree/master/cpp) | BCTV14a |


See also the [WebAssembly modules](https://github.com/QED-it/zkinterface-wasm/) used to build the [live demo](https://qed-it.github.io/zkinterface-wasm-demo/).

See the [ecosystem/](ecosystem/) folder for a collection of instructions to install and connect multiple systems.

<!-- ## How to use it

*TODO: Discuss how different stakeholders will use this (frontend authors, backend authors, gadget authors, integrators) and what they should do.* -->

### Repository content

|                           |                             |
| ------------------------- | --------------------------- |
| `zkInterface.pdf`         | The interface specification |
| `zkinterface.fbs`         | The gadget interface definition using FlatBuffers |
| `examples/`               | An example circuit in binary and JSON |
| `rust/`                   | Cargo package `zkinterface`           |
| `rust/src/zkinterface_generated.rs` | Generated Rust code         |
| `rust/src/reading.rs`               | Rust helpers to read messages |
| `rust/src/writing.rs`               | Rust helpers to write messages |
| `rust/src/cpp_gadget.rs`            | Rust helpers to interact with C++ |
| `rust/src/examples.rs`              | Example messages for a simple test circuit |
| `rust/src/gadget_call.rs`           | Example gadget call in Rust |
| `cpp/zkinterface_generated.h`       | Generated C++ code          |
| `cpp/gadget_example.cpp`            | Example gadget in C++       |
| `js/`                               | NPM package `zkinterface`   |
| `js/zkinterface_generated.js`       | Generated JavaScript code   |
| `build.rs`                | Generate Rust and C++ code from zkinterface.fbs, and compile the C++ example |
| `cpp/libsnark_integration.hpp` | Libsnark support            |
| `cpp/libsnark_example.cpp`     | Libsnark gadget example     |

### Testing

In the `rust` directory, run unit tests: 

`cargo test`

The following commands will generate and print a file containing the messages Circuit, R1CSConstraints, and Witness for a toy circuit in `rust/src/examples.rs`:

```
cargo run example  > example.zkif
cargo run explain  < example.zkif
```

### Generated code

Generated C++ and Rust code is included.

For other languages, install the FlatBuffers code generator (`flatc`).
One way is to compile it with the following:

```
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
make
```

Then run:
`flatc --LANGUAGE zkinterface.fbs`

## How to contribute

<!-- define broad goals and some more specific goals -->

- In a frontend, implement a feature to export the circuits or gadgets to zkInterface format.
- In a proving system, support loading circuits from zkInterface buffers or files.

See the implementation guide section in the spec above for more details, and check out the existing implementations below.


## Specification

The [zkInterface specification document](zkInterface.pdf) providers further context on the above, and defines the communication protocol and calling convention between frontends and backends:

* The logical content of messages being exchange.
* The serialization format of the messages (which is based on [FlatBuffers](FlatBuffers) and may be used in-memory, saved or streamed).
* A protocol for building a constraint system from gadget composition.
* Technical recommendations for implementation.

## Limitations

This first revision focuses on non-interactive proof systems (NIZKs) for general statements (i.e., NP relations) represented in the R1CS/QAP-style constraint system representation. Extension to other forms of constraint systems is planned.

The associated code is experimental.

See the [specification document](zkInterface.pdf) for more information about limitations and scope.

## Integration examples

### Dalek Bulletproofs
#### Install
    git clone https://github.com/QED-it/bulletproofs.git
    cd bulletproofs
    cargo install --features yoloproofs --path .

#### Prove
    zkif example - | zkif_bulletproofs prove

#### Verify
    zkif example - | zkif_bulletproofs verify


### Bellman
#### Install
    git clone https://github.com/QED-it/zkinterface-bellman.git
    cd zkinterface-bellman
    cargo install --path .

#### Validate / Print
    zkif example - | zkif_bellman print

#### Prove
    zkif example - | zkif_bellman prove

#### Verify
    (todo)


### libSNARK
#### Install
    export ZKINTERFACE_LIBSNARK_PATH=$PWD/dist
    PATH=$PATH:$ZKINTERFACE_LIBSNARK_PATH/bin

    git clone https://github.com/QED-it/zkinterface.git
    cd zkinterface/cpp

    ./deps.sh

    cmake -DCMAKE_INSTALL_PREFIX=$ZKINTERFACE_LIBSNARK_PATH . && make install

#### Setup
    zkif example - | zkif_snark setup

#### Prove
    zkif example - | zkif_snark prove

#### Verify
    zkif example - | zkif_snark verify
    
### Gadgets in Rust
    [dependencies]
        zkinterface = { version = "1.1.3" }
        zkinterface-libsnark = { version = "1.1.3" }

See examples in `libsnark-rust/src/test.rs`

### Gadgets in C / Foreign Function Interface
    CFLAGS="-I $ZKINTERFACE_LIBSNARK_PATH/include -L $ZKINTERFACE_LIBSNARK_PATH/lib -l ff -l gmp -l zkif_gadgetlib"

See API in `gadgetlib.h`

See FFI call in Rust `libsnark-rust/src/gadgetlib.rs`
