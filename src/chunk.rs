#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpConstant(usize),
    OpReturn,
}
pub type Value = f64;

#[derive(Debug)]
pub struct Chunk {
    pub code: Vec<OpCode>,
    pub constants: Vec<Value>,
    pub lines: Vec<i8>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write(&mut self, byte: OpCode, line: i8) {
        self.code.push(byte);
        self.lines.push(line)
    }

    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push(value);

        self.constants.len() - 1
    }
}

// impl Drop for Chunk {
//     fn drop(&mut self) {
//         println!("Dropping {:?}", self);
//     }
// }
