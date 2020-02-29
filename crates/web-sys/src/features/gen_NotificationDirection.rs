use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `NotificationDirection` enum.
///
///*This API requires the following crate features to be activated: `NotificationDirection`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum NotificationDirection {
    Auto = "auto",
    Ltr = "ltr",
    Rtl = "rtl",
}
