
#include "libsnark_zkif_import.cpp"
#include <fstream>
#include <iterator>

using namespace zkinterface_libsnark;

void run() {
    string testPath = "../test_messages/all.zkif";

    ifstream testFile(testPath, ios::binary);
    vector<char> buf((istreambuf_iterator<char>(testFile)),
                     istreambuf_iterator<char>());

    if (testFile) {
        cerr << "Read messages from file." << endl;
    } else {
        throw "Error: could not open file";
    }

    import_zkif iz;
    iz.load(buf);

    iz.get_circuit();

    iz.generate_constraints();
    iz.generate_witness();
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