use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct DjinnError {
    position: (u32, u32),
    message: String,
}

impl DjinnError {
    pub fn with_message(message: String) -> Self {
        Self {
            position: (0, 0),
            message,
        }
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct DjinnErrorList(Vec<DjinnError>);
