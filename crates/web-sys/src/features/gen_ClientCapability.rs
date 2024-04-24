#![allow(unused_imports)]
#![allow(clippy::all)]
use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ClientCapability` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `ClientCapability`*"]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClientCapability {
    ConditionalCreate = "conditionalCreate",
    ConditionalMediation = "conditionalMediation",
    HybridTransport = "hybridTransport",
    PasskeyPlatformAuthenticator = "passkeyPlatformAuthenticator",
    UserVerifyingPlatformAuthenticator = "userVerifyingPlatformAuthenticator",
}
