use std::collections::HashMap;
use std::sync::LazyLock;

use crate::asm::{AssemblerError, Location, Result};

static ALIASES: LazyLock<HashMap<&'static str, u8>> = LazyLock::new(|| {
    HashMap::from([
        // device types
        ("console", 0x00),
        ("video", 0x01),
        // console api
        ("log", 0x00),
        // video api
        ("clear", 0x00),
        ("put-pixel", 0x01),
        ("get-pixel", 0x02),
    ])
});

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alias(pub u8);

impl TryFrom<String> for Alias {
    type Error = AssemblerError;

    fn try_from(value: String) -> Result<Self> {
        let value = ALIASES
            .get(value.as_str())
            .ok_or(AssemblerError::UnknownAlias(Location::default(), value))?;

        Ok(Self(*value))
    }
}
