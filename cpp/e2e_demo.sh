#!/bin/sh
set -ex

# Build.
cmake -DCMAKE_INSTALL_PREFIX=dist . && make install

# Find the tools.
alias zkif="cargo run --manifest-path ../rust/Cargo.toml"
alias zkif_snark="dist/bin/zkif_snark"

# Generate a test statement.
pushd libsnark-rust
ZKINTERFACE_LIBSNARK_PATH=../dist cargo test
popd

# Look at the files.
DIR=libsnark-rust/local/test_statement
MAIN=$DIR/main.zkif
CONS=$DIR/constraints.zkif
WITN=$DIR/witness.zkif
NAME=$DIR/snark

zkif explain $MAIN $CONS $WITN

# Prove and verify.
cat $MAIN $CONS $WITN | zkif_snark validate $NAME
cat $MAIN $CONS       | zkif_snark setup    $NAME
cat $MAIN $WITN       | zkif_snark prove    $NAME
cat $MAIN             | zkif_snark verify   $NAME

