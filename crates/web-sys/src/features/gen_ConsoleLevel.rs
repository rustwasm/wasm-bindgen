use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ConsoleLevel` enum.
///
///*This API requires the following crate features to be activated: `ConsoleLevel`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ConsoleLevel {
    Log = "log",
    Warning = "warning",
    Error = "error",
}
