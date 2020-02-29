use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `HeadersGuardEnum` enum.
///
///*This API requires the following crate features to be activated: `HeadersGuardEnum`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum HeadersGuardEnum {
    None = "none",
    Request = "request",
    RequestNoCors = "request-no-cors",
    Response = "response",
    Immutable = "immutable",
}
