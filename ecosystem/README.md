

## zkInterface Toolkit
### Install
    cargo install zkinterface

### Usage
    zkif help


## Dalek Bulletproofs
### Install
    git clone https://github.com/QED-it/bulletproofs.git
    cd bulletproofs
    cargo +nightly install --path .

### Prove and Verify
    zkif example --field-order=7237005577332262213973186563042994240857116359379907606001950938285454250989 - | \
        zkif_bulletproofs prove
    
    zkif example --field-order=7237005577332262213973186563042994240857116359379907606001950938285454250989 - | \
        zkif_bulletproofs verify

With the field of Ed25519.
Tested with zkinterface 1.3.4, bulletproofs/zkinterface c210e4fe, rustc 1.52.0-nightly 2021-03-07.


## Bellman
### Install
    git clone https://github.com/QED-it/zkinterface-bellman.git
    cd zkinterface-bellman
    cargo +nightly install --path .

### Validate / Print
    zkif example --field-order=52435875175126190479447740508185965837690552500527637822603658699938581184513 - | \
        zkif_bellman print

### Setup, Prove and Verify
    zkif example --field-order=52435875175126190479447740508185965837690552500527637822603658699938581184513 - | \
        zkif_bellman setup
    
    zkif example --field-order=52435875175126190479447740508185965837690552500527637822603658699938581184513 - | \
        zkif_bellman prove
    
    zkif example --field-order=52435875175126190479447740508185965837690552500527637822603658699938581184513 - | \
        zkif_bellman verify

With the field of BLS12-381.
Tested with zkinterface 1.3.4, zkinterface-bellman 1.3.4, rustc 1.52.0-nightly 2021-03-07.

## libSNARK
### Install
    export ZKINTERFACE_LIBSNARK_PATH=$PWD/dist
    PATH=$PATH:$ZKINTERFACE_LIBSNARK_PATH/bin

    git clone https://github.com/QED-it/zkinterface.git
    cd zkinterface/cpp

    ./deps.sh

    cmake -DCMAKE_INSTALL_PREFIX=$ZKINTERFACE_LIBSNARK_PATH . && make install

### Setup
    zkif example - | zkif_snark setup

### Prove
    zkif example - | zkif_snark prove

### Verify
    zkif example - | zkif_snark verify

### Gadgets in Rust
    [dependencies]
        zkinterface = { version = "1.1.3" }
        zkinterface-libsnark = { version = "1.1.3" }

See examples in `libsnark-rust/src/test.rs`

### Gadgets in C / Foreign Function Interface
    CFLAGS="-I $ZKINTERFACE_LIBSNARK_PATH/include -L $ZKINTERFACE_LIBSNARK_PATH/lib -l ff -l gmp -l zkif_gadgetlib"

See API in `gadgetlib.h`

See FFI call in Rust `libsnark-rust/src/gadgetlib.rs`
