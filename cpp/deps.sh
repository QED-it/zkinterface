
# Latest.
git clone https://github.com/scipr-lab/libsnark.git
cd libsnark
git checkout 477c9dfd07b280e42369f82f89c08416319e24ae
git submodule init && git submodule update
mkdir build && cd build && cmake  ..
make
sudo make install

# Deprecated.
#sudo apt-get install build-essential git libgmp3-dev libprocps4-dev libgtest-dev python-markdown libboost-all-dev libssl-dev
#git clone https://github.com/scipr-lab/libsnark.git
#cd libsnark
#git checkout deprecated-master
#./prepare-depends.sh
#make

# MacOS
#port install gmp openssl boost
#cmake cmake -DWITH_PROCPS=OFF -DWITH_SUPERCOP=OFF -DCURVE=ALT_BN128 .
#make