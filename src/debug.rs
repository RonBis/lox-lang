use crate::instruction::{chunk::Chunk, opcode::OpCode, types::Value};

impl Chunk {
    pub fn disassemble_chunk(&self, name: &str) {
        println!("== {} ==", name);

        let mut offset = 0;
        while offset < self.code.len() {
            self.disassemble_instruction(offset);
            offset += 1;
        }
    }

    pub fn disassemble_instruction(&self, offset: usize) {
        print!("{:0>4} ", offset);

        // if chunk is on same line as previous one, print '|'
        if offset > 0 && self.lines[offset] == self.lines[offset - 1] {
            print!("   | ");
        } else {
            print!("{:>4} ", self.lines[offset]);
        }

        let instruction = self.code[offset];
        match instruction {
            OpCode::Constant(const_index) => {
                constant_instruction("OP_CONSTANT", self.constants[const_index], const_index);
            }
            OpCode::Add => simple_instruction("OP_ADD"),
            OpCode::Subtract => simple_instruction("OP_SUBTRACT"),
            OpCode::Multiply => simple_instruction("OP_MULTIPLY"),
            OpCode::Divide => simple_instruction("OP_DIVIDE"),
            OpCode::Negate => simple_instruction("OP_NEGATE"),
            OpCode::Return => simple_instruction("OP_RETURN"),
        };
    }
}

fn simple_instruction(name: &str) {
    println!("{}", name);
}

fn constant_instruction(name: &str, constant: Value, const_index: usize) {
    println!("{}    index->{}  value->{}", name, const_index, constant,);
}
