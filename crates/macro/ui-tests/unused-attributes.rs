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

#[wasm_bindgen(unchecked_return_type = "something", return_description = "something", unchecked_param_type = "something", param_description = "somthing")]
struct B {}

#[wasm_bindgen(unchecked_return_type = "something", return_description = "something", unchecked_param_type = "something", param_description = "somthing")]
impl B {
    #[wasm_bindgen]
    pub fn foo() {}
}

#[wasm_bindgen(unchecked_return_type = "something", return_description = "something", unchecked_param_type = "something", param_description = "somthing")]
pub enum D {
    Variat
}

#[wasm_bindgen(unchecked_return_type = "something", return_description = "something", unchecked_param_type = "something", param_description = "somthing")]
impl D {
    #[wasm_bindgen]
    pub fn foo() {}
}

fn main() {}
