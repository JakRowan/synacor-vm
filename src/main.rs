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
        let assembly = fs::read_to_string(in_path)?;
        let out_path = match env::args().nth(3) {
            Some(x) => x,
            None => {
                print_usage();
                return Ok(());
            }
        };
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
        let bytecode = fs::read(in_path)?;
        let vm = synacor::VirtualMachine::new().load(bytecode);

        vm.decompile();
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
    println!("  synacor-vm help                       Print this usage information");
    println!("  synacor-vm compile <input> <output>   Compile Synacor assembly into bytecode");
    println!("  synacor-vm decompile <input>          Decompile Synacor bytecode into assembly");
    println!("  synacor-vm execute <input>            Run compiled Synacor bytecode");
    println!("");
    println!("Options:");
    println!("  --help  Print this usage information");
    println!("");
    println!("Execution Commands:");
    println!("  !help   Prints execution command help menu when input during program execution");
}
