#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `ClientType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ClientType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientType {
    Window = "window",
    Worker = "worker",
    Sharedworker = "sharedworker",
    Serviceworker = "serviceworker",
    All = "all",
}
