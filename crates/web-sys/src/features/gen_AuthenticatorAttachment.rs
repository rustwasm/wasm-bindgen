use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AuthenticatorAttachment` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticatorAttachment`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AuthenticatorAttachment {
    Platform = "platform",
    CrossPlatform = "cross-platform",
}
