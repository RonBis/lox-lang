use std::{path::PathBuf, process::exit};

use clap::Parser;
use lox_lang::{
    instruction::{chunk::Chunk, opcode::OpCode},
    vm::VM,
};

#[derive(Parser, Debug)]
struct Cli {
    path: Option<PathBuf>,
}

fn main() {
    let args = Cli::parse();

    match &args.path {
        None => repl(),
        Some(path) => run_file(path),
    }

    let mut chunk = Chunk::new();

    let const_index = chunk.add_constant(2.2);
    chunk.write(OpCode::Constant(const_index), 123);

    let const_index = chunk.add_constant(3.4);
    chunk.write(OpCode::Constant(const_index), 123);

    chunk.write(OpCode::Add, 123);

    let const_index = chunk.add_constant(5.6);
    chunk.write(OpCode::Constant(const_index), 123);

    chunk.write(OpCode::Divide, 123);

    chunk.write(OpCode::Negate, 123);
    chunk.write(OpCode::Return, 123);

    // chunk.disassemble_chunk("test chunk");
}

fn repl() {}

fn run_file(path: &PathBuf) {
    let source = std::fs::read_to_string(path).expect("Couldn't read file");

    // let vm = VM::init(source);
    // let result = vm.interpret(true);

    // use lox_lang::vm::InterpretResult::*;

    // match result {
    //     CompileError => exit(65),
    //     RuntimeError => exit(70),
    //     Ok => exit(0),
    // };
}
