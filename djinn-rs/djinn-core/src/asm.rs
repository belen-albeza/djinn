#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ProcessType(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProcessId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Opcode {
    #[default]
    NoOp,
    // Process control
    Yield,
}
