mod asm;

pub use asm::{AssemblerError, Location};
pub use djinn_core::cart::Cartridge;

pub fn bundle(title: &str, source_code: &str) -> Result<Cartridge, AssemblerError> {
    let rom = asm::compile(source_code)?;
    Ok(Cartridge::new(title, rom))
}
