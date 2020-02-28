use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RequestMode` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestMode`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RequestMode {
    SameOrigin = "same-origin",
    NoCors = "no-cors",
    Cors = "cors",
    Navigate = "navigate",
}
