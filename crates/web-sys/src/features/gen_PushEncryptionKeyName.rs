use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `PushEncryptionKeyName` enum.
///
///*This API requires the following crate features to be activated: `PushEncryptionKeyName`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum PushEncryptionKeyName {
    P256dh = "p256dh",
    Auth = "auth",
}
