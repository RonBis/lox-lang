#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Constant(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

pub mod operand {
    pub enum BinaryOp {
        Add,
        Substract,
        Multiply,
        Divide,
    }
}
