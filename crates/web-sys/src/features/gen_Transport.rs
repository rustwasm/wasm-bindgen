use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `Transport` enum.
///
///*This API requires the following crate features to be activated: `Transport`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Transport {
    Bt = "bt",
    Ble = "ble",
    Nfc = "nfc",
    Usb = "usb",
}
