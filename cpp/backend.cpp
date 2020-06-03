
#include "libsnark_zkif_import.cpp"
#include <fstream>
#include <iterator>
#include <chrono>

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
    // TODO: init curves.

    auto buf = read_file();

    protoboard<FieldT> pb;

    import_zkif iz;
    iz.load(buf);

    auto begin = chrono::steady_clock::now();

    iz.get_circuit();
    iz.generate_constraints();
    iz.generate_witness();

    auto end = chrono::steady_clock::now();
    cout << "It took " << chrono::duration_cast<chrono::microseconds>(end - begin).count() << "µs"
         << endl;
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