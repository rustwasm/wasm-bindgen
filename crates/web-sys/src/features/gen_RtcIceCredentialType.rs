use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `RtcIceCredentialType` enum.\n\n*This API requires the following crate features to be activated: `RtcIceCredentialType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum RtcIceCredentialType {
    Password = "password",
    Token = "token",
}
