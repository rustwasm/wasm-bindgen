#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `TcpSocketBinaryType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `TcpSocketBinaryType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TcpSocketBinaryType {
    Arraybuffer = "arraybuffer",
    String = "string",
}
