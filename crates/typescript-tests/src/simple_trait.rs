use wasm_bindgen::prelude::*;

use crate::simple_struct::A;

#[wasm_bindgen]
pub trait B {
    fn other();
    fn greet(&self, c: &str);
    fn take_and_return(&self, c: bool) -> bool;
}

#[wasm_bindgen]
impl B for A {
    fn other() {}

    fn greet(&self, _: &str) {}

    fn take_and_return(&self, _: bool) -> bool {
        true
    }
}
