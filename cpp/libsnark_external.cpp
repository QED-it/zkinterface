/**
 * Call an external zkInterface circuit generator into a protoboard.
 */

/*
class gadget_import_zkif(pb, input_vars, zkif_executor)
{
    constructor()
    {
        request = serialize(
            Circuit
                r1cs_generation = false
                witness_generation = false
        )

        response_bytes = zkif_executor.call( request_bytes )

        response = deserialize(response_bytes)

        zkif_executor.free()

        for each message in responses
            if message type != circuit
                continue

            for each var in circuit
                pb.allocate_auxiliary(…)
    }


    generate_constraints() {
      responses = call zkinterface gadget(
          Circuit
              r1cs_generation = true
              witness_generation = false
      )

      for each message in responses
          if message.type != constraints
              continue

          for each var in message.constraints
              pb.add_constraint(…)
    }


    generate_witness() {
      response = call zkinterface gadget(
          Circuit
              r1cs_generation = false
              witness_generation = true
      )

      for each message in response
          if message type != witness
              continue

          for each var in response.witness
              pb.val(id, value)

    }


    create_request()
    {
        CircuitBuilder
            .add_connections([input_var.id])
            .add_free_variable_id(pb.next_free_var_id)
    }
}
*/