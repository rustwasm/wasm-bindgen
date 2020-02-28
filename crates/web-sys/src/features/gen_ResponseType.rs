use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ResponseType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ResponseType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ResponseType {
    Basic = "basic",
    Cors = "cors",
    Default = "default",
    Error = "error",
    Opaque = "opaque",
    Opaqueredirect = "opaqueredirect",
}
