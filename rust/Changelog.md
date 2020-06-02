# Version 1.1.0 (2020-06)

Interface definition:
- *(breaking)* Renamed R1CSContraints to ConstraintSystem.
- *(breaking)* Moved r1cs_generation and witness_generation from Circuit to Command.
- Added a dedicated Command message type to help with interoperable execution.
- Added constraint_type to support for both R1CS and arithmetic circuits.

Rust:
- *(breaking)* Moved "writing" helpers to the "owned" modules.
- Added "owned" versions of all message types.
- Added a tool to convert to JSON.
