extern crate synacor_vm;

use std::{env, fmt, fs, process};
use synacor_vm::{teleporter, Error, Result, VirtualMachine};

#[derive(Default)]
struct Options {
    out: Option<String>,
    asm_addresses: bool,
}

impl Options {
    fn from_args() -> Self {
        let mut opts = Options::default();

        for arg in env::args() {
            match arg.as_ref() {
                x if x.starts_with("--out=") => opts.out = Some(x.chars().skip(6).collect()),
                "--with-addresses" => {
                    opts.asm_addresses = true;
                }
                x if x.starts_with("-") => print_err_usage(&format!(
                    "No option \"{}\" exists",
                    x.chars().take_while(|&c| c != '=').collect::<String>()
                )),
                _ => {}
            }
        }
        opts
    }
}

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
            let opts = Options::from_args();
            let asm = VirtualMachine::new()
                .load_bytecode(&bytecode)?
                .disassemble(opts.asm_addresses);

            match opts.out {
                Some(outfile) => {
                    fs::write(outfile, &asm).map_err(|error| Error::IoErr { pc: 0, error })?
                }
                None => println!("{}", asm),
            };
        }

        "solve-calibration" => {
            if let Some(hx) = env::args().nth(2) {
                println!("Solving calibration for HX = {}", hx);
                let hx: u16 = match hx.parse() {
                    Ok(x) => x,
                    _ => print_err_usage(format!("Could not parse HX as u16: {}", hx)),
                };
                let mut mem = teleporter::Memory::new(hx);
                match teleporter::calibrate(&mut mem) {
                    6 => println!("Set HX to {} for proper teleportation!", hx),
                    x => println!("Result: {}, is not valid.", x),
                }
            } else {
                println!("Solving calibration for HX...");
                match teleporter::solve_calibration_for_hx() {
                    Some(v) => println!("Set HX to {} for proper teleportation!", v),
                    None => println!("No valid HX found! Something is wrong here..."),
                }
            }
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
    println!("    synacor-vm disassemble <infile> [options]  Disassemble compiled synacor binary");
    println!("    synacor-vm solve-calibration [value]       Solve calibration for HX register");
    println!();
    println!("Options:");
    println!("    --out=<outfile>   Write to a given output file instead of stdout");
    println!("    --with-addresses  Specify that the assembly should be addressed");
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
