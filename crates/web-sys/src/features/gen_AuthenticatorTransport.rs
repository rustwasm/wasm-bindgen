use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `AuthenticatorTransport` enum.
///
///*This API requires the following crate features to be activated: `AuthenticatorTransport`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum AuthenticatorTransport {
    Usb = "usb",
    Nfc = "nfc",
    Ble = "ble",
}
