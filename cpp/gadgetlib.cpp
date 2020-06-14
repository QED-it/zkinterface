#include <iostream>
#include "gadgetlib_example.hpp"

using namespace std;

void run(string action, string zkif_out_path) {
    if (action == "constraints") {
        r1cs_request(nullptr, nullptr, nullptr, nullptr, nullptr);
    } else if (action == "witness") {
        assignments_request(nullptr, nullptr, nullptr, nullptr, nullptr);
    }
}

static const char USAGE[] =
        R"(libsnark gadget lib.

    Usage:
      gadgetlib constraints <zkinterface_output_file>
      gadgetlib witness <zkinterface_output_file>
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