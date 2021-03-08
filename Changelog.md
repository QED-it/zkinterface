# Version v1.3.4, 2021-02, example --field-order

Rust:
- Generate example circuit with a configurable field order in the header: `zkif example --field-order 101`

# Version v1.3.3, 2021-02, Metrics

Rust:
- Circuit generator for benchmarking (`zkif metrics`).
- More documentation.

# Version v1.3.2, 2020-10, Improve simulator

Rust:
- Fix a bug when using simulate from stdin.
- Simplify and unify the interface of Validator and Simulator.
- Method `CircuitHeader.list_witness_ids()`.

# Version v1.3.1, 2020-10, Rust streaming

Rust:
- WorkspaceSink creates multiple constraints files instead of a large one.
- Example code to read in chunked/streamed mode.
- Consolidate the workspace file discovery into its module.

# Version v1.3.0, 2020-10, Rust streaming

Rust:
- Unified Workspace reader with streaming mode.
- Supports reading from stdin stream.
- Supports reading chunk by chunk from unlimited files.
- CLI stats, validate, and simulate work in streaming mode.
- *(breaking)* Renamed FileSink to WorkspaceSink.


# Version v1.2.1, 2020-10

Rust:
- Make CLI tools available as a library.
- Fix a bug in the simulator.
- Start organizing workspace features in a module.
- clean_workspace() function.

# Version v1.2.0, 2020-09, Rust refactor

FlatBuffers schema (binary encoding remains unchanged):
- Renamed `Circuit` to `CircuitHeader`.
- Renamed `connections` to `instance_variables`.

CLI:
- `to-json` - Convert messages into one-line compact JSON.
- `to-yaml` - Convert messages into readable YAML.
- `validate` - Validate the format of messages, from the point of view of the verifier.
- `simulate` - Validate the format of messages and verify that the statement is true, from the point of view of the prover.

Rust:
- *(breaking)* Moved "owned" structures to a `structs` module and removed the "Owned" suffix.
- *(breaking)* Organized code into `producers` and `consumers` modules.
- *(breaking)* Renamed `reading::Messages` into `consumers::reader::Reader`.
- Renamed occurrences of `circuit` or `main` to `header`.
- Simplified `StatementBuilder`. Renamed occurrences of "Store" to "Sink".
- Moved helpers for external gadget calls into a `gadget_caller` module.
- Using the StructOpt library for the CLI.


# Version v1.1.4, 2020-08, Rust fixes

Rust:
- Default trait, build() method, doctests for *Owned structures.
- Fix all potential divide-by-zero.


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
