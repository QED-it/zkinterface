/*
./zokrates compile --input zokrates_cli/examples/simple_add.code
./zokrates setup --backend zkinterface
./zokrates compute-witness -a 3 4
./zokrates generate-proof --backend zkinterface

flatc --json --raw-binary --size-prefixed ../gadget_standard/zkinterface.fbs -- call.zkif          && cat call.json
flatc --json --raw-binary --size-prefixed ../gadget_standard/zkinterface.fbs -- return_r1cs.zkif   && cat return_r1cs.json
flatc --json --raw-binary --size-prefixed ../gadget_standard/zkinterface.fbs -- r1cs.zkif          && cat r1cs.json
flatc --json --raw-binary --size-prefixed ../gadget_standard/zkinterface.fbs -- return_assign.zkif && cat return_assign.json
flatc --json --raw-binary --size-prefixed ../gadget_standard/zkinterface.fbs -- assign.zkif        && cat assign.json
*/

use num_bigint::BigUint;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Output};
use zkinterface::{
    reading::{
        CallbackContext,
        parse_call,
    },
};


pub fn exec_zokrates(call_msg: &[u8]) -> Result<CallbackContext, String> {
    let (call, inputs) = parse_call(call_msg).unwrap();

    let program = "demo.code"; // "zokrates_cli/examples/simple_add.code";
    let zokrates_home = env::var("ZOKRATES_HOME").unwrap();
    let zokrates_home = Path::new(&zokrates_home);

    let make_zokrates_command = || {
        Command::new("./exec_zokrates")
    };

    let mut context = CallbackContext {
        constraints_messages: vec![],
        assigned_variables_messages: vec![],
        return_message: None,
    };

    {
        let mut load_message = |name: &str| {
            let path = zokrates_home.join(name);
            let mut file = File::open(&path).unwrap();
            let mut buf = Vec::new();
            file.read_to_end(&mut buf);
            println!("loaded {} ({} bytes)", name, buf.len());
            context.store_message(buf)
        };

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
            cmd.args(&["compile", "--input", program]);
            let out = exec(&mut cmd);
        }

        // Get R1CS -> r1cs.zkif
        {
            let mut cmd = make_zokrates_command();
            cmd.args(&["setup", "--backend", "zkinterface", "-p", "r1cs.zkif"]);
            let out = exec(&mut cmd);

            load_message("r1cs.zkif")?;
            load_message("return_r1cs.zkif")?;
        }

        if call.generate_assignment() {
            // Compute assignment.
            {
                let mut cmd = make_zokrates_command();
                cmd.args(&["compute-witness", "--arguments"]);

                // Convert input elements to decimal on the command line.
                for input in inputs {
                    cmd.arg(le_to_decimal(input.element));
                }

                let out = exec(&mut cmd);
            }

            // Get assignment -> assign.zkif
            {
                let mut cmd = make_zokrates_command();
                cmd.args(&["generate-proof", "--backend", "zkinterface", "-j", "assign.zkif"]);
                let out = exec(&mut cmd);

                load_message("assign.zkif")?;
                load_message("return_assign.zkif")?;
            }
        }
    }

    Ok(context)
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

