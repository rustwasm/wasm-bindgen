use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `AuthenticatorAttachment` enum.
///
///*This API requires the following crate features to be activated: `AuthenticatorAttachment`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AuthenticatorAttachment {
    Platform = "platform",
    CrossPlatform = "cross-platform",
}
