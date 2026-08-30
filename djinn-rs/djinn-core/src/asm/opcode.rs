use crate::{
    asm::{ProcessType, Value},
    devices::DeviceType,
};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Opcode {
    #[default]
    NoOp,
    // Device
    Device(DeviceType, u8),
    // Process control
    Yield,
    Spawn(ProcessType),
    Kill,
    // Stack
    Push(Value),
    Pop,
    Dup,
    // Vars
    StoreLocal(usize),
    LoadLocal(usize),
    StoreGlobal(usize),
    LoadGlobal(usize),
    // Flow control
    Jump(usize),
    // ALU
    Not,
    And,
    Or,
    Xor,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Inc,
    Dec,
    Eq,
    Neq,
    Lt,
    Leq,
    Gt,
    Geq,
}
