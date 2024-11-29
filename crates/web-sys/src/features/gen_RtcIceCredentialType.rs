#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen(no_export)]
#[doc = "The `RtcIceCredentialType` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `RtcIceCredentialType`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RtcIceCredentialType {
    Password = "password",
    Token = "token",
}
