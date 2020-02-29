use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `ResponseType` enum.
///
///*This API requires the following crate features to be activated: `ResponseType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum ResponseType {
    Basic = "basic",
    Cors = "cors",
    Default = "default",
    Error = "error",
    Opaque = "opaque",
    Opaqueredirect = "opaqueredirect",
}
