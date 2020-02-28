use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PushEncryptionKeyName` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PushEncryptionKeyName`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PushEncryptionKeyName {
    P256dh = "p256dh",
    Auth = "auth",
}
