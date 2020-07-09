#!/bin/sh
set -e

alias zkif="cargo run --manifest-path ../rust/Cargo.toml"
zkif explain  < ../examples/example.zkif
zkif stats    < ../examples/example.zkif

cmake -DCMAKE_INSTALL_PREFIX=dist . && make install

pushd libsnark-rust
ZKINTERFACE_LIBSNARK_PATH=../dist cargo test
popd

MAIN=libsnark-rust/local/test_statement_public_main.zkif
CONS=libsnark-rust/local/test_statement_public_constraints.zkif
WITN=libsnark-rust/local/test_statement_private_witness.zkif

dist/bin/zkif_snark validate $MAIN $CONS $WITN
dist/bin/zkif_snark setup $MAIN $CONS
dist/bin/zkif_snark prove $MAIN $WITN
dist/bin/zkif_snark verify $MAIN

