#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(skip_typescript)]
#[doc = "The `WriteCommandType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WriteCommandType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteCommandType {
    Write = "write",
    Seek = "seek",
    Truncate = "truncate",
}
