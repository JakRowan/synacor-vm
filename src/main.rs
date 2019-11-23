extern crate synacor_vm as synacor;

use std::{env, fs, io};

fn main() -> io::Result<()> {
    let command = match env::args().nth(1) {
        Some(c) => c,
        None => {
            print_usage();
            return Ok(());
        }
    };

    if command.as_str() == "help" {
        print_usage();
        return Ok(());
    }

    if command.as_str() == "compile" {
        let in_path = match env::args().nth(2) {
            Some(x) => x,
            None => {
                print_usage();
                return Ok(());
            }
        };
        let out_path = match env::args().nth(3) {
            Some(x) => x,
            None => {
                print_usage();
                return Ok(());
            }
        };
        let assembly = fs::read_to_string(in_path)?;
        let compiler = synacor::Compiler::new().load(assembly);

        compiler.compile(out_path)?;
    } else if command.as_str() == "decompile" {
        let in_path = match env::args().nth(2) {
            Some(x) => x,
            None => {
                print_usage();
                return Ok(());
            }
        };
        let out_path = match env::args().nth(3) {
            Some(x) => x,
            None => {
                print_usage();
                return Ok(());
            }
        };
        let bytecode = fs::read(in_path)?;
        let mut vm = synacor::VirtualMachine::new().load(bytecode);

        vm.decompile(out_path);
    } else if command.as_str() == "execute" {
        let in_path = match env::args().nth(2) {
            Some(x) => x,
            None => {
                print_usage();
                return Ok(());
            }
        };
        let bytecode = fs::read(in_path)?;
        let vm = synacor::VirtualMachine::new().load(bytecode);

        vm.execute();
    } else if command.as_str() == "solve-calibration" {
        println!("Solving calibration for R8...");

        match synacor::teleporter::solve_calibration_for_r8() {
            Some(v) => println!("Set R8 to {} for proper teleportation!", v),
            None => println!("No valid R8 found! Something is wrong here..."),
        }
    } else {
        print_usage();
    }

    Ok(())
}

fn print_usage() {
    println!("Synacor Challenge Utilities");
    println!("");
    println!("Virtual Machine and Reverse Engineering Tools");
    println!("");
    println!("Usage:");
    println!("  synacor-vm help                        Print this usage information");
    println!("  synacor-vm compile <input> <output>    Compile Synacor assembly into bytecode");
    println!("  synacor-vm decompile <input> <output>  Decompile Synacor bytecode into assembly");
    println!("  synacor-vm execute <input>             Run compiled Synacor bytecode");
    println!("  synacor-vm solve-calibration           Solve telepoter calibration for R8 value");
    println!("");
    println!("Options:");
    println!("  --help  Print this usage information");
    println!("");
    println!("Execution Commands:");
    println!("  !help         Prints execution command help when input during program execution");
    println!("  !disassemble  Dissasembles the current memory into a file");
    println!("  !save_state   Saves the vm state into a file");
    println!("  !load_state   Loads the state file into the vm");
}
