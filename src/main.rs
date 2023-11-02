use lox_lang::{
    instruction::{chunk::Chunk, opcode::OpCode},
    vm::VM,
};

fn main() {
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

    let vm = VM::init(chunk);
    vm.interpret(true);
}
