use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `IdbCursorDirection` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `IdbCursorDirection`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum IdbCursorDirection {
    Next = "next",
    Nextunique = "nextunique",
    Prev = "prev",
    Prevunique = "prevunique",
}
