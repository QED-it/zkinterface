# Version 1.1.0, 2020-06, Simplifications

Interface definition:
- *(breaking)* Renamed R1CSContraints to ConstraintSystem.
- *(breaking)* Moved r1cs_generation and witness_generation from Circuit to Command.
- Added a dedicated Command message type to help with interoperable execution.
- Added an example file in binary and JSON.

Rust:
- *(breaking)* Moved "writing" helpers to the "owned" modules.
- Added "owned" versions of all message types.
- Added a tool to convert to JSON.
