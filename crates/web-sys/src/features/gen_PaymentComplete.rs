use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `PaymentComplete` enum."]
#[doc = ""]
#[doc = "*This API requires the following crate features to be activated: `PaymentComplete`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PaymentComplete {
    Success = "success",
    Fail = "fail",
    Unknown = "unknown",
}
