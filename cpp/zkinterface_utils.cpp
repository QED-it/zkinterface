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

    const Circuit* read_circuit(char *buffer) {
        return GetSizePrefixedRoot(buffer)->message_as_Circuit();
    }

    const ConstraintSystem* read_constraint_system(char *buffer) {
        return GetSizePrefixedRoot(buffer)->message_as_ConstraintSystem();
    }

    const Witness* read_witness(char *buffer) {
        return GetSizePrefixedRoot(buffer)->message_as_Witness();
    }

    const Command* read_command(char *buffer) {
        return GetSizePrefixedRoot(buffer)->message_as_Command();
    }

    vector<char>::iterator next_message(vector<char>::iterator it) {
        uoffset_t size = read_size_prefix(&(*it));
        return it + size;
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

    const KeyValue *find_config(const Circuit *circuit, string key) {
        auto config = circuit->configuration();
        if (config != nullptr) {
            for (auto kv = config->begin(); kv != config->end(); kv++) {
                if (kv->key()->str() == key) {
                    return *kv;
                }
            }
        }
        return nullptr; // Not found.
    }

    string find_config_text(const Circuit *circuit, string key, string default_) {
        auto kv = find_config(circuit, key);
        if (kv != nullptr && kv->text() != nullptr) {
            return kv->text()->str();
        } else {
            return default_;
        }
    }

    const Vector<uint8_t> *find_config_data(const Circuit *circuit, string key) {
        auto kv = find_config(circuit, key);
        if (kv != nullptr && kv->text() != nullptr) {
            return kv->data();
        } else {
            return nullptr;
        }
    }

    int64_t find_config_number(const Circuit *circuit, string key, int64_t default_) {
        auto kv = find_config(circuit, key);
        if (kv != nullptr && kv->text() != nullptr) {
            return kv->number();
        } else {
            return default_;
        }
    }

} // namespace zkinterface_utils