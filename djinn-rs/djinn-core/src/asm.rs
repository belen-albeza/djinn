use std::fmt;

mod opcode;
mod value;
pub use opcode::Opcode;
pub use value::{Number, Value};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ln {}, Col {}", self.line, self.column)
    }
}

impl Default for Location {
    fn default() -> Self {
        Self { line: 1, column: 1 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
pub struct ProcessType(pub u32);

impl fmt::Display for ProcessType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "~{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u32);

impl fmt::Display for ProcessId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Instruction {
    pub opcode: Opcode,
    pub location: Location,
}

impl Instruction {
    pub fn new(opcode: Opcode, location: Location) -> Self {
        Self { opcode, location }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessDefinition {
    process_type: ProcessType,
    instructions: Vec<Instruction>,
}

impl ProcessDefinition {
    pub fn new(process_type: ProcessType, instructions: Vec<Instruction>) -> Self {
        Self {
            process_type,
            instructions,
        }
    }

    pub fn process_type(&self) -> ProcessType {
        self.process_type
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }
}
