var zkif = require('./zkinterface');
var log = console.log;


function write_example_circuit() {
    var builder = new zkif.Builder();

    zkif.Circuit.startCircuit(builder);
    zkif.Circuit.addFreeVariableId(builder, zkif.toId(3)); // ID 0, 1, 2.
    var circuit = zkif.Circuit.endCircuit(builder);

    return zkif.finishMessage(builder, zkif.Message.Circuit, circuit);
}


function write_example_witness() {
    var builder = new zkif.Builder();

    var ids = zkif.Variables.createVariableIdsVector(builder, [
        zkif.toId(0),
        zkif.toId(1),
        zkif.toId(2),
    ]);

    // The values are concatenated and must all have the same size (here 4 bytes as an example).
    var values = zkif.Variables.createValuesVector(builder, [
        1,   0,  0,  0, // Value One for ID 0.
        10, 11, 12, 13, // Value for ID 1.
        20, 21, 22, 24, // Value for ID 2.
    ]);

    zkif.Variables.startVariables(builder);
    zkif.Variables.addVariableIds(builder, ids);
    zkif.Variables.addValues(builder, values);
    var variables = zkif.Variables.endVariables(builder);

    zkif.Witness.startWitness(builder);
    zkif.Witness.addAssignedVariables(builder, variables);
    var witness = zkif.Witness.endWitness(builder);

    return zkif.finishMessage(builder, zkif.Message.Witness, witness);
}


function read(buffer) {
    var message = zkif.parseMessage(buffer);
    
    switch (message.messageType()) {
        
        case zkif.Message.Circuit:
            var circuit = message.message(new zkif.Circuit());
            
            log('Got a Circuit metadata message.');
            log('Number of variables:', zkif.fromId(circuit.freeVariableId()));

            break;

        case zkif.Message.R1CSConstraints:
            var constraints = message.message(new zkif.R1CSConstraints());

            log('Got a R1CSConstraints message.')

            break;
            
        case zkif.Message.Witness:
            var witness = message.message(new zkif.Witness());

            var variables = witness.assignedVariables();
            var values = variables.valuesArray();
            var num_vars = variables.variableIdsLength();
            var element_size = values.length / num_vars;

            log("Got a Witness for", num_vars, "variables with values of size", element_size);
            log("ID\t|\tValue");

            for(var i = 0; i < num_vars; i++) {
                var id = zkif.fromId(variables.variableIds(i));
                var value = values.subarray(element_size * i, element_size * (i + 1));
                log(id + "\t|\t", value);
            }

            break;
    }
}


var buffer = write_example_circuit();
read(buffer);

buffer = write_example_witness();
read(buffer);
