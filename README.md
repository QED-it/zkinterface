# ZK Gadget Standard Interface

zkInterface is a standard tool for zero-knowledge interoperability.

[The spec can be found here](zkInterface.pdf).

|                           |                             |
| ------------------------- | --------------------------- |
| `zkInterface.pdf`         | The interface specification |
| `zkinterface.fbs`         | The gadget interface definition using FlatBuffers |
| `rust/src/zkinterface_generated.rs` | Generated Rust code         |
| `rust/src/gadget_call.rs`           | Example gadget call in Rust |
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
