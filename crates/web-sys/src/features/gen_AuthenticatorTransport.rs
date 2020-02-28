use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `AuthenticatorTransport` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `AuthenticatorTransport`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AuthenticatorTransport {
    Usb = "usb",
    Nfc = "nfc",
    Ble = "ble",
}
