extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct A;

#[wasm_bindgen]
default impl A {
}

#[wasm_bindgen]
unsafe impl A {
}

#[wasm_bindgen]
impl Clone for A {
}

#[wasm_bindgen]
impl<T> A {
}

#[wasm_bindgen]
impl &'static A {
}

macro_rules! x { () => () }

#[wasm_bindgen]
impl A {
    const X: u32 = 3;
    type Y = u32;
    x!();

    // pub default fn foo() {} // TODO: compiler's pretty printer totally broken


    pub const fn foo() {}
    pub unsafe fn foo() {}
}
