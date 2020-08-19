

## zkInterface Toolkit
### Install
    cargo install zkinterface

### Usage
    zkif help


## Dalek Bulletproofs
### Install
    git clone https://github.com/QED-it/bulletproofs.git
    cd bulletproofs
    cargo install --features yoloproofs --path .

### Prove
    zkif example - | zkif_bulletproofs prove

### Verify
    zkif example - | zkif_bulletproofs verify


## Bellman
### Install
    git clone https://github.com/QED-it/zkinterface-bellman.git
    cd zkinterface-bellman
    cargo install --path .

### Validate / Print
    zkif example - | zkif_bellman print

### Prove
    zkif example - | zkif_bellman prove

### Verify
    (todo)


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
