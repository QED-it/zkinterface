# ZK Gadget Standard Interface

A demonstration of the ZK gadget standard.

|                           |                             |
| ------------------------- | --------------------------- |
| `src/gadget.fbs`          | The gadget interface definition using FlatBuffers |
| `src/gadget_call.rs`      | Example gadget call in Rust |
| `src/gadget_generated.rs` | Generated Rust code         |
| `cpp/gadget.cpp`          | Example gadget in C++       |
| `cpp/gadget_generated.h`  | Generated C++ code          |
| `build.rs`                | Generate Rust and C++ code from gadget.capnp, and compile the C++ |

## Test

`cargo test`

## Changing the interface

Install the FlatBuffers code generator (`flatc`):

```
git clone https://github.com/google/flatbuffers.git
cd flatbuffers
cmake -G "Unix Makefiles" -DCMAKE_BUILD_TYPE=Release
make
```
