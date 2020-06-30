/**
 * Import a zkInterface circuit into a protoboard.
 */

#ifndef ZKIF_LIBSNARK_IMPORTER_HPP
#define ZKIF_LIBSNARK_IMPORTER_HPP

#include "libsnark_converters.hpp"

#include "libsnark/gadgetlib1/gadget.hpp"
#include "libsnark/gadgetlib1/protoboard.hpp"


namespace libsnark_importer {
    using namespace zkinterface;
    using namespace libsnark_converters;
    using std::string;

    class import_zkif : public gadget<FieldT> {
       vector<char> buffer;

    public:
        import_zkif(Protoboard &pb, const string &annotation_prefix);

        Protoboard *get_pb();

        void load(vector<char> &buf);

        const Circuit *get_circuit();

        const ConstraintSystem *get_constraints();

        const Witness *get_witness();

        void allocate_variables();

        void generate_constraints();

        void generate_witness();
    };

} // namespace libsnark_importer

#endif // ZKIF_LIBSNARK_IMPORTER_HPP