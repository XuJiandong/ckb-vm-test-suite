mod cost_model;

use bytes::Bytes;
use ckb_vm::machine::asm::{AsmMachine, AsmCoreMachine};
use ckb_vm::machine::{SupportMachine, VERSION1};
use ckb_vm::DefaultMachineBuilder;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut file = File::open(args[0].clone()).unwrap();
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();
    let buffer: Bytes = buffer.into();
    let args: Vec<Bytes> = args.into_iter().map(|a| a.into()).collect();

    let asm_core = AsmCoreMachine::new(ckb_vm::ISA_IMC, VERSION1, 1 << 31);
    let core = DefaultMachineBuilder::<Box<AsmCoreMachine>>::new(asm_core)
        .instruction_cycle_func(Box::new(cost_model::instruction_cycles))
        .build();
    let mut machine = AsmMachine::new(core, None);

    machine.load_program(&buffer, &args).unwrap();
    let result = machine.run();
    let cycles = machine.machine.cycles();
    println!("Cycles = {:?}", cycles);
    if result != Ok(0) {
        println!("Error result: {:?}", result);
        exit(i32::from(result.unwrap_or(-1)));
    }
}
