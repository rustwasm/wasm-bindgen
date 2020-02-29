use wasm_bindgen::prelude::*;

#[wasm_bindgen]
///The `BasicCardType` enum.
///
///*This API requires the following crate features to be activated: `BasicCardType`*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum BasicCardType {
    Credit = "credit",
    Debit = "debit",
    Prepaid = "prepaid",
}
