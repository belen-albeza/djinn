use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Emulator {
    cart: djinnc::Cartridge,
}

impl Emulator {
    pub fn new(cart: djinnc::Cartridge) -> Self {
        Self { cart }
    }
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(getter)]
    pub fn title(&self) -> String {
        self.cart.title().to_string()
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct BuildError {
    position: (u32, u32),
    message: String,
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct BuildErrorList(Vec<BuildError>);

#[wasm_bindgen]
pub fn build(_title: &str) -> Result<Emulator, BuildErrorList> {
    // let cart = djinnc::build(title, "").map_err(|e| JsValue::from(e.to_string()))?;
    // Ok(Emulator::new(cart))
    let errors = BuildErrorList(vec![
        BuildError {
            position: (1, 1),
            message: "Unexpected character `*`".to_string(),
        },
        BuildError {
            position: (1, 1),
            message: "`main` process not found".to_string(),
        },
    ]);
    Err(errors)
}

#[wasm_bindgen(start)]
fn init() -> Result<(), JsValue> {
    Ok(())
}
