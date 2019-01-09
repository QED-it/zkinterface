# gadget_standard_rust

A demonstration of the ZK gadget standard.

|                          |                          |
| ------------------------ | ------------------------ |
| `src/gadget.capnp`       | The interface definition |
| `src/`                   | A Rust implementation    |
| `src/gadget_capnp.rs`    | Generated Rust code      |
| `cpp/`                   | A C++ implementation     |
| `cpp/gadget.capnp.h,c++` | Generated C++ code       |
| `build.rs`               | Generate Rust and C++ code from gadget.capnp, and compile the C++ |

## Install

Install Cap'n'Proto: https://capnproto.org/install.html

Then run: `cargo test`
