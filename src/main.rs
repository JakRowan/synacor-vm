extern crate synacor_vm;

use std::{env, fmt, fs, process};
use synacor_vm::{Error, Result, VirtualMachine};

fn main() -> Result<()> {
    let command = env::args().nth(1).unwrap_or("help".into());

    match command.as_ref() {
        "help" => {
            print_usage(true);
        }

        "run" => {
            let bytecode = read_bytecode();
            VirtualMachine::new().load_bytecode(&bytecode)?.run()?;
        }

        "disassemble" => {
            let bytecode = read_bytecode();
            let outfile = match env::args().nth(3) {
                Some(x) => x,
                None => print_err_usage("No <outfile> supplied."),
            };
            let asm = VirtualMachine::new()
                .load_bytecode(&bytecode)?
                .disassemble();

            fs::write(outfile, &asm).map_err(|error| Error::IoErr { pc: 0, error })?;
        }

        x => print_err_usage(format!("\"{}\" is not a valid command.", x)),
    }
    Ok(())
}

fn print_usage(print_header: bool) {
    if print_header {
        println!("Synacor Challenge Utilities");
        println!();
        println!("Virtual Machine and Reverse Engineering Tools");
        println!();
    }
    println!("Usage:");
    println!("    synacor-vm help                            Print this usage information");
    println!("    synacor-vm run <infile>                    Run compiled synacor binary");
    println!("    synacor-vm disassemble <infile> <outfile>  Run compiled synacor binary");
}

fn print_err_usage<M: fmt::Display>(err_msg: M) -> ! {
    println!("{}\n", err_msg);
    print_usage(false);
    process::exit(1)
}

fn read_bytecode() -> Vec<u8> {
    match env::args().nth(2) {
        Some(filepath) => match fs::read(filepath) {
            Ok(x) => x,
            Err(e) => {
                eprintln!("{}", e);
                process::exit(1);
            }
        },
        None => print_err_usage("No <infile> supplied."),
    }
}
