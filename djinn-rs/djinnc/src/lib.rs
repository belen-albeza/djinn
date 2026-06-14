use anyhow::Result;

use djinn_core::asm::Opcode;
pub use djinn_core::cart::Cartridge;
use djinn_core::cart::Rom;

pub fn build(title: &str, _source_code: &str) -> Result<Cartridge> {
    Ok(Cartridge::new(
        title,
        Rom::new(vec![Opcode::NoOp, Opcode::Yield, Opcode::NoOp]),
    ))
}
