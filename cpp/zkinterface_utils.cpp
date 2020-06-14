// zkInterface - integration helpers.
//
// @author Aurélien Nicolas <info@nau.re> for QED-it.com
// @date 2020

#include "zkinterface_utils.hpp"

namespace zkinterface_utils {

    const uoffset_t UNKNOWN_BUFFER_SIZE = uoffset_t(4) * 1000 * 1000 * 1000; // 4G.

    uoffset_t read_size_prefix(void *buffer) {
        uoffset_t message_length = ReadScalar<uoffset_t>(buffer);
        return sizeof(uoffset_t) + message_length;
    }

    const Root *find_message(char *buffer, uoffset_t buffer_size, Message type) {
        auto offset = 0;

        while (offset + sizeof(uoffset_t) * 2 <= buffer_size) {
            auto current = buffer + offset;

            auto size = read_size_prefix(current);
            if (offset + size > buffer_size) {
                throw "invalid offset";
            }

            auto root = GetSizePrefixedRoot(current);

            if (root->message_type() == type) {
                return root; // Found.
            }

            offset += size;
        }

        throw MessageNotFoundException();
    }

    const Root *find_message(char *buffer, Message type) {
        return find_message(buffer, UNKNOWN_BUFFER_SIZE, type);
    }

    const Root *find_message(vector<char> &buffer, Message type) {
        return find_message(buffer.data(), buffer.size(), type);
    }

} // namespace zkinterface_utils