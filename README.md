# ZK Gadget Standard Interface

zkInterface is a standard tool for zero-knowledge interoperability.

|                           |                             |
| ------------------------- | --------------------------- |
| `zkInterface.pdf`         | The interface specification |
| `gadget.fbs`              | The gadget interface definition using FlatBuffers |
| `rust/src/gadget_call.rs`      | Example gadget call in Rust |
| `rust/src/gadget_generated.rs` | Generated Rust code         |
| `cpp/gadget.cpp`          | Example gadget in C++       |
| `cpp/gadget_generated.h`  | Generated C++ code          |
| `build.rs`                | Generate Rust and C++ code from gadget.capnp, and compile the C++ |
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
`flatc --LANGUAGE gadget.fbs`
