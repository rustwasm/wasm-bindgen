use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `IdbCursorDirection` enum.
///
///*This API requires the following crate features to be activated: `IdbCursorDirection`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum IdbCursorDirection {
    Next = "next",
    Nextunique = "nextunique",
    Prev = "prev",
    Prevunique = "prevunique",
}
