#include <fstream>
#include <iterator>
#include <chrono>
#include <libff/common/default_types/ec_pp.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp>
#include "libsnark_zkif_import.cpp"

using namespace std;
using namespace zkinterface_libsnark;

vector<char> read_file(string zkifPath) {
    ifstream zkifFile(zkifPath, ios::binary);
    vector<char> buf((istreambuf_iterator<char>(zkifFile)),
                     istreambuf_iterator<char>());

    if (zkifFile) {
        cerr << "Read messages from file " << zkifPath << endl;
    } else {
        throw "Error: could not open file";
    }

    return buf;
}

protoboard<FieldT> load_protoboard(string zkifPath, bool with_constraints, bool with_witness) {
    CurveT::init_public_params();
    libff::inhibit_profiling_info = true;

    protoboard<FieldT> pb;
    import_zkif iz(pb, "import_zkif");

    auto buf = read_file(zkifPath);
    iz.load(buf);
    iz.allocate_variables();
    if (with_constraints) iz.generate_constraints();
    if (with_witness) iz.generate_witness();
    return pb;
}

void print_protoboard(protoboard<FieldT> &pb) {
    cerr << pb.num_inputs() << " public inputs" << endl;
    cerr << pb.num_variables() << " variables" << endl;
    cerr << pb.num_constraints() << " constraints" << endl;
}

class Benchmark {
    chrono::steady_clock::time_point begin = chrono::steady_clock::now();
public:
    void print() {
        auto dur = chrono::steady_clock::now() - begin;
        cerr << "ZKPROOF_BENCHMARK: {"
             << "\"iterations\":1, "
             << "\"microseconds\":"
             << chrono::duration_cast<chrono::microseconds>(dur).count()
             << "}" << endl;
    }
};

void run(string action, string zkifPath) {
    if (action == "validate") {
        auto pb = load_protoboard(zkifPath, true, true);
        print_protoboard(pb);
        cerr << "Satisfied: " << (pb.is_satisfied() ? "YES" : "NO") << endl;

    } else if (action == "setup") {
        auto pb = load_protoboard(zkifPath, true, false);

        auto keypair = r1cs_gg_ppzksnark_generator<CurveT>(pb.get_constraint_system());

        ofstream(zkifPath + ".pk", ios::binary) << keypair.pk;
        ofstream(zkifPath + ".vk", ios::binary) << keypair.vk;

    } else if (action == "prove") {
        auto pb = load_protoboard(zkifPath, false, true);

        r1cs_gg_ppzksnark_proving_key<CurveT> pk;
        ifstream(zkifPath + ".pk", ios::binary) >> pk;
        Benchmark bench;

        auto proof = r1cs_gg_ppzksnark_prover<CurveT>(pk, pb.primary_input(), pb.auxiliary_input());

        bench.print();
        ofstream(zkifPath + ".proof", ios::binary) << proof;

    } else if (action == "verify") {
        auto pb = load_protoboard(zkifPath, false, false);

        r1cs_gg_ppzksnark_verification_key<CurveT> vk;
        ifstream(zkifPath + ".vk", ios::binary) >> vk;

        r1cs_gg_ppzksnark_proof<CurveT> proof;
        ifstream(zkifPath + ".proof", ios::binary) >> proof;
        Benchmark bench;

        auto ok = r1cs_gg_ppzksnark_verifier_strong_IC(vk, pb.primary_input(), proof);

        bench.print();
        cout << endl << "Proof verified: " << (ok ? "YES" : "NO") << endl;
    }
}

static const char USAGE[] =
        R"(libsnark prover.

    Usage:
      snark validate <zkinterface_file>
      snark setup <zkinterface_file>
      snark prove <zkinterface_file>
      snark verify <zkinterface_file>
)";

int main(int argc, const char **argv) {

    if (argc < 3) {
        cerr << USAGE << endl;
        return 1;
    }

    try {
        run(string(argv[1]), string(argv[2]));
        return 0;
    } catch (const char *msg) {
        cerr << msg << endl;
        return 2;
    }
}