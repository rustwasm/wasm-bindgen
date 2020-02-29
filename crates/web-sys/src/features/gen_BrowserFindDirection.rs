use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `BrowserFindDirection` enum.
///
///*This API requires the following crate features to be activated: `BrowserFindDirection`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum BrowserFindDirection {
    Forward = "forward",
    Backward = "backward",
}
