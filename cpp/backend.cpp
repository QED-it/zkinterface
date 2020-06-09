#include <fstream>
#include <iterator>
#include <chrono>
#include <libff/common/default_types/ec_pp.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp>
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
    libff::default_ec_pp::init_public_params();

    auto buf = read_file();

    protoboard<FieldT> pb;

    import_zkif iz(pb, "import_zkif");
    iz.load(buf);

    auto begin = chrono::steady_clock::now();

    iz.allocate_variables();
    iz.generate_constraints();
    iz.generate_witness();

    auto end = chrono::steady_clock::now();
    cout << "It took " << chrono::duration_cast<chrono::microseconds>(end - begin).count() << "µs"
         << endl;

    cout << pb.num_inputs() << " public inputs" << endl;
    cout << pb.num_variables() << " variables" << endl;
    cout << pb.num_constraints() << " constraints" << endl;
    cout << "Satisfied: " << (pb.is_satisfied() ? "YES" : "NO") << endl;

    // Setup, prove, verify.
    auto cs = pb.get_constraint_system();
    auto keypair = r1cs_gg_ppzksnark_generator<libff::default_ec_pp>(cs);
    auto proof = r1cs_gg_ppzksnark_prover<libff::default_ec_pp>(keypair.pk, pb.primary_input(), pb.auxiliary_input());
    auto ok = r1cs_gg_ppzksnark_verifier_strong_IC(keypair.vk, pb.primary_input(), proof);
    cout << "Proof verified: " << (ok ? "YES" : "NO") << endl;
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