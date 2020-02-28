use wasm_bindgen::prelude::*;
#[wasm_bindgen]
#[doc = "The `BasicCardType` enum.\n\n*This API requires the following crate features to be activated: `BasicCardType`*"]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum BasicCardType {
    Credit = "credit",
    Debit = "debit",
    Prepaid = "prepaid",
}
