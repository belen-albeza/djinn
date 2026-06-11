use anyhow::Result;
pub use djinn_core::Cartridge;

pub fn build(title: &str, _source_code: &str) -> Result<Cartridge> {
    Ok(Cartridge::new(title))
}
