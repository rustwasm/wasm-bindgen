#![crate_type = "rlib"]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn foo() -> &u32 {}

#[wasm_bindgen]
extern "C" {
    fn foo(Foo(x): Foo);

    fn foo() -> &u32;
}
