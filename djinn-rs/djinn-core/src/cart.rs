use crate::asm::Opcode;
use crate::vm::InstructionProvider;

#[derive(Debug, Clone, PartialEq)]
pub struct Rom {
    instructions: Vec<Opcode>,
}

impl Rom {
    pub fn new(instructions: Vec<Opcode>) -> Self {
        Self { instructions }
    }
}

impl InstructionProvider for Rom {
    fn instructions(&self) -> &[Opcode] {
        &self.instructions
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cartridge {
    title: String,
    rom: Rom,
}

impl Cartridge {
    pub fn new(title: &str, rom: Rom) -> Self {
        Self {
            title: title.to_string(),
            rom,
        }
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn rom(&self) -> &Rom {
        &self.rom
    }
}
