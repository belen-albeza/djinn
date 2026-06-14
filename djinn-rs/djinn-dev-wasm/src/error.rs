use djinnc::AssemblerError;
use serde::Serialize;
use tsify::Tsify;
use wasm_bindgen::prelude::*;

// We need this to avoid including serde in the djinnc crate
// and still export a Location type to the WebAssembly module.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Tsify)]
pub struct Location {
    line: usize,
    column: usize,
}

impl From<djinnc::Location> for Location {
    fn from(location: djinnc::Location) -> Self {
        Location {
            line: location.line,
            column: location.column,
        }
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct DjinnError {
    position: Location,
    message: String,
}

impl DjinnError {
    pub fn new(position: Location, message: String) -> Self {
        Self { position, message }
    }

    pub fn with_message(message: String) -> Self {
        Self {
            position: Location { line: 0, column: 0 },
            message,
        }
    }
}

impl From<AssemblerError> for DjinnError {
    fn from(error: AssemblerError) -> Self {
        Self::new(error.location().into(), error.message().to_string())
    }
}

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct DjinnErrorList(pub Vec<DjinnError>);
