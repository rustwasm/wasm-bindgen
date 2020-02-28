use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ConsoleLevel` enum.\n\n*This API requires the following crate features to be activated: `ConsoleLevel`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ConsoleLevel {
    Log = "log",
    Warning = "warning",
    Error = "error",
}
