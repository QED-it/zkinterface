// zkInterface - integration helpers.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2020

#ifndef ZKINTERFACE_ZKINTERFACE_UTILS_HPP
#define ZKINTERFACE_ZKINTERFACE_UTILS_HPP

#include "zkinterface_generated.h"

namespace zkinterface_utils {
    using namespace std;
    using namespace flatbuffers;
    using namespace zkinterface;


// ==== Reading helpers ====

    uoffset_t read_size_prefix(void *buffer);

    // Find the first message of the requested type in a buffer.
    const Root *find_message(char *buffer, Message type);

    // find_message, with buffer size validation.
    const Root *find_message(char *buffer, uoffset_t buffer_size, Message type);

    // find_message in a vector, with buffer size validation.
    const Root *find_message(vector<char> &buffer, Message type);

    class MessageNotFoundException : public std::exception {
    public:
        inline const char *what() const throw() {
            return "message of the required type not found";
        }
    };

}
#endif //ZKINTERFACE_ZKINTERFACE_UTILS_HPP