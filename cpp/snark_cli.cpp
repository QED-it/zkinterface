#include "libsnark_importer.hpp"

#include <chrono>
#include <fstream>
#include <iterator>
#include <libff/common/default_types/ec_pp.hpp>
#include <libsnark/zk_proof_systems/ppzksnark/r1cs_gg_ppzksnark/r1cs_gg_ppzksnark.hpp>

using namespace std;
using namespace libsnark_converters;
using namespace libsnark_importer;

vector<char> read_files(vector<string> zkifPaths) {
    vector<char> buf;

    for (auto it = zkifPaths.begin(); it != zkifPaths.end(); it++) {
        ifstream zkifFile(*it, ios::binary);
        buf.insert(buf.end(), (istreambuf_iterator<char>(zkifFile)),
                   istreambuf_iterator<char>());

        if (zkifFile) {
            cerr << "Read messages from files " << *it << endl;
        } else {
            throw "Error: could not open file";
        }
    }

    return buf;
}

static bool endsWith(const std::string &str, const std::string &suffix) {
    return str.size() >= suffix.size() && 0 == str.compare(str.size() - suffix.size(), suffix.size(), suffix);
}

protoboard<FieldT> load_protoboard(bool with_constraints, bool with_witness) {
    // Read stdin.
    std::istreambuf_iterator<char> begin(std::cin), end;
    vector<char> buf(begin, end);

    protoboard<FieldT> pb;
    import_zkif iz(pb, "import_zkif");
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
    void print(string action) {
        auto dur = chrono::steady_clock::now() - begin;
        cerr << "ZKPROOF_BENCHMARK: {"
             << R"("system": "libsnark", )"
             << R"("action": ")" << action << "\", "
             << R"("iterations": 1, )"
             << R"("microseconds": )"
             << chrono::duration_cast<chrono::microseconds>(dur).count()
             << "}" << endl;
    }
};

void run(string action, string prefix) {
    CurveT::init_public_params();
    libff::inhibit_profiling_info = true;

    if (action == "validate") {
        auto pb = load_protoboard(true, true);
        print_protoboard(pb);
        cerr << "Satisfied: " << (pb.is_satisfied() ? "YES" : "NO") << endl;

    } else if (action == "setup") {
        auto pb = load_protoboard(true, false);

        auto keypair = r1cs_gg_ppzksnark_generator<CurveT>(pb.get_constraint_system());

        ofstream(prefix + "libsnark-pk", ios::binary) << keypair.pk;
        ofstream(prefix + "libsnark-vk", ios::binary) << keypair.vk;

    } else if (action == "prove") {
        auto pb = load_protoboard(false, true);

        r1cs_gg_ppzksnark_proving_key<CurveT> pk;
        ifstream(prefix + "libsnark-pk", ios::binary) >> pk;
        Benchmark bench;

        auto proof = r1cs_gg_ppzksnark_prover<CurveT>(pk, pb.primary_input(), pb.auxiliary_input());

        bench.print(action);
        ofstream(prefix + "libsnark-proof", ios::binary) << proof;

    } else if (action == "verify") {
        auto pb = load_protoboard(false, false);

        r1cs_gg_ppzksnark_verification_key<CurveT> vk;
        ifstream(prefix + "libsnark-vk", ios::binary) >> vk;

        r1cs_gg_ppzksnark_proof<CurveT> proof;
        ifstream(prefix + "libsnark-proof", ios::binary) >> proof;
        Benchmark bench;

        auto ok = r1cs_gg_ppzksnark_verifier_strong_IC(vk, pb.primary_input(), proof);

        bench.print(action);
        cout << endl << "Proof verified: " << (ok ? "YES" : "NO") << endl;
    }
}

static const char USAGE[] =
        R"(libsnark prover.

    Usage:
      snark validate <name>
      snark setup    <name>
      snark prove    <name>
      snark verify   <name>

    The input circuit and witness is read from stdin in zkInterface format.
    The filenames of keys and proofs are derived from the name argument.
)";

int main(int argc, const char **argv) {
    if (argc < 2) {
        cerr << USAGE << endl;
        return 1;
    }

    string prefix = (argc == 2) ? "" : argv[2];

    try {
        run(string(argv[1]), prefix);
        return 0;
    } catch (const char *msg) {
        cerr << msg << endl;
        return 2;
    }
}