// interface-types

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn take_and_return(a: u8) -> u16 {
    a.into()
}
