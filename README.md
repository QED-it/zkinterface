# ZK Gadget Standard Interface

zkInterface is a standard tool for zero-knowledge interoperability.

[The spec can be found here](zkInterface.pdf).

## How to contribute 
- In a frontend, implement a feature to export the circuits or gadgets to ZkInterface format.
- In a proving system, support loading circuits from ZkInterface buffers or files.

See the implementation guide section in the spec above for more details, and check out the existing implementations below.

## Implementations

|                                                           | Proving System | Export Circuits | Import Circuits |
| --------------------------------------------------------- | -------------- | --------------- | --------------- |
| [Bellman](https://github.com/QED-it/zkinterface-bellman) (Groth16) | Yes            | No              | Yes             |
| [Dalek](https://github.com/QED-it/bulletproofs/blob/zkinterface/src/r1cs/zkinterface_backend.rs) (Bulletproofs) | Yes | No | No |
| [ZoKrates](https://github.com/QED-it/ZoKrates/blob/zkinterface/zokrates_core/src/proof_system/zkinterface.rs) | - | Yes | No |
| [Libsnark](https://github.com/QED-it/zkinterface/tree/master/cpp) | PGHR | Yes | No |

See also the [WebAssembly modules](https://github.com/QED-it/zkinterface-wasm/) and the [live demo](https://qed-it.github.io/zkinterface-wasm-demo/).

## Structure of this repo

|                           |                             |
| ------------------------- | --------------------------- |
| `zkInterface.pdf`         | The interface specification |
| `zkinterface.fbs`         | The gadget interface definition using FlatBuffers |
| `rust/src/zkinterface_generated.rs` | Generated Rust code         |
| `rust/src/reading.rs`               | Rust helpers to read messages |
| `rust/src/writing.rs`               | Rust helpers to write messages |
| `rust/src/cpp_gadget.rs`            | Rust helpers to interact with C++ |
| `rust/src/examples.rs`              | Example messages for a simple test circuit |
| `cpp/zkinterface_generated.h`       | Generated C++ code          |
| `cpp/gadget_example.cpp`            | Example gadget in C++       |
| `build.rs`                | Generate Rust and C++ code from zkinterface.fbs, and compile the C++ example |
| `cpp/libsnark_integration.hpp` | Libsnark support            |
| `cpp/libsnark_example.cpp`     | Libsnark gadget example     |

## Test

In the `rust` directory:

`cargo test`

This will generate and compile Rust and C++ code, and run a test of both sides communicating
through the standard interface.

## Generated code

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
