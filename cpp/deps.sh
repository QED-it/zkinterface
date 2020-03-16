
git clone https://github.com/scipr-lab/libsnark.git
cd libsnark
git checkout 477c9dfd07b280e42369f82f89c08416319e24ae
git submodule init && git submodule update
mkdir build && cd build && cmake -DWITH_PROCPS=OFF ..
make

# MacOS
#port install gmp openssl boost

# Old way.
#wget https://github.com/scipr-lab/libsnark/archive/477c9dfd07b280e42369f82f89c08416319e24ae.zip
#mv 477c9dfd07b280e42369f82f89c08416319e24ae.zip libsnark.zip
#unzip libsnark.zip
#rm libsnark.zip
#mv libsnark-477c9dfd07b280e42369f82f89c08416319e24ae libsnark
#cd libsnark
