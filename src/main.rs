extern crate synacor;

use std::{env, fs, io};

fn main() -> io::Result<()> {
    let path = env::args().nth(1).unwrap();
    let bytecode = fs::read(path)?;

    synacor::run(bytecode);

    Ok(())
}
