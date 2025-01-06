#![deny(unused_variables)]

use wasm_bindgen::prelude::*;

struct A {}

#[wasm_bindgen]
impl A {
    #[wasm_bindgen(method)]
    pub fn foo() {}
}

#[wasm_bindgen]
pub struct MyStruct {
    hello: String,
}

#[wasm_bindgen(getter, typescript_custom_section)]
pub const FOO: &'static str = "FOO";

#[wasm_bindgen(readonly)]
pub fn bar() {}

#[wasm_bindgen(getter_with_clone, final)]
impl MyStruct {
    #[wasm_bindgen(getter, typescript_type = "Thing[]")]
    pub fn hello(&self) -> String {
        self.hello.clone()
    }
}

#[wasm_bindgen(return_type = "something", return_description = "something")]
struct B {}

#[wasm_bindgen(return_type = "something", return_description = "something")]
impl B {
    #[wasm_bindgen]
    pub fn foo() {}
}

#[wasm_bindgen(return_type = "something", return_description = "something")]
pub enum D {
    Variat
}

#[wasm_bindgen(return_type = "something", return_description = "something")]
impl D {
    #[wasm_bindgen]
    pub fn foo() {}
}

fn main() {}
