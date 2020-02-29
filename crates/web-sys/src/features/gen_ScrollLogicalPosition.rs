use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ScrollLogicalPosition` enum.
///
///*This API requires the following crate features to be activated: `ScrollLogicalPosition`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ScrollLogicalPosition {
    Start = "start",
    Center = "center",
    End = "end",
    Nearest = "nearest",
}
