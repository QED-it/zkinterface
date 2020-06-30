// zkInterface - integration helpers.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2020

#ifndef ZKIF_ZKINTERFACE_UTILS_HPP
#define ZKIF_ZKINTERFACE_UTILS_HPP

#include "zkinterface_generated.h"

namespace zkinterface_utils {
    using namespace std;
    using namespace flatbuffers;
    using namespace zkinterface;


// ==== Reading helpers ====

    uoffset_t read_size_prefix(void *buffer);

    const Circuit* read_circuit(char *buffer);

    const ConstraintSystem* read_constraint_system(char *buffer);

    const Witness* read_witness(char *buffer);

    const Command* read_command(char *buffer);

    vector<char>::iterator next_message(vector<char>::iterator it);

    // Find the first message of the requested type in a buffer.
    const Root *find_message(char *buffer, Message type);

    // find_message, with buffer size validation.
    const Root *find_message(char *buffer, uoffset_t buffer_size, Message type);

    // find_message in a vector, with buffer size validation.
    const Root *find_message(vector<char> &buffer, Message type);

    const KeyValue *find_config(const Circuit *circuit, string key);

    string find_config_text(const Circuit *circuit, string key, string default_ = "");

    const Vector<uint8_t> *find_config_data(const Circuit *circuit, string key);

    int64_t find_config_number(const Circuit *circuit, string key, int64_t default_ = 0);

    class MessageNotFoundException : public std::exception {
    public:
        inline const char *what() const throw() {
            return "message of the required type not found";
        }
    };

}
#endif //ZKIF_ZKINTERFACE_UTILS_HPP
