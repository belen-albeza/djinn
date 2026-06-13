use wasm_bindgen::prelude::*;

use djinn_core::devices::DeviceSet;
use djinn_core::vm::Machine;

use crate::error::DjinnError;

#[wasm_bindgen]
pub struct Emulator {
    cart: djinnc::Cartridge,
    vm: Machine<DeviceSet>,
}

impl Emulator {
    pub fn new(cart: djinnc::Cartridge) -> Self {
        Self {
            cart,
            vm: Machine::new(DeviceSet::default()),
        }
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen]
    pub fn step(&mut self) -> Result<bool, DjinnError> {
        // TODO: implement halting logic
        let shall_halt = true;

        self.vm
            .step()
            // TODO: relate error position to the source code
            .map_err(|e| DjinnError::with_message(e.to_string()))?;
        Ok(shall_halt)
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.cart.title().to_string()
    }
}
