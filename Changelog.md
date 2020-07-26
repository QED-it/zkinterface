# Version v1.1.3, 2020-07, Rust/C++/libSNARK end-to-end integration

Rust:
- *(breaking)* Expanded `*Owned` structures (`KeyValueOwned`, rename `write_into`, serialization of `ConstraintSystemOwned`).
- *(breaking)* Generic code improvements (Rust 2018, common `Result`, naming, modules re-exports).
- Consolidated all tools into one CLI called `zkif`.
- Available commands:
    - JSON converters (`json` and `pretty`).
    - Human-readable decoder called `explain`.
    - Example circuit generator.
    - Statistics for cost estimation (`stats`).
    - Support for byte streams to pipe with other CLIs (`cat` command and special `-` filename meaning stdin/stdout).
- Established a convention for easy file-based usage. The tools accept a workspace directory where `.zkif` files are organized.
- Structures to construct ZK statements. See `statement.rs / StatementBuilder`.
- A trait `GadgetCallbacks` and basic implementations to interact with gadgets (e.g. libSNARK below).
- Fixed a bug on empty variables array.

libsnark-rust:
- Support zero-copy mode using the generic `GadgetCallbacks` trait.
- Tests with a TinyRAM gadget and `StatementBuilder`.

C++:
- Support for multiple messages of each type.
- Simplify the `zkif_snark` CLI and work well with the workspace and pipe modes of the `zkif` CLI.


# Version v1.1.2, 2020-06, libsnark and gadgetlib for Rust

Rust:
- Support for `Command` messages.
- Remove C++ integration from the `zkinterface` crate.
- Introduce a dedicated `zkinterface-libsnark` crate.
- CMake integration in `build.rs`.
- C++ libsnark wrapper. See `gadgetlib_call_gadget()`.
- A test as an example. See `test_cpp_gadget()`.

C++:
- Organize built artifacts: `zkif_gadgetlib` library, headers, dependencies, CLI wrappers.
- Clean up and organize code.
- Fix and clarify the translation between libsnark variables and zkInterface IDs. See class `VarIdConverter`.
- Demonstrate a working gadget: a TinyRAM ALU operation.


# Version v1.1.1, 2020-06, libsnark and gadgetlib

Interface definition:
- Added fields for different data types in `KeyValue`.

C++:
- Added a CLI to setup/prove/verify using libsnark (see `cpp/backend.cpp`).
- Added a library and CLI to call gadgets from gadgetlib TinyRAM (see `cpp/gadgetlib.cpp`).


# Version 1.1.0, 2020-06, Simplifications

Interface definition:
- *(breaking)* Moved r1cs_generation and witness_generation from Circuit to Command.
- *(breaking)* The position of Circuit `field_maximum` and `configuration` changed (unused afaik.).
- Renamed R1CSContraints to ConstraintSystem.
- Added a dedicated Command message type to help with interoperable execution.
- Added an example file in binary and JSON.

Rust:
- *(breaking)* Moved "writing" helpers to the "owned" modules.
- Added "owned" versions of all message types.
- Added a tool to convert to JSON. (`src/bin/zkif_json.rs`)
- Added a tool to print basic statistics about a circuit. (`src/bin/stats.rs`)
