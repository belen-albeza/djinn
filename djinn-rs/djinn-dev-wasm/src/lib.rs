use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;

use djinn_core::devices::DeviceSet;
use djinn_core::vm::Machine;

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct DjinnError {
    position: (u32, u32),
    message: String,
}

impl DjinnError {
    fn with_message(message: String) -> Self {
        Self {
            position: (0, 0),
            message,
        }
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct DjinnErrorList(Vec<DjinnError>);

#[derive(Deserialize, Tsify)]
#[tsify(from_wasm_abi)]
pub struct Project {
    title: String,
    #[serde(rename = "sourceCode")]
    source_code: String,
}

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

#[wasm_bindgen]
pub fn build(project: Project) -> Result<Emulator, DjinnErrorList> {
    // TODO: Convert from djinnc errors to BuildErrorList
    let cart =
        djinnc::build(&project.title, &project.source_code).expect("Failed to build cartridge");
    Ok(Emulator::new(cart))
    // let errors = BuildErrorList(vec![
    //     BuildError {
    //         position: (1, 1),
    //         message: "Unexpected character `*`".to_string(),
    //     },
    //     BuildError {
    //         position: (1, 1),
    //         message: "`main` process not found".to_string(),
    //     },
    // ]);
    // Err(errors)
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    Ok(())
}
