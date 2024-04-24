#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PublicKeyCredentialHints` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PublicKeyCredentialHints`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicKeyCredentialHints {
    SecurityKey = "security-key",
    ClientDevice = "client-device",
    Hybrid = "hybrid",
}
