use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type A;

    // directly import function with reserved keywords
    #[wasm_bindgen]
    fn function();
    // directly import function with reserved keywords
    #[wasm_bindgen(js_name = "var")]
    fn keyword_var();

    // this is fine, because it's a method
    #[wasm_bindgen(method, js_name = "let")]
    fn keyword_let(arg: &A);
    // this is fine, because it's static
    #[wasm_bindgen(static_method_of = A, js_name = "let")]
    fn static_keyword_let();

    // directly import static with reserved keywords
    #[wasm_bindgen(thread_local_v2, js_name = "const")]
    static CONST: u32;

    // directly import type with reserved keywords
    #[wasm_bindgen(js_name = "throw")]
    type B;
    // fine with a namespace
    #[wasm_bindgen(js_name = "throw", js_namespace = ["foo"])]
    type C;
}

// Namespaces

// namespace on extern block
#[wasm_bindgen(js_namespace = ["public", "foo"])]
extern "C" {}

#[wasm_bindgen]
extern "C" {
    // invalid, because of its namespace
    #[wasm_bindgen(js_namespace = ["const", "bar"])]
    fn function();
    // okay, because it defines its own namespace
    #[wasm_bindgen(thread_local_v2, js_name = "const", js_namespace = ["bar", "new"])]
    static CONST: u32;
}

#[wasm_bindgen]
extern "C" {
    // empty namespace to be funny
    #[wasm_bindgen(js_namespace = [])]
    fn function();
}

// Classes and enums

#[wasm_bindgen]
pub struct class;
#[wasm_bindgen]
pub struct r#true; // forbid value-like keywords
#[wasm_bindgen]
pub enum switch {
    A,
    B,
}

fn main() {}
