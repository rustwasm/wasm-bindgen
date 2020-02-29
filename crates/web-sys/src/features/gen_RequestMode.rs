use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `RequestMode` enum.
///
///*This API requires the following crate features to be activated: `RequestMode`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum RequestMode {
    SameOrigin = "same-origin",
    NoCors = "no-cors",
    Cors = "cors",
    Navigate = "navigate",
}
