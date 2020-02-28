use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `ConnectionType` enum.\n\n*This API requires the following crate features to be activated: `ConnectionType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ConnectionType {
    Cellular = "cellular",
    Bluetooth = "bluetooth",
    Ethernet = "ethernet",
    Wifi = "wifi",
    Other = "other",
    None = "none",
    Unknown = "unknown",
}
