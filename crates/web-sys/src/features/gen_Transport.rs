use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `Transport` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `Transport`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Transport {
    Bt = "bt",
    Ble = "ble",
    Nfc = "nfc",
    Usb = "usb",
}
