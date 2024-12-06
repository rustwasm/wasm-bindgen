#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `BrowserFindDirection` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `BrowserFindDirection`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrowserFindDirection {
    Forward = "forward",
    Backward = "backward",
}
