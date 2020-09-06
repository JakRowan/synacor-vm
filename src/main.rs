extern crate synacor_vm;

use std::{env, fmt, fs, io};
use synacor_vm::VirtualMachine;

fn main() -> io::Result<()> {
    let command = env::args().nth(1).unwrap_or("help".into());

    match command.as_ref() {
        "help" => {
            print_usage();
            return Ok(());
        }

        "run" => {
            let bytecode = match env::args().nth(2) {
                Some(filepath) => fs::read(filepath)?,
                None => {
                    print_err_usage("No <bytcode_file> supplied.");
                    return Ok(());
                }
            };

            match VirtualMachine::new()
                .load_bytecode(&bytecode)
                .map(|vm| vm.run())
            {
                Ok(Err(e)) | Err(e) => {
                    println!("Error: {}", e);
                    return Ok(());
                }
                _ => {}
            }
        }

        x => {
            print_err_usage(format!("\"{}\" is not a valid command.", x));
            return Ok(());
        }
    }

    Ok(())
}

fn print_usage() {
    println!("Synacor Challenge Utilities");
    println!();
    println!("Virtual Machine and Reverse Engineering Tools");
    println!();
    println!("Usage:");
    println!("    synacor-vm help         Print this usage information");
    println!("    synacor-vm run <input>  Run compiled synacor binary");
}

fn print_err_usage<'a, M: fmt::Display>(err_msg: M) {
    println!("{}\n", err_msg);
    print_usage();
}
