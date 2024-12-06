#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `RequestRedirect` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RequestRedirect`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestRedirect {
    Follow = "follow",
    Error = "error",
    Manual = "manual",
}
