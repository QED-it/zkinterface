/*
./zokrates compile --input zokrates_cli/examples/simple_add.code
./zokrates setup --backend zkinterface
./zokrates compute-witness -a 3 4
./zokrates generate-proof --backend zkinterface

flatc --json --raw-binary --size-prefixed ../zkinterface/zkinterface.fbs -- call.zkif          && cat call.json
flatc --json --raw-binary --size-prefixed ../zkinterface/zkinterface.fbs -- circuit_r1cs.zkif   && cat circuit_r1cs.json
flatc --json --raw-binary --size-prefixed ../zkinterface/zkinterface.fbs -- r1cs.zkif          && cat r1cs.json
flatc --json --raw-binary --size-prefixed ../zkinterface/zkinterface.fbs -- circuit_witness.zkif && cat circuit_witness.json
flatc --json --raw-binary --size-prefixed ../zkinterface/zkinterface.fbs -- witness.zkif        && cat witness.json
*/

use num_bigint::BigUint;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Output};
use zkinterface::{
    reading::{
        Messages,
        parse_call,
        is_contiguous,
    },
};


pub fn exec_zokrates(call_msg: &[u8]) -> Result<Messages, String> {
    let (call, inputs) = parse_call(call_msg).unwrap();

    // Non-contiguous IDs are not supported by ZoKrates yet.
    let in_connections = call.connections().unwrap();
    let input_ids = in_connections.variable_ids().unwrap().safe_slice();
    assert!(is_contiguous(1, input_ids));
    assert_eq!(1 + input_ids.len() as u64, call.free_variable_id());

    let program = "src/test/demo.code";
    let program = env::current_dir().unwrap().join(program).into_os_string().into_string().unwrap();
    let zokrates_home = env::var("ZOKRATES_HOME").unwrap();
    let zokrates_home = Path::new(&zokrates_home);
    let make_zokrates_command = || { Command::new("src/test/exec_zokrates") };

    let mut messages = Messages::new(call.free_variable_id());

    {
        // Write Call message -> call.zkif
        {
            let call_path = zokrates_home.join("call.zkif");
            println!("Writing {:?}", call_path);
            let mut file = File::create(call_path).unwrap();
            file.write_all(call_msg).unwrap();
        }

        // Compile script.
        {
            let mut cmd = make_zokrates_command();
            cmd.args(&["compile", "--input", &program]);
            let _out = exec(&mut cmd);
        }

        // Get R1CS -> r1cs.zkif
        {
            let mut cmd = make_zokrates_command();
            cmd.args(&["setup", "--backend", "zkinterface", "-p", "r1cs.zkif"]);
            let _out = exec(&mut cmd);

            messages.read_file(zokrates_home.join("r1cs.zkif"))?;
            messages.read_file(zokrates_home.join("circuit_r1cs.zkif"))?;
        }

        if call.witness_generation() {
            // Compute assignment.
            {
                let mut cmd = make_zokrates_command();
                cmd.args(&["compute-witness", "--arguments"]);

                // Convert input elements to decimal on the command line.
                for input in inputs {
                    cmd.arg(le_to_decimal(input.value));
                }

                let _out = exec(&mut cmd);
            }

            // Get assignment -> witness.zkif
            {
                let mut cmd = make_zokrates_command();
                cmd.args(&["generate-proof", "--backend", "zkinterface", "-j", "witness.zkif"]);
                let _out = exec(&mut cmd);

                messages.read_file(zokrates_home.join("witness.zkif"))?;
                messages.read_file(zokrates_home.join("circuit_witness.zkif"))?;
            }
        }
    }

    Ok(messages)
}

/// Convert zkInterface little-endian bytes to zokrates decimal.
fn le_to_decimal(bytes_le: &[u8]) -> String {
    BigUint::from_bytes_le(bytes_le).to_str_radix(10)
}

fn exec(cmd: &mut Command) -> Output {
    let out = cmd.output().expect("failed to execute zokrates generate-proof");
    debug_command(&cmd, &out);
    assert!(out.status.success());
    out
}

fn debug_command(cmd: &Command, out: &Output) {
    use std::str::from_utf8;
    println!("{:?}: {}\n{}\n{}\n",
             cmd,
             out.status.success(),
             from_utf8(&out.stdout).unwrap(),
             from_utf8(&out.stderr).unwrap());
}

