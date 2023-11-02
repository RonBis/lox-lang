use crate::instruction::{
    chunk::Chunk,
    opcode::{operand::BinaryOp, OpCode},
    types::Value,
};

struct VMStack {
    stack: [Value; 256],
    top: usize,
}

impl VMStack {
    fn new() -> Self {
        let arr = [0.0; 256];
        VMStack { stack: arr, top: 0 }
    }

    fn push(&mut self, value: Value) {
        self.stack[self.top] = value;
        self.top += 1;
    }

    fn pop(&mut self) -> Value {
        self.top -= 1;
        self.stack[self.top]
    }

    fn _reset(mut self) {
        self.top = 0;
    }

    pub fn binary_op(&mut self, op: BinaryOp) {
        let b = self.pop();
        let a = self.pop();

        let res = match op {
            BinaryOp::Add => a + b,
            BinaryOp::Substract => a - b,
            BinaryOp::Multiply => a * b,
            BinaryOp::Divide => a / b,
        };

        self.push(res);
    }
}

pub struct VM {
    pub chunk: Chunk,
    vmstack: VMStack,
}

impl VM {
    pub fn init(chunk: Chunk) -> Self {
        VM {
            chunk,
            vmstack: VMStack::new(),
        }
    }

    pub fn interpret(mut self, debug_trace_execution: bool) -> InterpretResult {
        self.run(debug_trace_execution);

        InterpretResult::Ok
    }

    fn run(&mut self, debug_trace_execution: bool) {
        let ip = self.chunk.code.iter(); // Instruction Pointer

        let vmstack = &mut self.vmstack;

        for (offset, instruction) in ip.enumerate() {
            if debug_trace_execution {
                print!("          ");

                let mut counter = 0;
                while counter < vmstack.top {
                    print!("[ {} ]", vmstack.stack[counter]);
                    counter += 1;
                }

                println!();
                self.chunk.disassemble_instruction(offset);
            }

            match instruction {
                OpCode::Constant(const_index) => {
                    let constant = self.chunk.constants[*const_index];
                    vmstack.push(constant);
                }
                OpCode::Add => vmstack.binary_op(BinaryOp::Add),
                OpCode::Subtract => vmstack.binary_op(BinaryOp::Substract),
                OpCode::Multiply => vmstack.binary_op(BinaryOp::Multiply),
                OpCode::Divide => vmstack.binary_op(BinaryOp::Divide),
                OpCode::Negate => {
                    /* let popped = vmstack.pop();
                    vmstack.push(-popped); */

                    // just multiply top with -1
                    vmstack.stack[vmstack.top - 1] *= -1 as Value;
                }
                OpCode::Return => {
                    println!("{}", vmstack.pop());
                    // InterpretResult::Ok
                }
            };
        }
    }
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}
