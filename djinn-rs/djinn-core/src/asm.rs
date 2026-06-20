mod opcode;
mod value;
pub use opcode::Opcode;
pub use value::{Number, Value};

#[derive(Debug, Clone, Copy, PartialEq, Default, Eq, Hash)]
pub struct ProcessType(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcessId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub struct ProcessDefinition {
    process_type: ProcessType,
    instructions: Vec<Opcode>,
}

impl ProcessDefinition {
    pub fn new(process_type: ProcessType, instructions: Vec<Opcode>) -> Self {
        Self {
            process_type,
            instructions,
        }
    }

    pub fn process_type(&self) -> ProcessType {
        self.process_type
    }

    pub fn instructions(&self) -> &[Opcode] {
        &self.instructions
    }
}
