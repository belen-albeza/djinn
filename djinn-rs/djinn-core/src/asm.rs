#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessType(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Opcode {
    NoOp,
    // Process control
    Yield,
}
