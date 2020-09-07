extern crate synacor_vm;

use std::env;
use synacor_vm::VirtualMachine;

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let vm = match VirtualMachine::new().load_bytecode(&filepath) {
        Ok(x) => x,
        Err(e) => return println!("{}", e),
    };

    vm.exec();
}
