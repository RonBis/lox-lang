use lox_lang::chunk::{Chunk, OpCode};

fn main() {
    let mut chunk = Chunk::new();
    let const_index = chunk.add_constant(1.2);

    chunk.write(OpCode::OpConstant(const_index), 123);
    chunk.write(OpCode::OpReturn, 123);

    chunk.disassemble_chunk("test chunk");
}
