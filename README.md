# ZK Gadget Standard Interface

A demonstration of the ZK gadget standard.

|                           |                             |
| ------------------------- | --------------------------- |
| `gadget.fbs`              | The gadget interface definition using FlatBuffers |
| `rust/src/gadget_call.rs`      | Example gadget call in Rust |
| `rust/src/gadget_generated.rs` | Generated Rust code         |
| `cpp/gadget.cpp`          | Example gadget in C++       |
| `cpp/gadget_generated.h`  | Generated C++ code          |
| `build.rs`                | Generate Rust and C++ code from gadget.capnp, and compile the C++ |

## Test

In the `rust` directory:

`cargo test`

This will generate and compile Rust and C++ code, and run a test of both sides communicating
through the standard interface.

## Changing the interface

Install the FlatBuffers code generator (`flatc`):

```
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
make
```
