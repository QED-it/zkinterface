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