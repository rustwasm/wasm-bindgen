use wasm_bindgen::prelude::*;

use crate::simple_trait::B;

#[wasm_bindgen]
pub trait C {
    fn take_bool(_: bool);
}
