use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ClientType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ClientType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ClientType {
    Window = "window",
    Worker = "worker",
    Sharedworker = "sharedworker",
    Serviceworker = "serviceworker",
    All = "all",
}
