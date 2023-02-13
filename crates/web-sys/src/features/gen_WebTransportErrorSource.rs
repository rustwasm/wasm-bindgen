#![allow(unused_imports)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `WebTransportErrorSource` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `WebTransportErrorSource`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebTransportErrorSource {
    Stream = "stream",
    Session = "session",
}
