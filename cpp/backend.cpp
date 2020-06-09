#include <fstream>
#include <iterator>
#include <chrono>
#include <libff/common/default_types/ec_pp.hpp>
#include "libsnark_zkif_import.cpp"

using namespace zkinterface_libsnark;

vector<char> read_file() {
    string testPath = "../examples/example.zkif";

    ifstream testFile(testPath, ios::binary);
    vector<char> buf((istreambuf_iterator<char>(testFile)),
                     istreambuf_iterator<char>());

    if (testFile) {
        cerr << "Read messages from file." << endl;
    } else {
        throw "Error: could not open file";
    }

    return buf;
}

void run() {
    libff::alt_bn128_pp::init_public_params();

    auto buf = read_file();

    protoboard<FieldT> pb;

    import_zkif iz(pb, "import_zkif");
    iz.load(buf);

    auto begin = chrono::steady_clock::now();

    iz.allocate_variables();
    iz.generate_constraints();
    iz.generate_witness();

    cout << pb.get_constraint_system() << endl;
    for(auto i=0;i<6;i++){
        cout << i << ": " << pb.val(i).as_ulong() << endl;
    }

    auto end = chrono::steady_clock::now();
    cout << "It took " << chrono::duration_cast<chrono::microseconds>(end - begin).count() << "µs"
         << endl;

    cout << pb.num_inputs() << " public inputs" << endl;
    cout << pb.num_variables() << " variables" << endl;
    cout << pb.num_constraints() << " constraints" << endl;
    cout << "Satisfied: " << (pb.is_satisfied() ? "YES" : "NO") << endl;
}

int main(int, char **) {
    try {
        run();
        return 0;
    } catch (const char *msg) {
        cerr << msg << endl;
        return 1;
    }
}